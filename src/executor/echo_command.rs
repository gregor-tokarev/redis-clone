use crate::{command_context::CommandContext, command_router::EchoCommand};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn echo_command(socket: &mut TcpStream, _context: &CommandContext, command: EchoCommand) {
    socket
        .write_all(format!("+{}\r\n", command.echo).as_bytes())
        .await
        .unwrap();
}
