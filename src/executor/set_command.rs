use crate::{
    command_router::SetCommand,
    storage::{Item, Storage},
};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::Duration};

pub async fn set_command(socket: &mut TcpStream, storage: &mut Storage, command: SetCommand) {
    let duration = command
        .expire_after
        .map(|expire| Duration::from_millis(expire.parse().unwrap()));

    storage.set(
        command.key.to_owned(),
        Item::SimpleString(command.value.to_owned()),
        duration,
    );

    socket.write_all(b"+OK\r\n").await.unwrap();
}
