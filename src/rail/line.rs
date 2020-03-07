//! Codes for each MetroRail line.
use crate::{
    error::Error,
    rail::{client::responses, traits::NeedsLine},
    requests::Fetch,
};
use serde::{
    de::{Deserializer, Error as SerdeError},
    Deserialize,
};
use std::{error, fmt, str::FromStr};

/// All MetroRail lines.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Line {
    Red,
    Blue,
    Yellow,
    YellowLineRushPlus,
    Orange,
    Green,
    Silver,
}

impl Fetch for Line {}

impl NeedsLine for Line {}

impl Line {
    /// Provides the hex code color for this line.
    ///
    /// # Example
    /// ```
    /// use wmata::Line;
    /// assert_eq!(Line::Red.color(), "#4CAA61");
    /// ```
    pub fn color(self) -> String {
        match self {
            Line::Red => "#4CAA61".to_string(),
            Line::Blue => "#357ABB".to_string(),
            Line::Yellow => "#F0CD4E".to_string(),
            Line::Orange => "#4CAA61".to_string(),
            Line::Green => "#4CAA61".to_string(),
            Line::Silver => "#A4A6A3".to_string(),
            Line::YellowLineRushPlus => "#F0CD4E".to_string(),
        }
    }

    /// Station location and address information for all stations on this line.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330c)
    ///
    /// # Examples
    /// ```
    /// use wmata::Line;
    /// use tokio_test::block_on;
    ///
    /// let stations = block_on(async { Line::Red.stations("9e38c3eab34c4e6c990828002828f5ed").await });
    /// assert!(stations.is_ok());
    /// ```
    pub async fn stations(self, api_key: &str) -> Result<responses::Stations, Error> {
        self.stations_on(Some(self), &api_key).await
    }
}

impl<'de> Deserialize<'de> for Line {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let line = String::deserialize(deserializer)?;

        Line::from_str(&line).map_err(|_| SerdeError::custom("String provided is not a Line code."))
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
            Line::YellowLineRushPlus => "YLRP".to_string(),
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
            "YLRP" => Ok(Line::YellowLineRushPlus),
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
