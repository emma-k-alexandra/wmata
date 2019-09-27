use reqwest;
use serde::Deserialize;
use serde_json;

pub mod responses;
pub mod tests;

use crate::error::{responses as error_responses, Error as WMATAError};
use crate::line::{responses as line_responses, LineCode, STATIONS};
use crate::station::{responses as station_responses, StationCode, STATION_TO_STATION};

const LINES: &'static str = "https://api.wmata.com/Rail.svc/json/jLines";
const ENTRANCES: &'static str = "https://api.wmata.com/Rail.svc/json/jStationEntrances";
const POSITIONS: &'static str = "https://api.wmata.com/TrainPositions/TrainPositions";
const ROUTES: &'static str = "https://api.wmata.com/TrainPositions/StandardRoutes";
const CIRCUITS: &'static str = "https://api.wmata.com/TrainPositions/TrackCircuits";
const ELEVATOR_AND_ESCALATOR_INCIDENTS: &'static str =
    "https://api.wmata.com/Incidents.svc/json/ElevatorIncidents";
const INCIDENTS: &'static str = "https://api.wmata.com/Incidents.svc/json/Incidents";

pub struct Rail<'a> {
    pub api_key: &'a str,
}

impl<'a> Rail<'a> {
    fn serialize<T>(response: &'a str) -> Result<T, WMATAError>
    where
        T: Deserialize<'a>,
    {
        serde_json::from_str::<T>(&response).or_else(|_| {
            match serde_json::from_str::<error_responses::Error>(&response) {
                Ok(json) => Err(WMATAError::new(json.message.to_string())),
                Err(err) => Err(WMATAError::new(err.to_string())),
            }
        })
    }
}

impl Rail<'_> {
    pub fn lines<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::Lines, WMATAError>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(LINES)
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::Lines>(&response)),
        );
    }

    pub fn entrances<F>(&self, latitude: f64, longitude: f64, radius: f64, completion: F)
    where
        F: FnOnce(Result<responses::StationEntrances, WMATAError>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(ENTRANCES)
                .query(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)])
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::StationEntrances>(&response)),
        );
    }

    pub fn stations<F>(&self, line: Option<LineCode>, completion: F)
    where
        F: FnOnce(Result<line_responses::Stations, WMATAError>) -> (),
    {
        let mut response = reqwest::Client::new().get(STATIONS);

        if let Some(line_code) = line {
            response = response.query(&[("LineCode", line_code.to_string())]);
        }

        completion(
            response
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<line_responses::Stations>(&response)),
        );
    }

    pub fn station<F>(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
        completion: F,
    ) where
        F: FnOnce(Result<station_responses::StationToStationInfos, WMATAError>) -> (),
    {
        let mut response = reqwest::Client::new().get(STATION_TO_STATION);

        let mut query: Vec<(&str, &str)> = vec![];

        if let Some(station_code) = from_station {
            query.push(("FromStationCode", station_code.to_string()));
        }

        if let Some(station_code) = to_destination_station {
            query.push(("ToStationCode", station_code.to_string()));
        }

        response = response.query(&query);

        completion(
            response
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| {
                    Rail::serialize::<station_responses::StationToStationInfos>(&response)
                }),
        );
    }

    pub fn positions<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrainPositions, WMATAError>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(POSITIONS)
                .query(&[("contentType", "json")])
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::TrainPositions>(&response)),
        );
    }

    pub fn routes<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::StandardRoutes, WMATAError>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(ROUTES)
                .query(&[("contentType", "json")])
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::StandardRoutes>(&response)),
        );
    }

    pub fn circuits<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrackCircuits, WMATAError>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(CIRCUITS)
                .query(&[("contentType", "json")])
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::TrackCircuits>(&response)),
        );
    }

    pub fn elevator_and_escalator_incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::ElevatorAndEscalatorIncidents, WMATAError>) -> (),
    {
        let mut response = reqwest::Client::new().get(ELEVATOR_AND_ESCALATOR_INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        completion(
            response
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| {
                    Rail::serialize::<responses::ElevatorAndEscalatorIncidents>(&response)
                }),
        );
    }

    pub fn incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::RailIncidents, WMATAError>) -> (),
    {
        let mut response = reqwest::Client::new().get(INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        completion(
            response
                .header("api_key", self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| WMATAError::new(err.to_string()))
                .and_then(|response| Rail::serialize::<responses::RailIncidents>(&response)),
        );
    }
}
