//! Traits used for WMATA clients
use crate::error::{Error, ErrorResponse};
use crate::types::Request as WMATARequest;

use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

/// A trait indicating the implementor can request and deserialize data
/// from the WMATA API.
pub trait Fetch: Requester + Deserializer {
    // / Requests and deserializes JSON data from a WMATA endpoint.
    // / Used internally by MetroRail and MetroBus clients.
    fn fetch<U, V>(&self, wmata_request: WMATARequest) -> Result<U, Error>
    where
        U: DeserializeOwned,
        V: Serialize + Sized,
    {
        self.request(wmata_request).and_then(Self::deserialize)
    }
}

/// A trait indicating the implementor can request data from the
/// WMATA API.
pub trait Requester {
    /// Requests data JSON data from a WMATA endpoint.
    fn request<T>(&self, wmata_request: WMATARequest) -> Result<String, Error>
    where
        T: Serialize + Sized,
    {
        let mut request = reqwest::Client::new().get(&wmata_request.path);

        if let Some(query) = wmata_request.query {
            request = request.query(&query)
        }

        request
            .header("api_key", wmata_request.api_key)
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
impl<T> Requester for T where T: Fetch {}

/// Auto implement Deserializer where ApiKey is present.
impl<T> Deserializer for T where T: Fetch {}

/// Auto implement Fetch where ApiKey is present.
impl<T> Fetch for T where T: ApiKey {}
