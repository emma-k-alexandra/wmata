pub mod responses;
mod tests;

use crate::error::Error;
use crate::line::{responses as line_responses, LineCode};
use crate::request_and_deserialize;
use crate::station::{responses as station_responses, StationCode};
use crate::urls::URLs;
use std::str::FromStr;

pub struct Client {
    pub api_key: String,
}

impl Client {
    pub fn lines(&self) -> Result<responses::Lines, Error> {
        request_and_deserialize::<responses::Lines, [(); 0]>(
            &self.api_key,
            &URLs::Lines.to_string(),
            None,
        )
    }

    pub fn entrances(
        &self,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> Result<responses::StationEntrances, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Entrances.to_string(),
            Some(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)]),
        )
    }

    pub fn stations(&self, line: Option<LineCode>) -> Result<line_responses::Stations, Error> {
        let mut query = vec![];

        if let Some(line_code) = line {
            query.push(("LineCode", line_code.to_string()));
        }

        request_and_deserialize(&self.api_key, &URLs::Stations.to_string(), Some(&query))
    }

    pub fn station(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
    ) -> Result<station_responses::StationToStationInfos, Error> {
        let mut query = vec![];

        if let Some(station_code) = from_station {
            query.push(("FromStationCode", station_code.to_string()));
        }

        if let Some(station_code) = to_destination_station {
            query.push(("ToStationCode", station_code.to_string()));
        }

        request_and_deserialize(
            &self.api_key,
            &URLs::StationToStation.to_string(),
            Some(&query),
        )
    }

    pub fn positions(&self) -> Result<responses::TrainPositions, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Positions.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    pub fn routes(&self) -> Result<responses::StandardRoutes, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Routes.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    pub fn circuits(&self) -> Result<responses::TrackCircuits, Error> {
        request_and_deserialize(
            &self.api_key,
            &URLs::Circuits.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    pub fn elevator_and_escalator_incidents(
        &self,
        station: Option<StationCode>,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        let mut query = vec![];

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        request_and_deserialize(
            &self.api_key,
            &URLs::ElevatorAndEscalatorIncidents.to_string(),
            Some(&query),
        )
    }

    pub fn incidents(
        &self,
        station: Option<StationCode>,
    ) -> Result<responses::RailIncidents, Error> {
        let mut query = vec![];

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        request_and_deserialize(&self.api_key, &URLs::Incidents.to_string(), Some(&query))
    }
}

impl FromStr for Client {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client {
            api_key: s.to_string(),
        })
    }
}
