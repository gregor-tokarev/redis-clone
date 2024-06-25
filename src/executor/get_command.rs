use crate::{
    command_context::CommandContext,
    command_router::{Command, GetCommand},
    resp_utils::build_bulk,
    storage::item::Item, transaction::{TransactionContainer},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn get_command(socket: &mut TcpStream, context: &CommandContext, command: GetCommand, transaction: &mut TransactionContainer) {
    if transaction.active {
        transaction.store_action(Command::Get(command.clone()));

        socket
            .write_all(build_bulk("QUEUED".to_owned()).as_bytes())
            .await
            .unwrap();

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
