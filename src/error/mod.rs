use std::fmt;
#[derive(Debug, Clone)]
pub(crate) struct BaseError {
    message: String,
}

impl BaseError {
    pub fn new(message: &str) -> Self {
        BaseError {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for BaseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}
