use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use tokio::{sync::Mutex};

use crate::{resp_utils::build_bulk};

#[derive(Debug, Clone)]
pub enum Item {
    SimpleString(String),
    Numeric(isize),
    // Arr(Vec<Item>),
    // None,
}

pub type StorageState = HashMap<String, Item>;

pub type StorageExpire = HashMap<String, u128>;

pub struct Storage {
    state: Arc<Mutex<StorageState>>,
    expire_list: Arc<Mutex<StorageExpire>>,
}
impl Storage {
    pub fn new(dump: (StorageState, StorageExpire)) -> Self {
        Self {
            state: Arc::new(Mutex::new(dump.0)),
            expire_list: Arc::new(Mutex::new(dump.1)),
        }
    }

    pub async fn set(&mut self, key: String, value: Item, expire_after: Option<Duration>) {
        self.state.lock().await.insert(key.clone(), value);

        if let Some(expr_after) = expire_after {
            let now = self.now();
            let expire_at = now + expr_after;

            self.expire_list
                .lock()
                .await
                .insert(key, expire_at.as_millis());
        }
    }

    fn now(&self) -> Duration {
        SystemTime::now().duration_since(UNIX_EPOCH).unwrap()
    }

    pub async fn remove(&self, key: &str) {
        self.state.lock().await.remove(key);
    }

    pub async fn get(&self, key: &str) -> Option<Item> {
        let mut state = self.state.lock().await;
        let mut expire_list = self.expire_list.lock().await;

        if let Some(expire_time) = expire_list.get(key) {
            if self.now().as_millis() >= *expire_time {
                state.remove(key);
                expire_list.remove(key);
                return None;
            }
        };

        state.get(key).cloned()
    }

    pub async fn keys(&self, _pattern: String) -> Vec<String> {
        let state = self.state.lock().await;

        state.keys().cloned().collect()
    }

    pub async fn tick(&mut self) {
        let current_time = self.now().as_millis();
        let mut keys = vec![];

        {
            let expire_list = self.expire_list.lock().await;
            for (key, value) in expire_list.iter() {
                if current_time >= *value {
                    keys.push(key.clone());
                }
            }
        }

        if !keys.is_empty() {
            let mut expire_list = self.expire_list.lock().await;
            for key in &keys {
                expire_list.remove(key);
            }
        };

        for key in keys {
            self.remove(&key).await;
        }
    }
}

impl Item {
    pub fn build_response_string(&self) -> String {
        match self {
            Self::SimpleString(s) => build_bulk(s.to_owned()),
            Self::Numeric(n) => format!(":{}\r\n", n),
        }
    }
}
