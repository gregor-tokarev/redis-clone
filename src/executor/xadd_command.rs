use std::{
    isize,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    command_context::CommandContext, command_router::XAddCommand, resp_utils::build_bulk,
    storage::item::split_id,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn xadd_command(socket: &mut TcpStream, context: &CommandContext, command: XAddCommand) {
    let mut storage = context.storage.lock().await;

    let last_entry = storage.get_top_stream_item(command.key.clone()).await;

    let (new_timestamp_statment, new_count_statement) = split_id(command.id.clone());

    let new_timestamp = match new_timestamp_statment {
        Some(t) => t,
        None => SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as isize,
    };

    let new_count = match new_count_statement {
        Some(t) => t,
        None => match last_entry.clone() {
            Some(entry) => {
                let (timestamp, count) = entry.split_id().unwrap();

                if timestamp >= new_timestamp {
                    count + 1
                } else if new_timestamp != 0 {
                    0
                } else {
                    1
                }
            }
            None => {
                if new_timestamp != 0 {
                    0
                } else {
                    1
                }
            }
        },
    };

    if new_timestamp == 0 && new_count == 0 {
        socket
            .write_all(b"-ERR The ID specified in XADD must be greater than 0-0\r\n")
            .await
            .unwrap();
        return;
    }

    let id = format!("{}-{}", new_timestamp, new_count);

    match last_entry {
        Some(entry) => {
            let (last_timestamp, last_count) = entry.clone().split_id().unwrap();

            if last_timestamp > new_timestamp
                || (last_timestamp == new_timestamp && last_count >= new_count)
            {
                socket.write_all(b"-ERR The ID specified in XADD is equal or smaller than the target stream top item\r\n").await.unwrap();
                return;
            }

            storage.xadd(command.key, id.clone(), command.data).await;

            socket.write_all(build_bulk(id).as_bytes()).await.unwrap();
        }
        None => {
            storage.xadd(command.key, id.clone(), command.data).await;

            socket.write_all(build_bulk(id).as_bytes()).await.unwrap();
        }
    };

    let blocking = context.in_block.lock().await;
    println!("{}", blocking);
    if *blocking {
        context.blocking_tx.lock().await.send(()).await.unwrap();
    }
}
