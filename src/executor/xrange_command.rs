use crate::{
    command_context::CommandContext, command_router::XRangeCommand, resp_utils::build_array,
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn xrange_command(
    socket: &mut TcpStream,
    context: &CommandContext,
    command: XRangeCommand,
) {
    let storage = context.storage.lock().await;

    let items = storage
        .get_range(
            command.key,
            command.start_statement,
            command.end_statement,
            false,
        )
        .await;

    match items {
        Some(itms) => {
            let streams = itms
                .into_iter()
                .map(|stream| stream.build_bulk())
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
