pub mod line;
pub mod rail;
pub mod station;

pub mod error;
mod urls;

use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

use crate::error::{responses, Error};

fn deserialize<T>(response: String) -> Result<T, Error>
where
    T: DeserializeOwned,
{
    serde_json::from_str::<T>(&response).or_else(|_| {
        match serde_json::from_str::<responses::Error>(&response) {
            Ok(json) => Err(Error::new(json.message.to_string())),
            Err(err) => Err(Error::new(err.to_string())),
        }
    })
}

fn request_and_deserialize<T, U>(api_key: &str, path: &str, query: Option<U>) -> Result<T, Error>
where
    T: DeserializeOwned,
    U: Serialize + Sized,
{
    let mut request = reqwest::Client::new().get(path);

    if let Some(some_query) = query {
        request = request.query(&some_query)
    }

    request
        .header("api_key", api_key)
        .send()
        .and_then(|mut response| response.text())
        .map_err(|err| Error::new(err.to_string()))
        .and_then(deserialize)
}
