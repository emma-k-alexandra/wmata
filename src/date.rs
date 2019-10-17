//! Date related structs and implementations
use chrono::{DateTime, FixedOffset, ParseResult, TimeZone};
use serde::{self, Deserialize, Deserializer};

const FORMAT: &str = "%Y-%m-%dT%H:%M:%S";
const HOUR: i32 = 3600;

pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Date {
    pub fn new(year: u16, month: u8, day: u8) -> Self {
        Date { year, month, day }
    }
}

impl ToString for Date {
    fn to_string(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }
}

fn string_time_to_date(time: &str) -> ParseResult<DateTime<FixedOffset>> {
    FixedOffset::west(HOUR * 5).datetime_from_str(time, FORMAT)
}

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    string_time_to_date(&String::deserialize(deserializer)?).map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    String::deserialize(deserializer).map(|s| string_time_to_date(&s).ok())
}
