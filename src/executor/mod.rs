use crate::command_context::CommandContext;

use crate::Command;
use echo_command::echo_command;
use get_command::get_command;
use info_command::info_command;
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
mod replconf_command;

pub async fn execute_command(command: Command, socket: &mut TcpStream, context: &CommandContext) {
    match command {
        Command::Ping => ping_command(socket, context, command).await,
        Command::Echo(cmd) => echo_command(socket, context, cmd).await,
        Command::Set(cmd) => set_command(socket, context, cmd).await,
        Command::Get(cmd) => get_command(socket, context, cmd).await,
        Command::Info => info_command(socket, context, command).await,
        Command::Replconf => replconf_command(socket, context, command).await,
        Command::Unrecognized => {
            println!("Unrecognized command");
            socket.write_all(b"+OK\r\n").await.unwrap();
        },
    }
}
