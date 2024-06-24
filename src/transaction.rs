use futures::future::{BoxFuture, Future};
use std::boxed;
use std::pin::Pin;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use tokio::sync::mpsc::{channel, Receiver, Sender};

use crate::command_context::CommandContext;
use crate::command_router::Command;
use crate::executor::get_command::get_command_action;
use crate::executor::incr_command::incr_command_action;
use crate::executor::set_command::set_command_action;
use crate::resp_utils::build_bulk;

pub(crate) struct MultiexecContainer {
    pub active: bool,
    execution_queue: Vec<Command>,
}

impl MultiexecContainer {
    pub(crate) fn new() -> Self {
        Self {
            execution_queue: vec![],
            active: false,
        }
    }

    pub(crate) fn store_action(&mut self, command: Command) {
        self.execution_queue.push(command)
    }

    // pub(crate) async fn execute
    pub(super) async fn exec_commands(&self, context: &CommandContext) -> Vec<String> {
        // self.execution_queue.iter().map(|c| match c {
        //     Command::Set(cmd) => {set_command_action(context, cmd.clone());}
        //     _ => {}
        // });
        //

        let mut resp = vec![];

        for cmd in &self.execution_queue {
            match cmd {
                Command::Set(cmd) => {
                    set_command_action(context, cmd.clone()).await;
                    resp.push(build_bulk("OK".to_owned()));
                }
                Command::Incr(cmd) => {
                    match incr_command_action(context, cmd.clone()).await {
                        Some(val) => resp.push(val.build_response_string()),
                        None => {}
                    };
                },
                Command::Get(cmd) => {
                    resp.push(get_command_action(context, cmd.clone()).await)
                }
                _ => {}
            }
        };

        resp
    }

    pub(super) async fn has_key_in_transaction(&self, key: String) -> bool {
        let mut has_key = false;

        println!("{:?}", self.execution_queue);
        for cmd in &self.execution_queue {
            match cmd {
              Command::Set(c) if c.key == key => has_key = true,
              Command::Incr(c) if c.key == key => has_key = true,
              _ => {}
            };
        }

        has_key
    }
}
