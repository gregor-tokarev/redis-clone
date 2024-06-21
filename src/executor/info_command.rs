use crate::{command_context::CommandContext, command_router::Command, resp_utils::build_bulk};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn info_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let replication = context.replication_info.lock().await;

    let response = format!("role:{}", if replication.is_master {"master"} else {"slave"});

    socket
        .write_all(build_bulk(response).as_bytes())
        .await
        .unwrap();
}
