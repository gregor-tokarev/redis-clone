use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use crate::command_context::CommandContext;
use crate::command_router::Command;

pub async fn replconf_command(socket: &mut TcpStream, _context: &CommandContext, _command: Command) {
    socket.write_all(b"+OK\r\n").await.unwrap();
}
