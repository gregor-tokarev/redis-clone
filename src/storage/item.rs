use std::collections::HashMap;

use crate::resp_utils::build_bulk;

#[derive(Debug, Clone)]
pub struct StreamDataEntry {
   pub id: String,
   pub data: HashMap<String, String>
}

#[derive(Debug, Clone)]
pub struct StreamData {
    pub value: Vec<StreamDataEntry>
}

#[derive(Debug, Clone)]
pub enum Item {
    SimpleString(String),
    Numeric(isize),
    Stream(StreamData),
    // Arr(Vec<Item>),
    // None,
}

impl Item {
    pub fn build_response_string(&self) -> String {
        match self {
            Self::SimpleString(s) => build_bulk(s.to_owned()),
            Self::Numeric(n) => format!(":{}\r\n", n),
            _ =>  "stream".to_owned()
        }
    }

    pub fn get_type_string(&self) -> String {
        match self {
            Self::SimpleString(_) => "string".to_owned(),
            Self::Numeric(_) => "number".to_owned(),
            Self::Stream(_) => "stream".to_owned()
        }
    }
}
