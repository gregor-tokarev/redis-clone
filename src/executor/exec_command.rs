use crate::{
    command_context::CommandContext, command_router::Command, resp_utils::build_array,
    transaction::TransactionContainer,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn exec_command(
    socket: &mut TcpStream,
    context: &CommandContext,
    _command: Command,
    transaction: &mut TransactionContainer,
) {
    if !transaction.active {
        socket
            .write_all(b"-ERR EXEC without MULTI\r\n")
            .await
            .unwrap();
        return;
    }

    let resp = transaction.exec_commands(context).await;

    socket
        .write_all(build_array(resp).as_bytes())
        .await
        .unwrap();
    transaction.active = false;
}
