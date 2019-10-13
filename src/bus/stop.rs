use crate::bus::client::responses;
use crate::bus::traits::NeedsStop;
use crate::error::Error;
use crate::traits::Fetch;
use serde::{Deserialize, de::{Deserializer, Error as SerdeError}};

#[derive(Debug)]
pub struct Stop(pub String);

impl Stop {
    pub fn new(stop: &str) -> Self {
        Stop(stop.to_string())
    }
}

impl Fetch for Stop {}

impl NeedsStop for Stop {}

// Overwriting NeedsStop methods
impl Stop {
    /// Next bus arrivals at this stop.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476365e031f590f38092508/operations/5476365e031f5909e4fe331d)
    ///
    /// # Examples
    /// ```
    /// use wmata::Stop;
    ///
    /// assert!(Stop::new("1001195").next_buses("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn next_buses(&self, api_key: &str) -> Result<responses::Predictions, Error> {
        <Self as NeedsStop>::next_buses(&self, self, api_key)
    }

    /// Buses scheduled at this stop for an optional given date.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d6c?)
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Examples
    /// ```
    /// use wmata::Stop;
    ///
    /// assert!(Stop::new("1001195").schedule(None, "9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    ///
    /// with date
    /// ```
    /// use wmata::Stop;
    ///
    /// assert!(Stop::new("1001195").schedule(Some("2019-10-02"), "9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn schedule(
        &self,
        date: Option<&str>,
        api_key: &str,
    ) -> Result<responses::StopSchedule, Error> {
        self.stop_schedule(self, date, api_key)
    }
}

impl<'de> Deserialize<'de> for Stop {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let stop = String::deserialize(deserializer)?;

        if [""].contains(&stop.as_str()) {
            return Err(SerdeError::custom("Stop isn't present"))
        }

        Ok(Stop::new(&stop))
    }
}