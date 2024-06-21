use crate::{command_router::Command, storage::Storage};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn ping_command(socket: &mut TcpStream, _storage: &mut Storage, _command: Command) {
    socket.write_all(b"+PONG\r\n").await.unwrap();
}
