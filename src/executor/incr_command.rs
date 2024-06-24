

use crate::{
    command_context::CommandContext,
    command_router::{Command, IncrCommand},
    storage::Item,
};

use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn incr_command(socket: &mut TcpStream, context: &CommandContext, command: IncrCommand) {
    let mut transaction = context.transaction.lock().await;
    if transaction.active {
        transaction.store_action(Command::Incr(command));

        socket.write_all(b"+QUEUED\r\n").await.unwrap();
        return;
    };

    match incr_command_action(context, command).await {
        Some(itm) => socket
            .write_all(itm.build_response_string().as_bytes())
            .await
            .unwrap(),
        None => socket
            .write_all(b"-ERR value is not an integer or out of range\r\n")
            .await
            .unwrap(),
    };
}

pub(crate) async fn incr_command_action(
    context: &CommandContext,
    command: IncrCommand,
) -> Option<Item> {
    let mut storage = context.storage.lock().await;

    match storage.get(&command.key).await {
        Some(val) => match val {
            Item::Numeric(n) => {
                let value = Item::Numeric(n + command.step);
                storage.set(command.key, value.clone(), None).await;

                Some(value)
            }
            _ => None,
        },
        None => {
            let value = Item::Numeric(command.step);
            storage.set(command.key, value.clone(), None).await;

            Some(value)
        }
    }
}
