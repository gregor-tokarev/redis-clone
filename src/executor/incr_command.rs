use std::isize;

use crate::{
    command_context::CommandContext,
    command_router::{IncrCommand, SetCommand},
    storage::Item,
};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::Duration};

pub async fn incr_command(socket: &mut TcpStream, context: &CommandContext, command: IncrCommand) {
    let mut transaction = context.multi_exec.lock().await;
    if transaction.active {
        transaction.store_action(Box::new(())).await;
        socket.write_all(b"+QUEUED\r\n").await.unwrap();
        return;
    };

    let mut storage = context.storage.lock().await;

    match storage.get(&command.key).await {
        Some(val) => match val {
            Item::Numeric(n) => {
                let value = Item::Numeric(n + command.step);
                storage.set(command.key, value.clone(), None).await;

                socket
                    .write(value.build_response_string().as_bytes())
                    .await
                    .unwrap();
            }
            _ => {
                socket
                    .write_all(b"-ERR value is not an integer or out of range\r\n")
                    .await
                    .unwrap();
            }
        },
        None => {
            let value = Item::Numeric(command.step);
            storage.set(command.key, value.clone(), None).await;

            socket
                .write(value.build_response_string().as_bytes())
                .await
                .unwrap();
        }
    }
}
