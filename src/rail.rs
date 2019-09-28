pub mod responses;
mod tests;

use crate::error::Error;
use crate::line::{responses as line_responses, LineCode, STATIONS};
use crate::request_and_deserialize;
use crate::station::{responses as station_responses, StationCode, STATION_TO_STATION};
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
        completion(request_and_deserialize::<responses::Lines, [(); 0]>(
            &self.api_key,
            LINES,
            None,
        ));
    }

    pub fn entrances<F>(&self, latitude: f64, longitude: f64, radius: f64, completion: F)
    where
        F: FnOnce(Result<responses::StationEntrances, Error>) -> (),
    {
        completion(request_and_deserialize(
            &self.api_key,
            ENTRANCES,
            Some(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)]),
        ));
    }

    pub fn stations<F>(&self, line: Option<LineCode>, completion: F)
    where
        F: FnOnce(Result<line_responses::Stations, Error>) -> (),
    {
        let mut query = vec![];

        if let Some(line_code) = line {
            query.push(("LineCode", line_code.to_string()));
        }

        completion(request_and_deserialize(
            &self.api_key,
            STATIONS,
            Some(&query),
        ));
    }

    pub fn station<F>(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
        completion: F,
    ) where
        F: FnOnce(Result<station_responses::StationToStationInfos, Error>) -> (),
    {
        let mut query = vec![];

        if let Some(station_code) = from_station {
            query.push(("FromStationCode", station_code.to_string()));
        }

        if let Some(station_code) = to_destination_station {
            query.push(("ToStationCode", station_code.to_string()));
        }

        completion(request_and_deserialize(
            &self.api_key,
            STATION_TO_STATION,
            Some(&query),
        ));
    }

    pub fn positions<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrainPositions, Error>) -> (),
    {
        completion(request_and_deserialize(
            &self.api_key,
            POSITIONS,
            Some(&[("contentType", "json")]),
        ));
    }

    pub fn routes<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::StandardRoutes, Error>) -> (),
    {
        completion(request_and_deserialize(
            &self.api_key,
            ROUTES,
            Some(&[("contentType", "json")]),
        ))
    }

    pub fn circuits<F>(&self, completion: F)
    where
        F: FnOnce(Result<responses::TrackCircuits, Error>) -> (),
    {
        completion(request_and_deserialize(
            &self.api_key,
            CIRCUITS,
            Some(&[("contentType", "json")]),
        ))
    }

    pub fn elevator_and_escalator_incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::ElevatorAndEscalatorIncidents, Error>) -> (),
    {
        let mut query = vec![];

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        completion(request_and_deserialize(
            &self.api_key,
            ELEVATOR_AND_ESCALATOR_INCIDENTS,
            Some(&query),
        ));
    }

    pub fn incidents<F>(&self, station: Option<StationCode>, completion: F)
    where
        F: FnOnce(Result<responses::RailIncidents, Error>) -> (),
    {
        let mut query = vec![];

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        completion(request_and_deserialize(
            &self.api_key,
            INCIDENTS,
            Some(&query),
        ));
    }
}
