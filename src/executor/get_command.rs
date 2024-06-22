use crate::{command_context::CommandContext, command_router::GetCommand};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn get_command(socket: &mut TcpStream, context: &CommandContext, command: GetCommand) {
    let storage = context.storage.lock().await;

    let value = storage.get(&command.key).await;

    match value {
        Some(v) => {
            let resp = v.build_response_string();

            socket.write_all(resp.as_bytes()).await.unwrap();
        }
        None => {
            socket.write_all(b"$-1\r\n").await.unwrap();
        }
    };
}
