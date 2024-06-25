use crate::{
    command_context::CommandContext,
    command_router::KeysCommand,
    resp_utils::{build_array, build_bulk},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn keys_command(socket: &mut TcpStream, context: &CommandContext, command: KeysCommand) {
    let storage = context.storage.lock().await;
    let keys = storage
        .keys(command.pattern)
        .await
        .iter()
        .map(|key| build_bulk(key.to_owned()))
        .collect();

    socket
        .write_all(build_array(keys).as_bytes())
        .await
        .unwrap();
}
