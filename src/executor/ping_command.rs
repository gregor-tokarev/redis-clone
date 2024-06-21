use crate::{command_context::CommandContext, command_router::Command};
use tokio::{io::AsyncWriteExt, net::TcpStream};


pub async fn ping_command(socket: &mut TcpStream, _context: &CommandContext, _command: Command) {
    socket.write_all(b"+PONG\r\n").await.unwrap();
}
