//! MetroRail client. Contains the client for fetching data from
//! the WMATA API and data structures returned from those endpoint calls.
pub mod responses;
mod tests;

use crate::error::Error;
use crate::rail::line::LineCode;
use crate::rail::station::StationCode;
use crate::rail::traits::{NeedsLineCode, NeedsStationCode};
use crate::rail::urls::URLs;
use crate::traits::Fetch;
use crate::types::{RadiusAtLatLong, Request as WMATARequest};
use std::str::FromStr;

/// MetroRail client. Used to fetch MetroRail-related information from the WMATA API.
pub struct Client {
    /// The WMATA API key to use for all requests routed through this client.
    pub key: String,
}


impl Fetch for Client {}

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
        self.fetch::<responses::Lines>(WMATARequest::new(
            &self.key,
            &URLs::Lines.to_string(),
            None,
        ))
    }

    /// A list of nearby station entrances based on latitude, longitude, and radius (meters).
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330f?)
    ///
    /// # Example
    /// ```
    /// use wmata::{RailClient, RadiusAtLatLong};
    ///
    /// let client = RailClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.entrances(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426)).is_ok());
    /// ```
    pub fn entrances(
        &self,
        radius_at_lat_long: RadiusAtLatLong,
    ) -> Result<responses::StationEntrances, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Entrances.to_string(),
            Some(radius_at_lat_long.to_query()),
        ))
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
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Positions.to_string(),
            Some(vec![("contentType".to_string(), "json".to_string())]),
        ))
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
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Routes.to_string(),
            Some(vec![("contentType".to_string(), "json".to_string())]),
        ))
    }

    pub fn circuits(&self) -> Result<responses::TrackCircuits, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Circuits.to_string(),
            Some(vec![("contentType".to_string(), "json".to_string())]),
        ))
    }
}

impl NeedsStationCode for Client {}

// Overwriting NeedsStationCode
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
        <Self as NeedsStationCode>::station_to_station(
            &self,
            from_station,
            to_destination_station,
            &self.key,
        )
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
        <Self as NeedsStationCode>::elevator_and_escalator_incidents_at(&self, station, &self.key)
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
        <Self as NeedsStationCode>::incidents_at(&self, station, &self.key)
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
        <Self as NeedsStationCode>::next_trains(&self, station_code, &self.key)
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
        <Self as NeedsStationCode>::station_information(&self, station_code, &self.key)
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
        <Self as NeedsStationCode>::parking_information(&self, station_code, &self.key)
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
        <Self as NeedsStationCode>::path_from(&self, from_station, to_station, &self.key)
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
        <Self as NeedsStationCode>::timings(&self, station_code, &self.key)
    }
}

impl NeedsLineCode for Client {}

/// Overwriting NeedsLineCode methods
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
        <Self as NeedsLineCode>::stations_on(&self, line, &self.key)
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
