use std::{collections::HashMap, isize};

use crate::resp_utils::{build_array, build_bulk};

#[derive(Debug, Clone)]
pub struct StreamDataEntry {
    pub id: String,
    pub data: HashMap<String, String>,
}

impl StreamDataEntry {
    pub fn split_id(&self) -> Result<(isize, isize), String> {
        let mut split = self.id.split('-');

        let timestamp = split
            .next()
            .ok_or_else(|| String::from("parse error"))?
            .parse::<isize>()
            .map_err(|_| String::from("parse error"))?;

        let count = split
            .next()
            .ok_or_else(|| String::from("parse error"))?
            .parse::<isize>()
            .map_err(|_| String::from("parse error"))?;

        Ok((timestamp, count))
    }
}

impl StreamDataEntry {
    pub fn build_bulk(&self) -> String {
        let mut data_part = vec![];

        for (key, value) in &self.data {
            data_part.push(build_bulk(key.to_owned()));
            data_part.push(build_bulk(value.to_owned()));
        }

        build_array(vec![build_bulk(self.id.clone()), build_array(data_part)])
    }
}

pub fn split_id(id: String) -> (Option<isize>, Option<isize>) {
    if id.as_bytes() == b"*" {
        return (None, None);
    };

    let mut split = id.split('-');

    let timestamp = match split
        .next()
        .unwrap_or("*")
        .parse::<isize>()
        .map_err(|_| None::<isize>)
    {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    let count = match split
        .next()
        .unwrap_or("*")
        .parse::<isize>()
        .map_err(|_| None::<isize>)
    {
        Ok(num) => Some(num),
        Err(_) => None,
    };

    (timestamp, count)
}

#[derive(Debug, Clone)]
pub struct StreamData {
    pub value: Vec<StreamDataEntry>,
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
            _ => "stream".to_owned(),
        }
    }

    pub fn get_type_string(&self) -> String {
        match self {
            Self::SimpleString(_) => "string".to_owned(),
            Self::Numeric(_) => "number".to_owned(),
            Self::Stream(_) => "stream".to_owned(),
        }
    }
}
