use crate::{command_context::CommandContext, command_router::{Command}};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn multi_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    println!("Start multi command");
    let mut transaction = context.multi_exec.lock().await;
    transaction.active = true;
    println!("{}", transaction.active);

    socket.write_all(b"+OK\r\n").await.unwrap()
}
