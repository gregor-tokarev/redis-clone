use crate::{command_context::CommandContext, command_router::Command, transaction::{self, TransactionContainer}};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn multi_command(socket: &mut TcpStream, context: &CommandContext, _command: Command, transaction: &mut TransactionContainer) {
    transaction.active = true;

    socket.write_all(b"+OK\r\n").await.unwrap()
}
