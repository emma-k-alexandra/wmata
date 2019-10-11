//! MetroRail client. Contains the client for fetching data from
//! the WMATA API and data structures returned from those endpoint calls.
pub mod responses;
mod tests;

use crate::error::Error;
use crate::rail::line::Line;
use crate::rail::station::Station;
use crate::rail::traits::{NeedsLine, NeedsStation};
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
    /// use wmata::MetroRail;
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
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
    /// use wmata::MetroRail;
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.lines().is_ok());
    /// ```
    pub fn lines(&self) -> Result<responses::Lines, Error> {
        self.fetch::<responses::Lines>(WMATARequest::new(&self.key, &URLs::Lines.to_string(), None))
    }

    /// A list of nearby station entrances based on latitude, longitude, and radius (meters).
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330f?)
    ///
    /// # Example
    /// ```
    /// use wmata::{MetroRail, RadiusAtLatLong};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.entrances(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426)).is_ok());
    /// ```
    pub fn entrances(
        &self,
        radius_at_lat_long: RadiusAtLatLong,
    ) -> Result<responses::StationEntrances, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Entrances.to_string(),
            Some(
                radius_at_lat_long
                    .to_query()
                    .iter()
                    .map(|(key, value)| (key.as_str(), value.clone()))
                    .collect(),
            ),
        ))
    }

    /// Uniquely identifiable trains in service and what track circuits they currently occupy.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5763fa6ff91823096cac1057/operations/5763fb35f91823096cac1058)
    ///
    /// # Example
    /// ```
    /// use wmata::MetroRail;
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.positions().is_ok());
    /// ```
    pub fn positions(&self) -> Result<responses::TrainPositions, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Positions.to_string(),
            Some(vec![("contentType", "json".to_string())]),
        ))
    }

    /// Returns an ordered list of mostly revenue (and some lead) track circuits, arranged by line and track number.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5763fa6ff91823096cac1057/operations/57641afc031f59363c586dca?)
    ///
    /// # Example
    /// ```
    /// use wmata::MetroRail;
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.routes().is_ok());
    /// ```
    pub fn routes(&self) -> Result<responses::StandardRoutes, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Routes.to_string(),
            Some(vec![("contentType", "json".to_string())]),
        ))
    }

    pub fn circuits(&self) -> Result<responses::TrackCircuits, Error> {
        self.fetch(WMATARequest::new(
            &self.key,
            &URLs::Circuits.to_string(),
            Some(vec![("contentType", "json".to_string())]),
        ))
    }
}

impl NeedsStation for Client {}

// Overwriting NeedsStation
impl Client {
    /// Distance, fare information, and estimated travel time between any two stations, including those on different lines.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe3313?)
    ///
    /// # Example
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.station_to_station(Some(Station::A01), Some(Station::A02)).is_ok());
    /// ```
    pub fn station_to_station(
        &self,
        from_station: Option<Station>,
        to_destination_station: Option<Station>,
    ) -> Result<responses::StationToStationInfos, Error> {
        <Self as NeedsStation>::station_to_station(
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
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.elevator_and_escalator_incidents_at(Some(Station::A01)).is_ok());
    /// ```
    pub fn elevator_and_escalator_incidents_at(
        &self,
        station: Option<Station>,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        <Self as NeedsStation>::elevator_and_escalator_incidents_at(&self, station, &self.key)
    }

    /// Reported rail incidents (significant disruptions and delays to normal service)
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.incidents_at(Some(Station::A01)).is_ok());
    /// ```
    pub fn incidents_at(
        &self,
        station: Option<Station>,
    ) -> Result<responses::RailIncidents, Error> {
        <Self as NeedsStation>::incidents_at(&self, station, &self.key)
    }

    /// Next train arrivals for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.next_trains(Station::A01).is_ok());
    /// ```
    pub fn next_trains(&self, station_code: Station) -> Result<responses::RailPredictions, Error> {
        <Self as NeedsStation>::next_trains(&self, station_code, &self.key)
    }

    /// Location and address information at the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.station_information(Station::A01).is_ok());
    /// ```
    pub fn station_information(
        &self,
        station_code: Station,
    ) -> Result<responses::StationInformation, Error> {
        <Self as NeedsStation>::station_information(&self, station_code, &self.key)
    }

    /// Parking information for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.parking_information(Station::A01).is_ok());
    /// ```
    pub fn parking_information(
        &self,
        station_code: Station,
    ) -> Result<responses::StationsParking, Error> {
        <Self as NeedsStation>::parking_information(&self, station_code, &self.key)
    }

    /// Set of ordered stations and distances between two stations on the **same line**.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path_from(Station::A01, Station::A02).is_ok());
    /// ```
    pub fn path_from(
        &self,
        from_station: Station,
        to_station: Station,
    ) -> Result<responses::PathBetweenStations, Error> {
        <Self as NeedsStation>::path_from(&self, from_station, to_station, &self.key)
    }

    /// Opening and scheduled first/last train times for the given station.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Station};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.timings(Station::A01).is_ok());
    /// ```
    pub fn timings(&self, station_code: Station) -> Result<responses::StationTimings, Error> {
        <Self as NeedsStation>::timings(&self, station_code, &self.key)
    }
}

impl NeedsLine for Client {}

/// Overwriting NeedsLine methods
impl Client {
    /// Station location and address information for all stations on the given line.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe330c)
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroRail, Line};
    ///
    /// let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stations_on(Some(Line::Red)).is_ok());
    /// ```
    pub fn stations_on(&self, line: Option<Line>) -> Result<responses::Stations, Error> {
        <Self as NeedsLine>::stations_on(&self, line, &self.key)
    }
}

impl FromStr for Client {
    type Err = Error;

    /// Converts a string into a MetroRail Client.
    ///
    /// # Examples
    /// ```
    /// use wmata::MetroRail;
    ///
    /// let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    ///
    /// assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client { key: s.to_string() })
    }
}
