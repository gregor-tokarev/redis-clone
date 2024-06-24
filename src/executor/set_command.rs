use std::isize;

use crate::{
    command_context::CommandContext, command_router::SetCommand, storage::{Item}
};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::Duration};

pub async fn set_command(socket: &mut TcpStream, context: &CommandContext, command: SetCommand) {
    let mut storage = context.storage.lock().await;

    let duration = command
        .expire_after
        .map(|expire| Duration::from_millis(expire.parse().unwrap()));

    let value = match command.value.parse::<isize>() {
        Ok(num) => Item::Numeric(num),
        Err(_) => Item::SimpleString(command.value)
    };

    storage.set(
        command.key.to_owned(),
        value,
        duration,
    ).await;

    socket.write_all(b"+OK\r\n").await.unwrap();
}
