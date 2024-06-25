use crate::command_context::CommandContext;

use crate::transaction::TransactionContainer;
use crate::Command;
use config_command::config_command;
use discard_command::discard_command;
use echo_command::echo_command;
use exec_command::exec_command;
use get_command::get_command;
use incr_command::incr_command;
use info_command::info_command;
use keys_command::keys_command;
use multi_command::multi_command;
use ping_command::ping_command;
use replconf_command::replconf_command;
use set_command::set_command;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use type_command::type_command;
use xadd_command::xadd_command;

mod config_command;
mod discard_command;
mod echo_command;
mod exec_command;
pub mod get_command;
pub mod incr_command;
mod info_command;
mod keys_command;
mod multi_command;
mod ping_command;
mod replconf_command;
pub mod set_command;
mod type_command;
mod xadd_command;

pub async fn execute_command(
    command: Command,
    socket: &mut TcpStream,
    context: &CommandContext,
    transaction: Arc<Mutex<TransactionContainer>>,
) {
    let mut tx = transaction.lock().await;

    match command {
        Command::Ping => ping_command(socket, context, command).await,
        Command::Echo(cmd) => echo_command(socket, context, cmd).await,
        Command::Set(cmd) => set_command(socket, context, cmd, &mut tx).await,
        Command::Get(cmd) => get_command(socket, context, cmd, &mut tx).await,
        Command::Info => info_command(socket, context, command, &mut tx).await,
        Command::Replconf => replconf_command(socket, context, command).await,
        Command::Config(cmd) => config_command(socket, context, cmd).await,
        Command::Keys(cmd) => keys_command(socket, context, cmd).await,
        Command::Incr(cmd) => incr_command(socket, context, cmd, &mut tx).await,
        Command::Multi => multi_command(socket, context, command, &mut tx).await,
        Command::Exec => exec_command(socket, context, command, &mut tx).await,
        Command::Discard => discard_command(socket, context, command, &mut tx).await,
        Command::Type(cmd) => type_command(socket, context, cmd).await,
        Command::XAdd(cmd) => xadd_command(socket, context, cmd).await,
        Command::Unrecognized => {
            println!("Unrecognized command");
            socket.write_all(b"+OK\r\n").await.unwrap();
        }
    }
}
