pub mod responses;
mod tests;

use crate::error::Error;
use crate::line::{responses as line_responses, LineCode, STATIONS};
use crate::serialize;
use crate::station::{responses as station_responses, StationCode, STATION_TO_STATION};
use reqwest;
use std::str::FromStr;

const LINES: &'static str = "https://api.wmata.com/Rail.svc/json/jLines";
const ENTRANCES: &'static str = "https://api.wmata.com/Rail.svc/json/jStationEntrances";
const POSITIONS: &'static str = "https://api.wmata.com/TrainPositions/TrainPositions";
const ROUTES: &'static str = "https://api.wmata.com/TrainPositions/StandardRoutes";
const CIRCUITS: &'static str = "https://api.wmata.com/TrainPositions/TrackCircuits";
const ELEVATOR_AND_ESCALATOR_INCIDENTS: &'static str =
    "https://api.wmata.com/Incidents.svc/json/ElevatorIncidents";
const INCIDENTS: &'static str = "https://api.wmata.com/Incidents.svc/json/Incidents";

pub struct Rail {
    pub api_key: String,
}

impl FromStr for Rail {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Rail {
            api_key: s.to_string(),
        })
    }
}

impl Rail {
    pub fn lines<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::Lines, Error>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(LINES)
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::Lines>(&response)),
        );
    }

    pub fn entrances<F>(&self, latitude: f64, longitude: f64, radius: f64, completion: F)
    where
        F: FnOnce(Result<responses::StationEntrances, Error>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(ENTRANCES)
                .query(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)])
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::StationEntrances>(&response)),
        );
    }

    pub fn stations<F>(&self, line: Option<LineCode>, completion: F)
    where
        F: FnOnce(Result<line_responses::Stations, Error>) -> (),
    {
        let mut response = reqwest::Client::new().get(STATIONS);

        if let Some(line_code) = line {
            response = response.query(&[("LineCode", line_code.to_string())]);
        }

        completion(
            response
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<line_responses::Stations>(&response)),
        );
    }

    pub fn station<F>(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
        completion: F,
    ) where
        F: FnOnce(Result<station_responses::StationToStationInfos, Error>) -> (),
    {
        let mut response = reqwest::Client::new().get(STATION_TO_STATION);

        let mut query: Vec<(String, String)> = vec![];

        if let Some(station_code) = from_station {
            query.push(("FromStationCode".to_string(), station_code.to_string()));
        }

        if let Some(station_code) = to_destination_station {
            query.push(("ToStationCode".to_string(), station_code.to_string()));
        }

        response = response.query(&query);

        completion(
            response
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| {
                    serialize::<station_responses::StationToStationInfos>(&response)
                }),
        );
    }

    pub fn positions<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrainPositions, Error>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(POSITIONS)
                .query(&[("contentType", "json")])
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::TrainPositions>(&response)),
        );
    }

    pub fn routes<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::StandardRoutes, Error>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(ROUTES)
                .query(&[("contentType", "json")])
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::StandardRoutes>(&response)),
        );
    }

    pub fn circuits<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrackCircuits, Error>) -> (),
    {
        completion(
            reqwest::Client::new()
                .get(CIRCUITS)
                .query(&[("contentType", "json")])
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::TrackCircuits>(&response)),
        );
    }

    pub fn elevator_and_escalator_incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::ElevatorAndEscalatorIncidents, Error>) -> (),
    {
        let mut response = reqwest::Client::new().get(ELEVATOR_AND_ESCALATOR_INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        completion(
            response
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| {
                    serialize::<responses::ElevatorAndEscalatorIncidents>(&response)
                }),
        );
    }

    pub fn incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::RailIncidents, Error>) -> (),
    {
        let mut response = reqwest::Client::new().get(INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        completion(
            response
                .header("api_key", &self.api_key)
                .send()
                .and_then(|mut response| response.text())
                .map_err(|err| Error::new(err.to_string()))
                .and_then(|response| serialize::<responses::RailIncidents>(&response)),
        );
    }
}
