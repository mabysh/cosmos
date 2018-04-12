use std::error::Error;
use std::fmt::{self, Display};

#[derive(Debug)]
pub struct CosmError {
    message: String,
    kind: CosmErrorKind,
    cause: Option<Box<CosmError>>,
}

impl CosmError {
    pub fn new(desc: &str, kind: CosmErrorKind, cause: Option<CosmError>) -> CosmError {
        let c = match cause {
            Some(er) => Some(Box::new(er)),
            None => None,
        };
        CosmError {
            message: String::from(desc),
            kind: kind,
            cause: c,
        }
    }

    pub fn from_std_error(e: &Error) -> CosmError {
        let cause = match e.cause() {
            Some(er) => Some(Box::new(CosmError::from_std_error(er))),
            None => None,
        };
        CosmError {
            message: String::from(e.description()),
            kind: CosmErrorKind::StdLibError,
            cause: cause,
        }
    }
}

#[derive(Debug)]
pub enum CosmErrorKind {
    StdLibError,
    MissingArg,
    InvalidArg,
}

impl Error for CosmError {
    fn description(&self) -> &str {
        &self.message.as_str()
    }

    fn cause(&self) -> Option<&Error> {
        match self.cause {
            Some(ref b) => Some(b.as_ref()),
            None => None,
        }
    }
}

impl Display for CosmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut info = format!("*** CosmError ({:?}): {}", self.kind, self.description());
        match self.cause() {
            Some(er) => {
                info.push_str(format!("\nCaused by: {}", er).as_str());
            }
            None => {}
        }
        write!(f, "{}", info)
    }
}
