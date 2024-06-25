use tokio::sync::Mutex;

use crate::{args::Args, replication::Replication, storage::Storage};

pub(crate) struct CommandContext {
    pub replication_info: Mutex<Replication>,
    pub storage: Mutex<Storage>,
    pub args: Args,
}

impl CommandContext {
    pub fn new(replication: Replication, storage: Storage, args: Args) -> Self {
        Self {
            replication_info: Mutex::new(replication),
            storage: Mutex::new(storage),
            args,
        }
    }
}
