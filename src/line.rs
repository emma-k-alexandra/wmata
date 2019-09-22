use std::error;
use std::fmt;

pub const STATIONS: &'static str = "https://api.wmata.com/Rail.svc/json/jStations";

pub struct Line {
    pub api_key: String,
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

impl LineCode {
    pub fn to_string<'a>(&self) -> &'a str {
        match &self {
            LineCode::Red => "RD",
            LineCode::Blue => "BL",
            LineCode::Yellow => "YL",
            LineCode::Orange => "OR",
            LineCode::Green => "GR",
            LineCode::Silver => "SV",
        }
    }

    pub fn from(string: &str) -> Result<LineCode, StringIsNotLineCodeError> {
        match string {
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

pub trait ToLineCode {
    fn to_line_code(&self) -> Result<LineCode, StringIsNotLineCodeError>;
}

impl ToLineCode for &str {
    fn to_line_code(&self) -> Result<LineCode, StringIsNotLineCodeError> {
        LineCode::from(&self)
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
