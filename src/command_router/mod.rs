use core::fmt;
use std::{collections::HashMap, iter::zip, str::FromStr, time::Duration};

use config::ConfigCommand;

use crate::storage::item::split_id;

pub mod config;

#[derive(Debug, Clone)]
pub struct SetCommand {
    pub key: String,
    pub value: String,
    pub expire_after: Option<String>,
}

#[derive(Debug, Clone)]
pub struct GetCommand {
    pub key: String,
}

#[derive(Debug)]
pub struct EchoCommand {
    pub echo: String,
}

#[derive(Debug)]
pub struct KeysCommand {
    pub pattern: String,
}

#[derive(Debug, Clone)]
pub struct IncrCommand {
    pub key: String,
    pub step: isize,
}

#[derive(Debug, Clone)]
pub struct TypeCommand {
    pub key: String,
}

#[derive(Debug, Clone)]
pub struct XAddCommand {
    pub key: String,
    pub id: String,
    pub data: HashMap<String, String>,
}

#[derive(Debug, Clone)]
pub enum XRangeStatement {
    Id((Option<isize>, Option<isize>)),
    Positive,
    Negative,
}

impl FromStr for XRangeStatement {
    type Err = ParsingError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "-" => Ok(Self::Negative),
            "+" => Ok(Self::Positive),
            id => Ok(Self::Id(split_id(id.to_owned()))),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XRangeCommand {
    pub key: String,
    pub start_statement: XRangeStatement,
    pub end_statement: XRangeStatement,
}

#[derive(Debug, Clone)]
pub struct XReadCommand {
    pub keys: Vec<(String, String)>,
    pub blocking: Option<Duration>
}

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(EchoCommand),
    Set(SetCommand),
    Get(GetCommand),
    Replconf,
    Info,
    Incr(IncrCommand),
    Config(ConfigCommand),
    Keys(KeysCommand),
    Multi,
    Exec,
    Discard,
    Type(TypeCommand),
    XAdd(XAddCommand),
    XRange(XRangeCommand),
    XRead(XReadCommand),
    Unrecognized,
}

#[derive(Debug, Clone)]
pub struct ParsingError;

impl fmt::Display for ParsingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error parsing command")
    }
}

impl<'a> Command {
    pub fn new(command_str: &str) -> Result<Self, ParsingError> {
        let lowered = command_str.to_lowercase();
        let mut lines = lowered.lines();

        let number_of_commands_line = lines.next().ok_or(ParsingError)?;
        let number_of_commands: usize = number_of_commands_line[1..]
            .parse()
            .map_err(|_| ParsingError)?;

        let mut main_statements: Vec<&str> = vec![];

        for _i in 0..number_of_commands {
            lines.next().ok_or(ParsingError)?;
            let main_statement = lines.next().ok_or(ParsingError)?;

            main_statements.push(main_statement);
        }

        Ok(match *main_statements.first().ok_or(ParsingError)? {
            "ping" => Self::Ping,
            "echo" => Self::Echo(EchoCommand {
                echo: main_statements[1].to_owned(),
            }),
            "set" => {
                let mut set_command = SetCommand {
                    key: main_statements[1].to_owned(),
                    value: main_statements[2].to_owned(),
                    expire_after: None,
                };

                if let Some(other) = main_statements.get(3) {
                    if other == &"px" {
                        set_command.expire_after = Some(main_statements[4].to_owned())
                    }
                }

                Self::Set(set_command)
            }
            "get" => Self::Get(GetCommand {
                key: main_statements[1].to_owned(),
            }),
            "info" => Command::Info,
            "replconf" => Command::Replconf,
            "config" => Command::Config(ConfigCommand::from_statements(&main_statements[1..])),
            "incr" => Command::Incr(IncrCommand {
                key: main_statements[1].to_owned(),
                step: match main_statements.get(2) {
                    Some(incr) => incr.parse().unwrap(),
                    None => 1,
                },
            }),
            "keys" => Command::Keys(KeysCommand {
                pattern: main_statements[1].to_owned(),
            }),
            "multi" => Command::Multi,
            "exec" => Command::Exec,
            "discard" => Command::Discard,
            "type" => Command::Type(TypeCommand {
                key: main_statements[1].to_owned(),
            }),
            "xadd" => {
                let mut data: HashMap<String, String> = HashMap::new();

                let mut iter = main_statements.into_iter().skip(1);

                let key = iter.next().unwrap().to_owned();
                let id = iter.next().unwrap().to_owned();

                let mut data_key: Option<String> = None;
                let mut data_value: Option<String> = None;
                for itm in iter {
                    let mut splitted = itm.split(' ');
                    if splitted.clone().count() == 2 {
                        let key = splitted.next().unwrap();
                        let value = splitted.next().unwrap();

                        data.insert(key.to_owned(), value.to_owned());
                        continue;
                    };

                    if data_key.is_none() && data_value.is_none() {
                        data_key = Some(itm.to_owned());
                    } else if data_key.is_some() && data_value.is_none() {
                        data_value = Some(itm.to_owned());

                        if let (Some(key), Some(value)) = (data_key.clone(), data_value.clone()) {
                            data.insert(key, value);

                            data_key = None;
                            data_value = None;
                        }
                    }
                }

                Command::XAdd(XAddCommand { key, id, data })
            }
            "xrange" => Command::XRange(XRangeCommand {
                key: main_statements[1].to_owned(),
                start_statement: main_statements[2].parse().unwrap(),
                end_statement: main_statements[3].parse().unwrap(),
            }),
            "xread" => {
                let mut blocking = None;
                let mut base_iter = main_statements.clone().into_iter().skip(1);

                let next = base_iter.next().unwrap();

                let is_blocking = next == "block";
                if is_blocking {
                   let millis = base_iter.next().unwrap(); 
                   base_iter.next().unwrap(); // skipping "streams"

                   blocking = Some(Duration::from_millis(millis.parse().unwrap()))
                }


                let base_vec: Vec<&str> = base_iter.collect();

                let list: Vec<&str> = if base_vec.len() == 3 {
                    base_vec[2].split(' ').collect()
                } else {
                    base_vec.clone().into_iter().collect()
                };

                let keys = list.clone().into_iter().take(list.len() / 2);
                let ids = list.clone().into_iter().skip(list.len() / 2);

                let data = zip(keys, ids)
                    .map(|(key, id)| (key.to_owned(), id.to_owned()))
                    .collect();

                Command::XRead(XReadCommand { keys: data, blocking })
            }
            _ => Self::Unrecognized,
        })
    }
}
