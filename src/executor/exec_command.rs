use crate::{
    command_context::CommandContext,
    command_router::{Command, EchoCommand, KeysCommand},
    resp_utils::{build_array, build_bulk},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn exec_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let mut transaction = context.multi_exec.lock().await;
    if !transaction.active {
        socket.write_all(b"-ERR EXEC without MULTI\r\n").await.unwrap();
        return;
    }

    let resp = transaction.exec_commands(context).await;

    socket.write_all(build_array(resp).as_bytes()).await.unwrap();
    transaction.active = false;
}
