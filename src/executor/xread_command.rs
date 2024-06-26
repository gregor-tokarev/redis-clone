use crate::{
    command_context::CommandContext,
    command_router::{XRangeStatement, XReadCommand},
    resp_utils::{build_array, build_bulk},
    storage::item::split_id,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn xread_command(
    socket: &mut TcpStream,
    context: &CommandContext,
    command: XReadCommand,
) {
    if let Some(blocking) = command.blocking {
        tokio::time::sleep(blocking).await;
    };

    let storage = context.storage.lock().await;

    let mut resp = vec![];
    for (key, id) in command.keys {
        let items = storage
            .get_range(
                key.clone(),
                XRangeStatement::Id(split_id(id)),
                XRangeStatement::Positive,
                true,
            )
            .await;

        if let Some(ref itms) = items {
            println!("{itms:?}");
            if itms.is_empty() {
                continue;
            }

            let list = itms
                .iter()
                .map(|stream| stream.build_bulk())
                .collect::<Vec<String>>();

            resp.push(build_array(vec![
                build_bulk(key.to_owned()),
                build_array(list),
            ]));
        }
    }

    if resp.is_empty() {
        socket.write_all(b"$-1\r\n").await.unwrap()
    } else {
        socket
            .write_all(build_array(resp).as_bytes())
            .await
            .unwrap();
    }
}
