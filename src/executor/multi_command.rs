use crate::{command_context::CommandContext, command_router::{Command, EchoCommand, KeysCommand}, resp_utils::{build_array, build_bulk}, multi_exec};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn multi_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let mut transaction = context.multi_exec.lock().await;
    transaction.active = true;

    socket.write_all(b"+OK\r\n").await.unwrap()
}
