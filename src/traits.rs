use crate::error::{Error, ErrorResponse};

use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub trait ApiKey {
    fn api_key(&self) -> &str;
}

pub trait Fetch: Requester + Deserializer {
    fn fetch<U, V>(&self, path: &str, query: Option<V>) -> Result<U, Error>
    where
        U: DeserializeOwned,
        V: Serialize + Sized,
    {
        self.request(path, query).and_then(Self::deserialize)
    }
}

pub trait Requester: ApiKey {
    fn request<T>(&self, path: &str, query: Option<T>) -> Result<String, Error>
    where
        T: Serialize + Sized,
    {
        let mut request = reqwest::Client::new().get(path);

        if let Some(some_query) = query {
            request = request.query(&some_query)
        }

        request
            .header("api_key", self.api_key())
            .send()
            .and_then(|mut response| response.text())
            .map_err(|err| Error::new(err.to_string()))
    }
}

pub trait Deserializer {
    fn deserialize<T>(response: String) -> Result<T, Error>
    where
        T: DeserializeOwned,
    {
        serde_json::from_str::<T>(&response).or_else(|original_err| {
            match serde_json::from_str::<ErrorResponse>(&response) {
                Ok(json) => Err(Error::new(json.message.to_string())),
                Err(_) => Err(Error::new(original_err.to_string())),
            }
        })
    }
}

impl<T> Requester for T where T: ApiKey {}

impl<T> Deserializer for T where T: ApiKey {}

impl<T> Fetch for T where T: ApiKey {}
