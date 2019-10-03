//! Errors used throughout the crate.
use serde::{Deserialize, Serialize};
use std::{error, fmt};

/// An error, generated from the WMATA API
#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub message: String,
}

impl Error {
    pub fn new(message: String) -> Self {
        Error { message }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}

/// An error from the WMATA API
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorResponse<'a> {
    pub message: &'a str,
}
