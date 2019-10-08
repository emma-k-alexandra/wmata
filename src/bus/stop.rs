use crate::bus::client::responses;
use crate::bus::traits::NeedsStop;
use crate::error::Error;
use crate::traits::Fetch;

pub struct Stop<'a>(pub &'a str);

impl<'a> Fetch for Stop<'a> {}

impl<'a> NeedsStop for Stop<'a> {}

// Overwriting NeedsStop methods
impl<'a> Stop<'a> {
    pub fn next_buses(&self, api_key: &str) -> Result<responses::Predictions, Error> {
        <Self as NeedsStop>::next_buses(&self, self, api_key)
    }

    pub fn schedule(
        &self,
        date: Option<&str>,
        api_key: &str,
    ) -> Result<responses::StopSchedule, Error> {
        self.stop_schedule(self, date, api_key)
    }
}
