use std::isize;

use crate::{
    command_context::CommandContext, command_router::{Command, SetCommand, XAddCommand}, resp_utils::build_bulk, storage::item::Item, transaction::TransactionContainer
};
use tokio::{io::AsyncWriteExt, net::TcpStream, time::Duration};

pub async fn xadd_command(socket: &mut TcpStream, context: &CommandContext, command: XAddCommand) {
    let mut storage = context.storage.lock().await;

    storage.xadd(command.key, command.id.clone(), command.data).await;

    socket.write_all(build_bulk(command.id).as_bytes()).await.unwrap();
}


