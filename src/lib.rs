use serde::Deserialize;
use serde_json;

use crate::error::{responses as error_responses, Error as WMATAError};

fn serialize<'de, T>(response: &'de str) -> Result<T, WMATAError>
where
    T: Deserialize<'de>,
{
    serde_json::from_str::<T>(&response).or_else(|_| {
        match serde_json::from_str::<error_responses::Error>(&response) {
            Ok(json) => Err(WMATAError::new(json.message.to_string())),
            Err(err) => Err(WMATAError::new(err.to_string())),
        }
    })
}

pub mod line;
pub mod rail;
pub mod station;

pub mod error;
