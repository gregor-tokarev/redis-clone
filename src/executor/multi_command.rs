use crate::{command_context::CommandContext, command_router::{Command}};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn multi_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let mut transaction = context.transaction.lock().await;
    transaction.active = true;

    socket.write_all(b"+OK\r\n").await.unwrap()
}
