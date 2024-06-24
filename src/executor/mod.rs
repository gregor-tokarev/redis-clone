use crate::command_context::CommandContext;

use crate::Command;
use config_command::config_command;
use echo_command::echo_command;
use get_command::get_command;
use incr_command::incr_command;
use info_command::info_command;
use keys_command::keys_command;
use ping_command::ping_command;
use replconf_command::replconf_command;
use set_command::set_command;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;

mod echo_command;
mod get_command;
mod info_command;
mod ping_command;
mod set_command;
mod incr_command;
mod replconf_command;
mod config_command;
mod keys_command;

pub async fn execute_command(command: Command, socket: &mut TcpStream, context: &CommandContext) {
    match command {
        Command::Ping => ping_command(socket, context, command).await,
        Command::Echo(cmd) => echo_command(socket, context, cmd).await,
        Command::Set(cmd) => set_command(socket, context, cmd).await,
        Command::Get(cmd) => get_command(socket, context, cmd).await,
        Command::Info => info_command(socket, context, command).await,
        Command::Replconf => replconf_command(socket, context, command).await,
        Command::Config(cmd) => config_command(socket, context, cmd).await,
        Command::Keys(cmd) => keys_command(socket, context, cmd).await,
        Command::Incr(cmd) => incr_command(socket, context, cmd).await,
        Command::Unrecognized => {
            println!("Unrecognized command");
            socket.write_all(b"+OK\r\n").await.unwrap();
        },
    }
}
