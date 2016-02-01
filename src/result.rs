use std::result;
use std::error;
use std::fmt;
use std::io;
use yaml_rust;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    fn new(msg: String) -> Error {
        Error{ message: msg }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message[..]
    }
}

impl From<yaml_rust::scanner::ScanError> for Error {
    fn from(err: yaml_rust::scanner::ScanError) -> Error {
        Error::new(format!("Could not parse YAML: {}", err))
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::new(format!("IO Error: {}", err))
    }
}

impl From<String> for Error {
    fn from(err: String) -> Error {
        Error::new(err)
    }
}

impl<'a> From<&'a str> for Error {
    fn from(err: &'a str) -> Error {
        Error::new(format!("{}", err))
    }
}
