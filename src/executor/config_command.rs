use crate::{
    command_context::CommandContext,
    command_router::{ConfigCommand, ConfigCommandAction, GetCommand},
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
                if let Some(dir) = &context.args.dir {
                    socket
                        .write_all(
                            build_array(vec![
                                build_bulk("dir".to_owned()),
                                build_bulk(dir.to_owned()),
                            ])
                            .as_bytes(),
                        )
                        .await
                        .unwrap();
                }
            },
            "dbfilename" => {
                if let Some(filename) = &context.args.dbfilename {
                    socket
                        .write_all(
                            build_array(vec![
                                build_bulk("dbfilename".to_owned()),
                                build_bulk(filename.to_owned()),
                            ])
                            .as_bytes(),
                        )
                        .await
                        .unwrap();
                }
            },
            _ => {}
        },
    };
}
