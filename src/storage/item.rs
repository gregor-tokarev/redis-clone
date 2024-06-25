use crate::resp_utils::build_bulk;

#[derive(Debug, Clone)]
pub enum Item {
    SimpleString(String),
    Numeric(isize),
    // Arr(Vec<Item>),
    // None,
}

impl Item {
    pub fn build_response_string(&self) -> String {
        match self {
            Self::SimpleString(s) => build_bulk(s.to_owned()),
            Self::Numeric(n) => format!(":{}\r\n", n),
        }
    }

    pub fn get_type_string(&self) -> String {
        match self {
            Self::SimpleString(_) => "string".to_owned(),
            Self::Numeric(_) => "number".to_owned(),
        }
    }
}
