use crate::{
    command_context::CommandContext,
    command_router::{Command, GetCommand},
    resp_utils::build_bulk,
    storage::Item,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn get_command(socket: &mut TcpStream, context: &CommandContext, command: GetCommand) {
    let mut transaction = context.multi_exec.lock().await;
    if transaction.active {
        if transaction.has_key_in_transaction(command.clone().key).await {
            transaction.store_action(Command::Get(command.clone()));
            socket
                .write_all(build_bulk("QUEUED".to_owned()).as_bytes())
                .await
                .unwrap();
        } else {
            socket.write_all(b"$-1\r\n").await.unwrap();
        };

        return;
    }

    socket
        .write_all(get_command_action(context, command).await.as_bytes())
        .await
        .unwrap()
}

pub(crate) async fn get_command_action(context: &CommandContext, command: GetCommand) -> String {
    let storage = context.storage.lock().await;

    let value = storage.get(&command.key).await;

    match value {
        Some(v) => {
            let resp = match v {
                Item::Numeric(n) => n.to_string(),
                Item::SimpleString(s) => s,
            };

            build_bulk(resp)
        }
        None => String::from("$-1\r\n"),
    }
}
