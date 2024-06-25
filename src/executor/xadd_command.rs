use crate::{
    command_context::CommandContext, command_router::XAddCommand, resp_utils::build_bulk,
    storage::item::split_id,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn xadd_command(socket: &mut TcpStream, context: &CommandContext, command: XAddCommand) {
    let (new_timestamp, new_count) = split_id(command.id.clone()).unwrap();
    if new_timestamp == 0 && new_count == 0 {
        socket
            .write_all(b"-ERR The ID specified in XADD must be greater than 0-0\r\n")
            .await
            .unwrap();
        return;
    }

    let mut storage = context.storage.lock().await;

    let last_entry = storage.get_top_stream_item(command.key.clone()).await;

    match last_entry {
        Some(entry) => {
            let (last_timestamp, last_count) = split_id(entry.id).unwrap();

            if last_timestamp > new_timestamp || (last_timestamp == new_timestamp && last_count >= new_count) {
                // println!("gach ya");
                socket.write_all(b"-ERR The ID specified in XADD is equal or smaller than the target stream top item\r\n").await.unwrap();
                return;
            }

            storage
                .xadd(command.key, command.id.clone(), command.data)
                .await;

            socket
                .write_all(build_bulk(command.id).as_bytes())
                .await
                .unwrap();
        }
        None => {
            storage
                .xadd(command.key, command.id.clone(), command.data)
                .await;

            socket
                .write_all(build_bulk(command.id).as_bytes())
                .await
                .unwrap();
        }
    }
}
