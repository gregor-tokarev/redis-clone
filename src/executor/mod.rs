use crate::storage::Storage;
use crate::Command;
use echo_command::echo_command;
use get_command::get_command;
use info_command::info_command;
use ping_command::ping_command;
use set_command::set_command;
use tokio::net::TcpStream;

mod get_command;
mod ping_command;
mod set_command;
mod echo_command;
mod info_command;

pub async fn execute_command(command: Command, socket: &mut TcpStream, storage: &mut Storage) {
    match command {
        Command::Ping => ping_command(socket, storage, command).await,
        Command::Echo(cmd) => echo_command(socket, storage, cmd).await,
        Command::Set(cmd) => set_command(socket, storage, cmd).await,
        Command::Get(cmd) => get_command(socket, storage, cmd).await,
        Command::Info => info_command(socket, storage, command).await,
        Command::Unrecognized => println!("Unrecognized command"),
    };
}
