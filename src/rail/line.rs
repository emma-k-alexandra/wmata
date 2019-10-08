//! Codes for each MetroRail line.
use crate::error::Error;
use crate::rail::client::responses;
use crate::rail::traits::NeedsLineCode;
use crate::traits::Fetch;
use std::{error, fmt, str::FromStr};

/// All MetroRail lines.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum LineCode {
    Red,
    Blue,
    Yellow,
    Orange,
    Green,
    Silver,
}

impl Fetch for LineCode {}

impl NeedsLineCode for LineCode {}

impl LineCode {
    pub fn stations(&self, api_key: String) -> Result<responses::Stations, Error> {
        self.stations_on(Some(*self), &api_key)
    }
}

impl ToString for LineCode {
    fn to_string(&self) -> String {
        match self {
            LineCode::Red => "RD".to_string(),
            LineCode::Blue => "BL".to_string(),
            LineCode::Yellow => "YL".to_string(),
            LineCode::Orange => "OR".to_string(),
            LineCode::Green => "GR".to_string(),
            LineCode::Silver => "SV".to_string(),
        }
    }
}

impl FromStr for LineCode {
    type Err = StringIsNotLineCodeError;

    /// Converts a string to a [`LineCode`].
    ///
    /// # Examples
    /// ```
    /// use wmata::LineCode;
    ///
    /// let line_code: LineCode = "RD".parse().unwrap();
    ///
    /// assert_eq!(LineCode::Red, line_code);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RD" => Ok(LineCode::Red),
            "BL" => Ok(LineCode::Blue),
            "YL" => Ok(LineCode::Yellow),
            "OR" => Ok(LineCode::Orange),
            "GR" => Ok(LineCode::Green),
            "SV" => Ok(LineCode::Silver),
            _ => Err(StringIsNotLineCodeError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringIsNotLineCodeError;

impl fmt::Display for StringIsNotLineCodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provided string is not a valid line code.")
    }
}

impl error::Error for StringIsNotLineCodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
