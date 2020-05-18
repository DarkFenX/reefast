use std::error;
use std::fmt;
use std::io;

use serde_json;

pub(super) trait FromPathErr<T> {
    fn from_path_err<U: Into<String>>(err: T, path: U) -> Self;
}

#[derive(Debug)]
pub(super) struct Error {
    pub(super) msg: String,
}

impl Error {
    pub fn new<T: Into<String>>(msg: T) -> Error {
        Error { msg: msg.into() }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}

impl FromPathErr<io::Error> for Error {
    fn from_path_err<T: Into<String>>(err: io::Error, path: T) -> Self {
        Error::new(format!("{} reading failed: {}", path.into(), err))
    }
}

impl FromPathErr<serde_json::Error> for Error {
    fn from_path_err<T: Into<String>>(err: serde_json::Error, path: T) -> Self {
        Error::new(format!("{} parsing failed: {}", path.into(), err))
    }
}