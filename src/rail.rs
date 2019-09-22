use reqwest;
use serde_json;

pub mod responses;
pub mod tests;

use super::line::{LineCode, STATIONS};
use super::station::StationCode;

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

impl Rail {
    pub fn lines<F>(&self, completion: F)
    where
        F: FnOnce(responses::Lines) -> (),
    {
        let response = reqwest::Client::new()
            .get(LINES)
            .header("api_key", &self.api_key)
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
            .header("api_key", &self.api_key)
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
            .header("api_key", &self.api_key)
            .send()
            .expect("Send failed")
            .text()
            .expect("Text failed");

        let json: responses::Stations = serde_json::from_str(&response).expect("from_str failed");

        completion(json);
    }

    pub fn station<F>(&self, fromStation: Option<StationCode>, toDestinationStation: Option<StationCode>, completion: F)
    where
        F: FnOnce(responses::StationToStationInfos) -> (),
    {

    }
}
