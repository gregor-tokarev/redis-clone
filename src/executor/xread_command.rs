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
    // let items = storage.get_range(command.key, command.start_statement, XRangeStatement::Positive).await;

    socket
        .write_all(build_array(resp).as_bytes())
        .await
        .unwrap();
    // match items {
    //     Some(itms) => {
    //         let streams = itms
    //             .into_iter()
    //             .map(|stream| stream.build_bulk())
    //             .collect::<Vec<String>>();
    //         println!("{:?}", streams);
    //
    //         socket
    //             .write_all(build_array(streams).as_bytes())
    //             .await
    //             .unwrap();
    //     }
    //     None => {
    //         socket
    //             .write_all(build_array(vec![]).as_bytes())
    //             .await
    //             .unwrap();
    //     }
    // };
}
