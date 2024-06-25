use crate::{
    command_context::CommandContext, command_router::Command, transaction::TransactionContainer,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn multi_command(
    socket: &mut TcpStream,
    _context: &CommandContext,
    _command: Command,
    transaction: &mut TransactionContainer,
) {
    transaction.active = true;

    socket.write_all(b"+OK\r\n").await.unwrap()
}
