//! Codes for each MetroRail line.
use crate::error::Error;
use crate::rail::client::responses;
use crate::rail::traits::NeedsLine;
use crate::traits::Fetch;
use std::{error, fmt, str::FromStr};

/// All MetroRail lines.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Line {
    Red,
    Blue,
    Yellow,
    Orange,
    Green,
    Silver,
}

impl Fetch for Line {}

impl NeedsLine for Line {}

impl Line {
    /// Station location and address information for all stations on this line.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330c)
    /// 
    /// # Examples
    /// ```
    /// use wmata::Line;
    ///
    /// assert!(Line::Red.stations("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn stations(&self, api_key: &str) -> Result<responses::Stations, Error> {
        self.stations_on(Some(*self), &api_key)
    }
}

impl ToString for Line {
    fn to_string(&self) -> String {
        match self {
            Line::Red => "RD".to_string(),
            Line::Blue => "BL".to_string(),
            Line::Yellow => "YL".to_string(),
            Line::Orange => "OR".to_string(),
            Line::Green => "GR".to_string(),
            Line::Silver => "SV".to_string(),
        }
    }
}

impl FromStr for Line {
    type Err = StringIsNotLineError;

    /// Converts a string to a [`Line`].
    ///
    /// # Examples
    /// ```
    /// use wmata::Line;
    ///
    /// let line_code: Line = "RD".parse().unwrap();
    ///
    /// assert_eq!(Line::Red, line_code);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RD" => Ok(Line::Red),
            "BL" => Ok(Line::Blue),
            "YL" => Ok(Line::Yellow),
            "OR" => Ok(Line::Orange),
            "GR" => Ok(Line::Green),
            "SV" => Ok(Line::Silver),
            _ => Err(StringIsNotLineError),
        }
    }
}

#[derive(Debug, Clone)]
pub struct StringIsNotLineError;

impl fmt::Display for StringIsNotLineError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provided string is not a valid line code.")
    }
}

impl error::Error for StringIsNotLineError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
