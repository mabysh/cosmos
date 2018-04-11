use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct CosmError {
    description: String,
}

impl CosmError {
    pub fn new(desc: &str) -> CosmError {
        CosmError {
            description: String::from(desc),
        }
    }
}

impl Display for CosmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.description().fmt(f)
    }
}

impl Error for CosmError {
    fn description(&self) -> &str {
        &self.description.as_str()
    }
}
