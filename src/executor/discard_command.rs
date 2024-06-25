use crate::{
    command_context::CommandContext, command_router::Command, transaction::TransactionContainer,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn discard_command(
    socket: &mut TcpStream,
    _context: &CommandContext,
    _command: Command,
    transaction: &mut TransactionContainer,
) {
    if !transaction.active {
        socket
            .write_all(b"-ERR DISCARD without MULTI\r\n")
            .await
            .unwrap();
        return;
    }

    transaction.clear();

    socket.write_all(b"+OK\r\n").await.unwrap();
}
