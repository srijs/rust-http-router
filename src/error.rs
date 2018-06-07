use std::error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct Error {
    message: String,
}

impl Error {
    pub(crate) fn from_err<E>(err: E) -> Error
    where
        E: ::std::error::Error,
    {
        Error {
            message: err.to_string(),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "router error: {}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        "router error"
    }
}
