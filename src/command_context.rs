use tokio::sync::Mutex;

use crate::{args::Args, replication::Replication, storage::Storage, transaction::TransactionContainer};

pub(crate) struct CommandContext {
    pub replication_info: Mutex<Replication>,
    pub storage: Mutex<Storage>,
    pub multi_exec: Mutex<TransactionContainer>,
    pub args: Args,
}


impl CommandContext {
    pub fn new(replication: Replication, storage: Storage, transaction_container: TransactionContainer, args: Args) -> Self {
        Self {
            replication_info: Mutex::new(replication),
            storage: Mutex::new(storage),
            multi_exec: Mutex::new(transaction_container),
            args
        }
    }
}
