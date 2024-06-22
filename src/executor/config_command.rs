use crate::{
    command_context::CommandContext,
    command_router::{config::ConfigCommand, config::ConfigCommandAction},
    resp_utils::{build_array, build_bulk},
};
use tokio::{io::AsyncWriteExt, net::TcpStream};

pub async fn config_command(
    socket: &mut TcpStream,
    context: &CommandContext,
    command: ConfigCommand,
) {
    match command.action {
        ConfigCommandAction::Get(key) => match key.as_str() {
            "dir" => {
                socket
                    .write_all(
                        build_array(vec![
                            build_bulk("dir".to_owned()),
                            build_bulk(context.args.dir.clone()),
                        ])
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
            }
            "dbfilename" => {
                socket
                    .write_all(
                        build_array(vec![
                            build_bulk("dbfilename".to_owned()),
                            build_bulk(context.args.dbfilename.clone()),
                        ])
                        .as_bytes(),
                    )
                    .await
                    .unwrap();
            }
            _ => {}
        },
        ConfigCommandAction::Unrecognized => {}
    };
}
