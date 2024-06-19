use core::fmt;
use std::any::Any;

#[derive(Debug)]
pub enum Command {
    Ping,
    Echo(String),
    Set(String, String),
    Get(String),
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

        let number_of_commands_line = lines.next().ok_or_else(|| ParsingError)?;
        let number_of_commands: usize = number_of_commands_line[1..]
            .parse()
            .or_else(|_| Err(ParsingError))?;

        let mut main_statements: Vec<&str> = vec![];

        for _i in 0..number_of_commands {
            lines.next().ok_or_else(|| ParsingError)?;
            let main_statement = lines.next().ok_or_else(|| ParsingError)?;

            main_statements.push(main_statement);
        }

        Ok(match *main_statements.first().ok_or_else(|| ParsingError)? {
            "ping" => Self::Ping,
            "echo" => Self::Echo(main_statements[1].to_owned()),
            "set" => Self::Set(main_statements[1].to_owned(), main_statements[2].to_owned()),
            "get" => Self::Get(main_statements[1].to_owned()),
            _ => Self::Unrecognized
        })
    }

    fn length_statment(statment: &str) -> Result<isize, ParsingError> {
        match statment[1..].parse::<isize>() {
            Ok(num) => Ok(num),
            Err(_e) => Err(ParsingError),
        }
    }
}
