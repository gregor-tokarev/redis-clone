use crate::{
    command_context::CommandContext,
    command_router::{Command},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn discard_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let mut transaction = context.transaction.lock().await;
    if !transaction.active {
        socket.write_all(b"-ERR EXEC without MULTI\r\n").await.unwrap();
        return;
    }

    transaction.discard();
    

    socket.write_all(b"+OK\r\n").await.unwrap();
}
