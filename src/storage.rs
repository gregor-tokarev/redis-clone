use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration},
};

use tokio::time::Instant;

#[derive(Debug, Clone)]
pub enum Item {
    SimpleString(String),
    BulkString(String),
    Arr(Vec<Item>),
    None,
}

pub struct Storage {
    state: Arc<Mutex<HashMap<String, Item>>>,
    expire_list: Arc<Mutex<HashMap<String, Instant>>>,
}
impl Storage {
    pub fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(HashMap::new())),
            expire_list: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn set(&mut self, key: String, value: Item, expire_after: Option<Duration>) {
        self.state.lock().unwrap().insert(key.clone(), value);

        if let Some(expr_after) = expire_after {
            let now = Instant::now();
            let expire_at = now.checked_add(expr_after);

            if let Some(time) = expire_at {
                self.expire_list.lock().unwrap().insert(key, time);
            }
        }
    }

    pub fn remove(&self, key: &str) {
        self.state.lock().unwrap().remove(key);
    }

    pub fn get(&self, key: &str) -> Option<Item> {
        let mut state = self.state.lock().unwrap();
        let mut expire_list = self.expire_list.lock().unwrap();

        if let Some(expire_time) = expire_list.get(key) {
            println!("{:?} - {:?}", Instant::now(), expire_time);
            if Instant::now() >= *expire_time {
                state.remove(key);
                expire_list.remove(key);
                return None;
            }
        }

        state.get(key).cloned()
    }

    pub fn tick(&mut self) {
        let current_time = Instant::now();
        let mut keys = vec![];

        {
            let expire_list = self.expire_list.lock().unwrap();
            for (key, value) in expire_list.iter() {
                if current_time <= *value {
                    keys.push(key.clone());
                }
            }
        }

        if !keys.is_empty() {
            let mut expire_list = self.expire_list.lock().unwrap();
            for key in &keys {
                expire_list.remove(key);
            }
        };

        println!("Expired keys: {}", keys.len());

        for key in keys {
            self.remove(&key);
        }
    }
}

impl Item {
    pub fn build_response_string(&self) -> String {
        match self {
            Self::SimpleString(s) => format!("${}\r\n{}\r\n", s.len(), s),
            _ => String::new()
        }
    }
}
