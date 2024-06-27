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
        if blocking.as_millis() != 0 {
            tokio::time::sleep(blocking).await;
        } else {
            {
                let mut blocking = context.in_block.lock().await;
                *blocking = true;
            }

            let mut rx = context.blocing_rx.lock().await;
            rx.recv().await;
        }
    };

    let storage = context.storage.lock().await;

    let mut resp = vec![];
    for (key, id) in command.keys {
        let start_statement = if id.clone() == "$" {
            let last_entry = storage.get_top_stream_item(key.clone()).await;

            match last_entry {
                Some(entry) => XRangeStatement::Id(split_id(entry.id)),
                None => {
                    socket.write_all(b"$-1\r\n").await.unwrap();
                    return
                }
            }
            
        } else {
            XRangeStatement::Id(split_id(id.clone()))
        };

        let items = storage
            .get_range(
                key.clone(),
                start_statement,
                XRangeStatement::Positive,
                id != "$",
            )
            .await;

        if let Some(ref itms) = items {
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
    };

    let mut blocking = context.in_block.lock().await;
    *blocking = false;
}
