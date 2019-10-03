//! Traits used for WMATA clients
use crate::error::{Error, ErrorResponse};

use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

/// A trait indicating that the implementor provides a WMATA API Key.
pub trait ApiKey {
    /// Returns the WMATA API Key the implementor contains.
    /// # Examples
    /// ```
    /// use wmata::BusClient
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert_eq!(client.api_key(), "9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn api_key(&self) -> &str;
}

/// A trait indicating the implementor can request and deserialize data
/// from the WMATA API.
pub trait Fetch: Requester + Deserializer {
    /// Requests and deserializes JSON data from a WMATA endpoint.
    /// Used internally by MetroRail and MetroBus clients.
    fn fetch<U, V>(&self, path: &str, query: Option<V>) -> Result<U, Error>
    where
        U: DeserializeOwned,
        V: Serialize + Sized,
    {
        self.request(path, query).and_then(Self::deserialize)
    }
}

/// A trait indicating the implementor can request data from the 
/// WMATA API.
pub trait Requester: ApiKey {
    /// Requests data JSON data from a WMATA endpoint.
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

/// A trait indicating the implementor can deserialize data from the
/// WMATA API.
pub trait Deserializer {
    /// Deserializes JSON data from a WMATA endpoint.
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

/// Auto implement Requester where ApiKey is present.
impl<T> Requester for T where T: ApiKey {}

/// Auto implement Deserializer where ApiKey is present.
impl<T> Deserializer for T where T: ApiKey {}

/// Auto implement Fetch where ApiKey is present.
impl<T> Fetch for T where T: ApiKey {}
