use crate::{command_context::CommandContext, command_router::Command, resp_utils::build_bulk};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn info_command(socket: &mut TcpStream, context: &CommandContext, _command: Command) {
    let replication = context.replication_info.lock().await;

    let role = format!("role:{}\r\n", if replication.is_master {"master"} else {"slave"});

    let mut response = role.to_string();

    if let Some(master_id) = &replication.master_id {
        let id_str = format!("master_replid:{}\r\n", master_id);
        let repl_offset = "master_repl_offset:0\r\n".to_owned();

        response = format!("{}{}{}", response, id_str, repl_offset);
    }

    socket
        .write_all(build_bulk(response).as_bytes())
        .await
        .unwrap();
}
