use crate::{command_context::CommandContext, command_router::{KeysCommand, TypeCommand}, resp_utils::{build_array, build_bulk}};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn type_command(socket: &mut TcpStream, context: &CommandContext, command: TypeCommand) {
    let storage = context.storage.lock().await;
    let key = storage.get(&command.key).await;
    
    let resp = match key {
       Some(itm) => itm.get_type_string(),
       None => "none".to_owned()
    };

    socket
        .write_all(build_bulk(resp).as_bytes())
        .await
        .unwrap();
}
