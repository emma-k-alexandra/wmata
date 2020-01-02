//! Internal requests structs and traits.
use crate::error::{Error, ErrorResponse};

use async_trait::async_trait;
use reqwest;
use serde::de::DeserializeOwned;
use serde_json;

pub struct Request<'a> {
    pub api_key: &'a str,
    pub path: &'a str,
    pub query: Option<Vec<(&'a str, String)>>,
}

impl<'a> Request<'a> {
    pub fn new(api_key: &'a str, path: &'a str, query: Option<Vec<(&'a str, String)>>) -> Self {
        Request {
            api_key,
            path,
            query,
        }
    }
}

/// A trait indicating the implementor can request and deserialize data
/// from the WMATA API.
#[async_trait]
pub trait Fetch: Requester + Deserializer {
    // / Requests and deserializes JSON data from a WMATA endpoint.
    // / Used internally by MetroRail and MetroBus clients.
    async fn fetch<U>(&self, wmata_request: Request<'_>) -> Result<U, Error>
    where
        U: DeserializeOwned,
    {
        self.request(wmata_request)
            .await
            .and_then(Self::deserialize)
    }
}

/// A trait indicating the implementor can request data from the
/// WMATA API.
#[async_trait]
pub trait Requester {
    /// Requests data JSON data from a WMATA endpoint.
    async fn request(&self, wmata_request: Request<'_>) -> Result<String, Error> {
        let mut request = reqwest::Client::new().get(wmata_request.path);

        if let Some(query) = wmata_request.query {
            request = request.query(&query)
        }

        let response = request
            .header("api_key", wmata_request.api_key)
            .send()
            .await;

        match response {
            Ok(response) => response
                .text()
                .await
                .map_err(|err| Error::new(err.to_string())),
            Err(err) => Err(Error::new(err.to_string())),
        }
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

/// Auto implement Requester where Fetch is present.
impl<T> Requester for T where T: Fetch {}

/// Auto implement Deserializer where Fetch is present.
impl<T> Deserializer for T where T: Fetch {}
