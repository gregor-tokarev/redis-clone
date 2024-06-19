use std::collections::HashMap;


#[derive(Debug)]
pub enum Item {
    SimpleString(String),
    BulkString(String),
    Arr(Vec<Item>),
    None
}

pub struct Storage {
    _state: HashMap<String, Item>
}

impl Storage {
    pub fn new() -> Self {
        Self {
            _state: HashMap::new()
        }
    }

    pub fn set(&mut self, key: String, value: Item) {
        self._state.insert(key, value);
    }

    pub fn get(&self, key: String) -> Option<&Item> {
        self._state.get(key.as_str())
    }
}

impl Item {
    pub fn build_response_string(&self) -> String {
        let mut response = String::new();

        match self {
            Self::SimpleString(s) => response = format!("${}\r\n{}\r\n", s.len(), s),
            _ => {}
        };

        response
    }
}
