//! MetroRail client. Contains the client for fetching data from
//! the WMATA API and data structures returned from those endpoint calls.
pub mod responses;
mod tests;

use crate::error::Error;
use crate::rail::line::LineCode;
use crate::rail::station::StationCode;
use crate::rail::urls::URLs;
use crate::traits::{ApiKey, Fetch};
use crate::types::Empty;
use std::str::FromStr;

/// MetroRail client. Used to fetch MetroRail-related information from the WMATA API.
pub struct Client {
    /// The WMATA API key to use for all requests routed through this client.
    pub key: String,
}

impl ApiKey for Client {
    /// Returns the API key contained in this Client.
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn api_key(&self) -> &str {
        &self.key
    }
}

// Constructor
impl Client {
    /// Constructor for the MetroRail client.
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    // Again, not actually dead code
    #[allow(dead_code)]
    pub fn new(api_key: &str) -> Self {
        Client {
            key: api_key.to_string(),
        }
    }
}

// No Station or Line Codes
impl Client {
    /// Basic information on all MetroRail lines.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330c)
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.lines().is_ok());
    /// ```
    pub fn lines(&self) -> Result<responses::Lines, Error> {
        self.fetch::<responses::Lines, Empty>(&URLs::Lines.to_string(), None)
    }

    /// A list of nearby station entrances based on latitude, longitude, and radius (meters).
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330f?)
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.entrances(38.8817596, -77.0166426, 1000.0).is_ok());
    /// ```
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

    /// Uniquely identifiable trains in service and what track circuits they currently occupy.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5763fa6ff91823096cac1057/operations/5763fb35f91823096cac1058)
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.positions().is_ok());
    /// ```
    pub fn positions(&self) -> Result<responses::TrainPositions, Error> {
        self.fetch(
            &URLs::Positions.to_string(),
            Some(&[("contentType", "json")]),
        )
    }

    /// Returns an ordered list of mostly revenue (and some lead) track circuits, arranged by line and track number.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5763fa6ff91823096cac1057/operations/57641afc031f59363c586dca?)
    ///
    /// # Example
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.routes().is_ok());
    /// ```
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
    /// Distance, fare information, and estimated travel time between any two stations, including those on different lines.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe3313?)
    ///
    /// # Example
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.station_to_station(Some(StationCode::A01), Some(StationCode::A02)).is_ok());
    /// ```
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

    /// List of reported elevator and escalator outages at a given station.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763641281d83086473f232/operations/54763641281d830c946a3d76?)
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.elevator_and_escalator_incidents_at(Some(StationCode::A01)).is_ok());
    /// ```
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

    /// Reported rail incidents (significant disruptions and delays to normal service)
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.incidents_at(Some(StationCode::A01)).is_ok());
    /// ```
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

    /// Next train arrivals for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.next_trains(StationCode::A01).is_ok());
    /// ```
    pub fn next_trains(
        &self,
        station_code: StationCode,
    ) -> Result<responses::RailPredictions, Error> {
        self.fetch::<responses::RailPredictions, Empty>(
            &[URLs::NextTrains.to_string(), station_code.to_string()].join("/"),
            None,
        )
    }

    /// Location and address information at the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.station_information(StationCode::A01).is_ok());
    /// ```
    pub fn station_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationInformation, Error> {
        self.fetch(
            &URLs::Information.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    /// Parking information for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.parking_information(StationCode::A01).is_ok());
    /// ```
    pub fn parking_information(
        &self,
        station_code: StationCode,
    ) -> Result<responses::StationsParking, Error> {
        self.fetch(
            &URLs::ParkingInformation.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }

    /// Set of ordered stations and distances between two stations on the **same line**.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path_from(StationCode::A01, StationCode::A02).is_ok());
    /// ```
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

    /// Opening and scheduled first/last train times for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, StationCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.timings(StationCode::A01).is_ok());
    /// ```
    pub fn timings(&self, station_code: StationCode) -> Result<responses::StationTimings, Error> {
        self.fetch(
            &URLs::Timings.to_string(),
            Some(&[("StationCode", station_code.to_string())]),
        )
    }
}

// These take LineCodes
impl Client {
    /// Station location and address information for all stations on the given line.
    ///
    /// # Examples
    /// ```
    /// use wmata::{RailClient, LineCode};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stations_on(Some(LineCode::Red)).is_ok());
    /// ```
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

    /// Converts a string into a MetroRail Client.
    ///
    /// # Examples
    /// ```
    /// use wmata::RailClient;
    ///
    /// let client: RailClient = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    ///
    /// assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client { key: s.to_string() })
    }
}
