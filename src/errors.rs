use std::error;
use std::fmt;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum BuildError {
    InvalidParamName,
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BuildError::InvalidParamName => write!(f, "invalid param name"),
            BuildError::__Nonexhaustive => unreachable!(),
        }
    }
}

impl error::Error for BuildError {
    fn description(&self) -> &str {
        match self {
            BuildError::InvalidParamName => "invalid param name",
            BuildError::__Nonexhaustive => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum ParamError {
    NotFound,
    Decode(Utf8Error),
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamError::NotFound => write!(f, "route param not found"),
            ParamError::Decode(err) => write!(f, "route param decode error: {}", err),
        }
    }
}

impl error::Error for ParamError {
    fn description(&self) -> &str {
        match self {
            ParamError::NotFound => "route param not found",
            ParamError::Decode(_) => "route param decode error",
        }
    }
}
