use reqwest;
use serde_json;

pub mod responses;
pub mod tests;

use super::line::{LineCode, STATIONS};
use super::station::{StationCode, STATION_TO_STATION};

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

impl Rail<'_> {
    pub fn lines<F>(&self, completion: F)
    where
        F: FnOnce(responses::Lines) -> (),
    {
        let response = reqwest::Client::new()
            .get(LINES)
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::Lines = serde_json::from_str(&response).expect("from_str failed");

        completion(json);
    }

    pub fn entrances<F>(&self, latitude: f64, longitude: f64, radius: f64, completion: F)
    where
        F: FnOnce(responses::StationEntrances) -> (),
    {
        let response = reqwest::Client::new()
            .get(ENTRANCES)
            .query(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)])
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::StationEntrances =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json);
    }

    pub fn stations<F>(&self, line: Option<LineCode>, completion: F)
    where
        F: FnOnce(responses::Stations) -> (),
    {
        let mut response = reqwest::Client::new().get(STATIONS);

        if let Some(line_code) = line {
            response = response.query(&[("LineCode", line_code.to_string())]);
        }

        let response = response
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::Stations = serde_json::from_str(&response).expect("from_str failed");

        completion(json);
    }

    pub fn station<F>(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
        completion: F,
    ) where
        F: FnOnce(responses::StationToStationInfos) -> (),
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

        let response = response
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::StationToStationInfos =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }

    pub fn positions<F>(&self, completion: F)
    where
        F: FnOnce(responses::TrainPositions) -> (),
    {
        let response = reqwest::Client::new()
            .get(POSITIONS)
            .query(&[("contentType", "json")])
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::TrainPositions =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }

    pub fn routes<F>(&self, completion: F)
    where
        F: FnOnce(responses::StandardRoutes) -> (),
    {
        let response = reqwest::Client::new()
            .get(ROUTES)
            .query(&[("contentType", "json")])
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::StandardRoutes =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }

    pub fn circuits<F>(&self, completion: F)
    where
        F: FnOnce(responses::TrackCircuits) -> (),
    {
        let response = reqwest::Client::new()
            .get(CIRCUITS)
            .query(&[("contentType", "json")])
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::TrackCircuits =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }

    pub fn elevator_and_escalator_incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(responses::ElevatorAndEscalatorIncidents) -> (),
    {
        let mut response = reqwest::Client::new().get(ELEVATOR_AND_ESCALATOR_INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        let response = response
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::ElevatorAndEscalatorIncidents =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }

    pub fn incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(responses::RailIncidents) -> (),
    {
        let mut response = reqwest::Client::new().get(INCIDENTS);

        if let Some(station_code) = station {
            response = response.query(&[("StationCode", station_code.to_string())]);
        }

        let response = response
            .header("api_key", self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::RailIncidents =
            serde_json::from_str(&response).expect("from_str failed");

        completion(json)
    }
}
