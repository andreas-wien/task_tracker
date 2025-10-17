use std::fmt::Display;

pub struct CLI {
    arguments: Vec<String>,
}

pub struct CLIInvalidArgumentsError(pub String);

impl Display for CLIInvalidArgumentsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl CLI {
    pub fn new() -> Result<Self, CLIInvalidArgumentsError> {
        let arguments: Vec<String> = std::env::args().collect();
        if arguments.len() < 2 {
            return Err(
                CLIInvalidArgumentsError(format!("Invalid arguments: {}", arguments.join(", ")))
            );
        }
        Ok(CLI { arguments })
    }

    pub fn arguments(&self) -> &Vec<String> {
        &self.arguments
    }
}
