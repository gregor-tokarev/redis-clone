#[derive(Debug)]
pub enum ConfigCommandAction {
    Get(String),
    Unrecognized,
}

#[derive(Debug)]
pub struct ConfigCommand {
    pub action: ConfigCommandAction,
}

impl ConfigCommand {
    pub fn from_statements(statements: &[&str]) -> Self {
        match statements[0] {
            "get" => Self {
                action: ConfigCommandAction::Get(statements[1].to_string()),
            },
            _ => Self {
                action: ConfigCommandAction::Unrecognized,
            },
        }
    }
}
