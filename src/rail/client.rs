pub mod responses;
mod tests;

use crate::error::Error;
use crate::rail::line::LineCode;
use crate::rail::station::StationCode;
use crate::rail::urls::URLs;
use crate::traits::{ApiKey, Fetch};
use crate::types::Empty;
use std::str::FromStr;

pub struct Client {
    pub api_key: String,
}

impl ApiKey for Client {
    fn api_key(&self) -> &str {
        &self.api_key
    }
}

// Constructor
impl Client {
    fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
        }
    }
}

// No Station or Line Codes
impl Client {
    pub fn lines(&self) -> Result<responses::Lines, Error> {
        self.fetch::<responses::Lines, Empty>(&URLs::Lines.to_string(), None)
    }

    pub fn entrances(
        &self,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> Result<responses::StationEntrances, Error> {
        self.fetch(
            &URLs::Entrances.to_string(),
            Some(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)]),
        )
    }

    pub fn positions(&self) -> Result<responses::TrainPositions, Error> {
        self.fetch(
            &URLs::Positions.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    pub fn routes(&self) -> Result<responses::StandardRoutes, Error> {
        self.fetch(&URLs::Routes.to_string(), Some(&[("contentType", "json")]))
    }

    pub fn circuits(&self) -> Result<responses::TrackCircuits, Error> {
        self.fetch(
            &URLs::Circuits.to_string(),
            Some(&[("contentType", "json")]),
        )
    }
}

// These take StationCodes
impl Client {
    pub fn station_to_station(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
    ) -> Result<responses::StationToStationInfos, Error> {
        let mut query = vec![];

        if let Some(from_station) = from_station {
            query.push(("FromStationCode", from_station.to_string()));
        }

        if let Some(to_destination_station) = to_destination_station {
            query.push(("ToStationCode", to_destination_station.to_string()));
        }

        if !query.is_empty() {
            self.fetch(&URLs::StationToStation.to_string(), Some(&query))
        } else {
            self.fetch::<responses::StationToStationInfos, Empty>(
                &URLs::StationToStation.to_string(),
                None,
            )
        }
    }

    pub fn elevator_and_escalator_incidents_at(
        &self,
        station: Option<StationCode>,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        let mut query = vec![];

        if let Some(station) = station {
            query.push(("StationCode", station.to_string()));
        }

        if !query.is_empty() {
            self.fetch(
                &URLs::ElevatorAndEscalatorIncidents.to_string(),
                Some(&query),
            )
        } else {
            self.fetch::<responses::ElevatorAndEscalatorIncidents, Empty>(
                &URLs::ElevatorAndEscalatorIncidents.to_string(),
                None,
            )
        }
    }

    pub fn incidents_at(
        &self,
        station: Option<StationCode>,
    ) -> Result<responses::RailIncidents, Error> {
        let mut query = vec![];

        if let Some(station) = station {
            query.push(("StationCode", station.to_string()));
        }

        self.fetch(&URLs::Incidents.to_string(), Some(&query))
    }

    pub fn next_trains(
        &self,
        station_code: StationCode,
    ) -> Result<responses::RailPredictions, Error> {
        self.fetch::<responses::RailPredictions, Empty>(
            &[URLs::NextTrains.to_string(), station_code.to_string()].join("/"),
            None,
        )
    }

    pub fn station_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationInformation, Error> {
        self.fetch(
            &URLs::Information.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    pub fn parking_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationsParking, Error> {
        self.fetch(
            &URLs::ParkingInformation.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    pub fn path_from(
        &self,
        from_station: StationCode,
        to_station: StationCode,
    ) -> Result<responses::PathBetweenStations, Error> {
        self.fetch(
            &URLs::Path.to_string(),
            Some(&[
                ("FromStationCode", from_station.to_string()),
                ("ToStationCode", to_station.to_string()),
            ]),
        )
    }

    pub fn timings(&self, station_code: StationCode) -> Result<responses::StationTimings, Error> {
        self.fetch(
            &URLs::Timings.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }
}

// These take LineCodes
impl Client {
    pub fn stations_on(&self, line: Option<LineCode>) -> Result<responses::Stations, Error> {
        let mut query = vec![];

        if let Some(line) = line {
            query.push(("LineCode", line.to_string()));
        }

        if !query.is_empty() {
            self.fetch(&URLs::Stations.to_string(), Some(&query))
        } else {
            self.fetch::<responses::Stations, Empty>(&URLs::Stations.to_string(), None)
        }
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
