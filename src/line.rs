use std::{fmt, error, str::FromStr};

pub mod responses;
mod tests;

pub const STATIONS: &'static str = "https://api.wmata.com/Rail.svc/json/jStations";

pub struct Line<'a> {
    pub api_key: &'a str,
    pub line_code: LineCode,
}

pub enum LineCode {
    Red,
    Blue,
    Yellow,
    Orange,
    Green,
    Silver,
}

impl ToString for LineCode {
    fn to_string(&self) -> String {
        match &self {
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
