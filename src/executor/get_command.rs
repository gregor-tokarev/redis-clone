use crate::{command_router::GetCommand, storage::Storage};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn get_command(socket: &mut TcpStream, storage: &mut Storage, command: GetCommand) {
    let value = storage.get(&command.key);

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
