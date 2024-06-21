use tokio::sync::Mutex;

use crate::{replication::Replication, storage::Storage};

pub(crate) struct CommandContext {
    pub replication_info: Mutex<Replication>,
    pub storage: Mutex<Storage>
}


impl CommandContext {
    pub fn new(replication: Replication, storage: Storage) -> Self {
        Self {
            replication_info: Mutex::new(replication),
            storage: Mutex::new(storage)
        }
    }
}
