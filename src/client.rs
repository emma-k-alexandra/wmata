pub mod responses;
mod tests;

use crate::error::Error;
use crate::line::LineCode;
use crate::station::StationCode;
use crate::urls::URLs;
use std::str::FromStr;

use reqwest;
use serde::{de::DeserializeOwned, Serialize};
use serde_json;

pub struct Client {
    pub api_key: String,
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
        self.request_and_deserialize::<responses::Lines, [(); 0]>(&URLs::Lines.to_string(), None)
    }

    pub fn entrances(
        &self,
        latitude: f64,
        longitude: f64,
        radius: f64,
    ) -> Result<responses::StationEntrances, Error> {
        self.request_and_deserialize(
            &URLs::Entrances.to_string(),
            Some(&[("Lat", latitude), ("Lon", longitude), ("Radius", radius)]),
        )
    }

    pub fn positions(&self) -> Result<responses::TrainPositions, Error> {
        self.request_and_deserialize(
            &URLs::Positions.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    pub fn routes(&self) -> Result<responses::StandardRoutes, Error> {
        self.request_and_deserialize(&URLs::Routes.to_string(), Some(&[("contentType", "json")]))
    }

    pub fn circuits(&self) -> Result<responses::TrackCircuits, Error> {
        self.request_and_deserialize(
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

        if let Some(station_code) = from_station {
            query.push(("FromStationCode", station_code.to_string()));
        }

        if let Some(station_code) = to_destination_station {
            query.push(("ToStationCode", station_code.to_string()));
        }

        if query.len() > 0 {
            self.request_and_deserialize(&URLs::StationToStation.to_string(), Some(&query))
        } else {
            self.request_and_deserialize::<responses::StationToStationInfos, [(); 0]>(
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

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        if query.len() > 0 {
            self.request_and_deserialize(
                &URLs::ElevatorAndEscalatorIncidents.to_string(),
                Some(&query),
            )
        } else {
            self.request_and_deserialize::<responses::ElevatorAndEscalatorIncidents, [(); 0]>(
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

        if let Some(station_code) = station {
            query.push(("StationCode", station_code.to_string()));
        }

        self.request_and_deserialize(&URLs::Incidents.to_string(), Some(&query))
    }

    pub fn next_trains(
        &self,
        station_code: StationCode,
    ) -> Result<responses::RailPredictions, Error> {
        self.request_and_deserialize::<responses::RailPredictions, [(); 0]>(
            &[URLs::NextTrains.to_string(), station_code.to_string()].join("/"),
            None,
        )
    }

    pub fn station_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationInformation, Error> {
        self.request_and_deserialize(
            &URLs::Information.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    pub fn parking_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationsParking, Error> {
        self.request_and_deserialize(
            &URLs::ParkingInformation.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    pub fn path_from(
        &self,
        from_station: StationCode,
        to_station: StationCode,
    ) -> Result<responses::PathBetweenStations, Error> {
        self.request_and_deserialize(
            &URLs::Path.to_string(),
            Some(&[
                ("FromStationCode", from_station.to_string()),
                ("ToStationCode", to_station.to_string()),
            ]),
        )
    }

    pub fn timings(&self, station_code: StationCode) -> Result<responses::StationTimings, Error> {
        self.request_and_deserialize(
            &URLs::Timings.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }
}

// These take LineCodes
impl Client {
    pub fn stations_on(&self, line: Option<LineCode>) -> Result<responses::Stations, Error> {
        let mut query = vec![];

        if let Some(line_code) = line {
            query.push(("LineCode", line_code.to_string()));
        }

        if query.len() > 0 {
            self.request_and_deserialize(&URLs::Stations.to_string(), Some(&query))
        } else {
            self.request_and_deserialize::<responses::Stations, [(); 0]>(
                &URLs::Stations.to_string(),
                None,
            )
        }
    }
}

// Internal helper methods
impl Client {
    fn request_and_deserialize<T, U>(&self, path: &str, query: Option<U>) -> Result<T, Error>
    where
        T: DeserializeOwned,
        U: Serialize + Sized,
    {
        fn deserialize<T>(response: String) -> Result<T, Error>
        where
            T: DeserializeOwned,
        {
            serde_json::from_str::<T>(&response).or_else(|_| {
                match serde_json::from_str::<responses::Error>(&response) {
                    Ok(json) => Err(Error::new(json.message.to_string())),
                    Err(err) => Err(Error::new(err.to_string())),
                }
            })
        }

        let mut request = reqwest::Client::new().get(path);

        if let Some(some_query) = query {
            request = request.query(&some_query)
        }

        request
            .header("api_key", &self.api_key)
            .send()
            .and_then(|mut response| response.text())
            .map_err(|err| Error::new(err.to_string()))
            .and_then(deserialize)
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
