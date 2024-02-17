use std::fmt;

#[derive(Debug, Clone)]
pub(crate) struct GeneratorError {
    message: String,
}

impl GeneratorError {
    pub fn new(message: &str) -> Self {
        GeneratorError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for GeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
