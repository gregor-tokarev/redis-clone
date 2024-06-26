use crate::{
    command_context::CommandContext,
    command_router::{Command, XRangeCommand},
    resp_utils::build_array,
    storage::item::StreamDataEntry,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn xrange_command(
    socket: &mut TcpStream,
    context: &CommandContext,
    command: XRangeCommand,
) {
    let storage = context.storage.lock().await;

    let items = storage.xrange(command).await;

    match items {
        Some(itms) => {
            let streams = itms
                .into_iter()
                .map(|same_timestamp| {
                    build_array(
                        same_timestamp
                            .iter()
                            .map(|stream| stream.build_bulk())
                            .inspect(|arr| println!("{arr:?}"))
                            .collect(),
                    )
                })
                .collect::<Vec<String>>();
                println!("{:?}", streams);

            socket
                .write_all(build_array(streams).as_bytes())
                .await
                .unwrap();
        }
        None => {
            socket
                .write_all(build_array(vec![]).as_bytes())
                .await
                .unwrap();
        }
    };
}
