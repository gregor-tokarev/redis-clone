use crate::command_context::CommandContext;
use crate::command_router::Command;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

pub async fn replconf_command(
    socket: &mut TcpStream,
    _context: &CommandContext,
    _command: Command,
) {
    socket.write_all(b"+OK\r\n").await.unwrap();
}
