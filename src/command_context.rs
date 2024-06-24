use tokio::sync::Mutex;

use crate::{args::Args, rdb::RDB, replication::Replication, storage::Storage, transaction::MultiexecContainer};

pub(crate) struct CommandContext {
    pub replication_info: Mutex<Replication>,
    pub storage: Mutex<Storage>,
    pub multi_exec: Mutex<MultiexecContainer>,
    pub args: Args,
}


impl CommandContext {
    pub fn new(replication: Replication, storage: Storage, transaction_container: MultiexecContainer, args: Args) -> Self {
        Self {
            replication_info: Mutex::new(replication),
            storage: Mutex::new(storage),
            multi_exec: Mutex::new(transaction_container),
            args
        }
    }
}
