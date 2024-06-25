use crate::command_context::CommandContext;
use crate::command_router::Command;
use crate::executor::get_command::get_command_action;
use crate::executor::incr_command::incr_command_action;
use crate::executor::set_command::set_command_action;
use crate::resp_utils::build_bulk;

pub(crate) struct TransactionContainer {
    pub active: bool,
    execution_queue: Vec<Command>,
}

impl TransactionContainer {
    pub(crate) fn new() -> Self {
        Self {
            execution_queue: vec![],
            active: false,
        }
    }

    pub(crate) fn store_action(&mut self, command: Command) {
        self.execution_queue.push(command)
    }

    pub(super) async fn exec_commands(&mut self, context: &CommandContext) -> Vec<String> {
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
                        None => {
                            resp.push("-ERR value is not an integer or out of range\r\n".to_owned())
                        }
                    };
                }
                Command::Get(cmd) => resp.push(get_command_action(context, cmd.clone()).await),
                _ => {}
            }
        }

        self.clear();

        resp
    }

    pub(super) fn clear(&mut self) {
        self.active = false;
        self.execution_queue.clear();
    }

    // pub(super) async fn has_key_in_transaction(&self, key: String) -> bool {
    //     let mut has_key = false;
    //
    //     println!("{:?}", self.execution_queue);
    //     for cmd in &self.execution_queue {
    //         match cmd {
    //           Command::Set(c) if c.key == key => has_key = true,
    //           Command::Incr(c) if c.key == key => has_key = true,
    //           _ => {}
    //         };
    //     }
    //
    //     has_key
    // }
}
