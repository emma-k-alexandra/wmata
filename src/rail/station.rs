//! WMATA-defined codes for each MetroRail station.
use crate::{
    error::Error,
    rail::{client::responses, traits::NeedsStation},
    requests::Fetch,
};
use serde::{
    de::{Deserializer, Error as SerdeError},
    Deserialize,
};
use std::{error, fmt, str::FromStr};

/// Every MetroRail station code as defined by WMATA.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Station {
    A01,
    A02,
    A03,
    A04,
    A05,
    A06,
    A07,
    A08,
    A09,
    A10,
    A11,
    A12,
    A13,
    A14,
    A15,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11,
    B35,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C12,
    C13,
    C14,
    C15,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    F01,
    F02,
    F03,
    F04,
    F05,
    F06,
    F07,
    F08,
    F09,
    F10,
    F11,
    G01,
    G02,
    G03,
    G04,
    G05,
    J02,
    J03,
    K01,
    K02,
    K03,
    K04,
    K05,
    K06,
    K07,
    K08,
    N01,
    N02,
    N03,
    N04,
    N06,
}

impl Fetch for Station {}

impl NeedsStation for Station {}

impl Station {
    /// Distance, fare information, and estimated travel time between this and another optional station, including those on different lines.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476364f031f590f38092507/operations/5476364f031f5909e4fe3313?)
    ///
    /// # Example
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.to_station(Some(Station::A02), "9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn to_station(
        self,
        destination_station: Option<Station>,
        api_key: &str,
    ) -> Result<responses::StationToStationInfos, Error> {
        self.station_to_station(Some(self), destination_station, api_key)
    }

    // List of reported elevator and escalator outages at this station.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763641281d83086473f232/operations/54763641281d830c946a3d76?)
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.elevator_and_escalator_incidents("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn elevator_and_escalator_incidents(
        self,
        api_key: &str,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        self.elevator_and_escalator_incidents_at(Some(self), api_key)
    }

    /// Reported rail incidents (significant disruptions and delays to normal service) at this station
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.incidents("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn incidents(self, api_key: &str) -> Result<responses::RailIncidents, Error> {
        self.incidents_at(Some(self), api_key)
    }

    /// Next train arrivals for this station
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.next_trains("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn next_trains(self, api_key: &str) -> Result<responses::RailPredictions, Error> {
        <Self as NeedsStation>::next_trains(&self, self, api_key)
    }

    /// Location and address information at this station
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.information("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn information(self, api_key: &str) -> Result<responses::StationInformation, Error> {
        self.station_information(self, api_key)
    }

    /// Parking information for this station
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.parking_information("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn parking_information(self, api_key: &str) -> Result<responses::StationsParking, Error> {
        <Self as NeedsStation>::parking_information(&self, self, api_key)
    }

    /// Set of ordered stations and distances between this station and another on the **same line**.
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.path_to(Station::A02, "9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn path_to(
        self,
        destination_station: Station,
        api_key: &str,
    ) -> Result<responses::PathBetweenStations, Error> {
        self.path_from(self, destination_station, api_key)
    }

    /// Opening and scheduled first/last train times for this station.
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// assert!(Station::A01.timings("9e38c3eab34c4e6c990828002828f5ed").is_ok());
    /// ```
    pub fn timings(self, api_key: &str) -> Result<responses::StationTimings, Error> {
        <Self as NeedsStation>::timings(&self, self, api_key)
    }
}

impl<'de> Deserialize<'de> for Station {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let station = String::deserialize(deserializer)?;

        Station::from_str(&station)
            .map_err(|_| SerdeError::custom("String provided is not a Station code"))
    }
}

impl ToString for Station {
    fn to_string(&self) -> String {
        match self {
            Station::A01 => "A01".to_string(),
            Station::A02 => "A02".to_string(),
            Station::A03 => "A03".to_string(),
            Station::A04 => "A04".to_string(),
            Station::A05 => "A05".to_string(),
            Station::A06 => "A06".to_string(),
            Station::A07 => "A07".to_string(),
            Station::A08 => "A08".to_string(),
            Station::A09 => "A09".to_string(),
            Station::A10 => "A10".to_string(),
            Station::A11 => "A11".to_string(),
            Station::A12 => "A12".to_string(),
            Station::A13 => "A13".to_string(),
            Station::A14 => "A14".to_string(),
            Station::A15 => "A15".to_string(),
            Station::B01 => "B01".to_string(),
            Station::B02 => "B02".to_string(),
            Station::B03 => "B03".to_string(),
            Station::B04 => "B04".to_string(),
            Station::B05 => "B05".to_string(),
            Station::B06 => "B06".to_string(),
            Station::B07 => "B07".to_string(),
            Station::B08 => "B08".to_string(),
            Station::B09 => "B09".to_string(),
            Station::B10 => "B10".to_string(),
            Station::B11 => "B11".to_string(),
            Station::B35 => "B35".to_string(),
            Station::C01 => "C01".to_string(),
            Station::C02 => "C02".to_string(),
            Station::C03 => "C03".to_string(),
            Station::C04 => "C04".to_string(),
            Station::C05 => "C05".to_string(),
            Station::C06 => "C06".to_string(),
            Station::C07 => "C07".to_string(),
            Station::C08 => "C08".to_string(),
            Station::C09 => "C09".to_string(),
            Station::C10 => "C10".to_string(),
            Station::C12 => "C12".to_string(),
            Station::C13 => "C13".to_string(),
            Station::C14 => "C14".to_string(),
            Station::C15 => "C15".to_string(),
            Station::D01 => "D01".to_string(),
            Station::D02 => "D02".to_string(),
            Station::D03 => "D03".to_string(),
            Station::D04 => "D04".to_string(),
            Station::D05 => "D05".to_string(),
            Station::D06 => "D06".to_string(),
            Station::D07 => "D07".to_string(),
            Station::D08 => "D08".to_string(),
            Station::D09 => "D09".to_string(),
            Station::D10 => "D10".to_string(),
            Station::D11 => "D11".to_string(),
            Station::D12 => "D12".to_string(),
            Station::D13 => "D13".to_string(),
            Station::E01 => "E01".to_string(),
            Station::E02 => "E02".to_string(),
            Station::E03 => "E03".to_string(),
            Station::E04 => "E04".to_string(),
            Station::E05 => "E05".to_string(),
            Station::E06 => "E06".to_string(),
            Station::E07 => "E07".to_string(),
            Station::E08 => "E08".to_string(),
            Station::E09 => "E09".to_string(),
            Station::E10 => "E10".to_string(),
            Station::F01 => "F01".to_string(),
            Station::F02 => "F02".to_string(),
            Station::F03 => "F03".to_string(),
            Station::F04 => "F04".to_string(),
            Station::F05 => "F05".to_string(),
            Station::F06 => "F06".to_string(),
            Station::F07 => "F07".to_string(),
            Station::F08 => "F08".to_string(),
            Station::F09 => "F09".to_string(),
            Station::F10 => "F10".to_string(),
            Station::F11 => "F11".to_string(),
            Station::G01 => "G01".to_string(),
            Station::G02 => "G02".to_string(),
            Station::G03 => "G03".to_string(),
            Station::G04 => "G04".to_string(),
            Station::G05 => "G05".to_string(),
            Station::J02 => "J02".to_string(),
            Station::J03 => "J03".to_string(),
            Station::K01 => "K01".to_string(),
            Station::K02 => "K02".to_string(),
            Station::K03 => "K03".to_string(),
            Station::K04 => "K04".to_string(),
            Station::K05 => "K05".to_string(),
            Station::K06 => "K06".to_string(),
            Station::K07 => "K07".to_string(),
            Station::K08 => "K08".to_string(),
            Station::N01 => "N01".to_string(),
            Station::N02 => "N02".to_string(),
            Station::N03 => "N03".to_string(),
            Station::N04 => "N04".to_string(),
            Station::N06 => "N06".to_string(),
        }
    }
}

impl FromStr for Station {
    type Err = StringIsNotStationError;

    /// Converts a string to a [`Station`].
    ///
    /// # Examples
    /// ```
    /// use wmata::Station;
    ///
    /// let station_code: Station = "A01".parse().unwrap();
    ///
    /// assert_eq!(Station::A01, station_code);
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A01" => Ok(Station::A01),
            "A02" => Ok(Station::A02),
            "A03" => Ok(Station::A03),
            "A04" => Ok(Station::A04),
            "A05" => Ok(Station::A05),
            "A06" => Ok(Station::A06),
            "A07" => Ok(Station::A07),
            "A08" => Ok(Station::A08),
            "A09" => Ok(Station::A09),
            "A10" => Ok(Station::A10),
            "A11" => Ok(Station::A11),
            "A12" => Ok(Station::A12),
            "A13" => Ok(Station::A13),
            "A14" => Ok(Station::A14),
            "A15" => Ok(Station::A15),
            "B01" => Ok(Station::B01),
            "B02" => Ok(Station::B02),
            "B03" => Ok(Station::B03),
            "B04" => Ok(Station::B04),
            "B05" => Ok(Station::B05),
            "B06" => Ok(Station::B06),
            "B07" => Ok(Station::B07),
            "B08" => Ok(Station::B08),
            "B09" => Ok(Station::B09),
            "B10" => Ok(Station::B10),
            "B11" => Ok(Station::B11),
            "B35" => Ok(Station::B35),
            "C01" => Ok(Station::C01),
            "C02" => Ok(Station::C02),
            "C03" => Ok(Station::C03),
            "C04" => Ok(Station::C04),
            "C05" => Ok(Station::C05),
            "C06" => Ok(Station::C06),
            "C07" => Ok(Station::C07),
            "C08" => Ok(Station::C08),
            "C09" => Ok(Station::C09),
            "C10" => Ok(Station::C10),
            "C12" => Ok(Station::C12),
            "C13" => Ok(Station::C13),
            "C14" => Ok(Station::C14),
            "C15" => Ok(Station::C15),
            "D01" => Ok(Station::D01),
            "D02" => Ok(Station::D02),
            "D03" => Ok(Station::D03),
            "D04" => Ok(Station::D04),
            "D05" => Ok(Station::D05),
            "D06" => Ok(Station::D06),
            "D07" => Ok(Station::D07),
            "D08" => Ok(Station::D08),
            "D09" => Ok(Station::D09),
            "D10" => Ok(Station::D10),
            "D11" => Ok(Station::D11),
            "D12" => Ok(Station::D12),
            "D13" => Ok(Station::D13),
            "E01" => Ok(Station::E01),
            "E02" => Ok(Station::E02),
            "E03" => Ok(Station::E03),
            "E04" => Ok(Station::E04),
            "E05" => Ok(Station::E05),
            "E06" => Ok(Station::E06),
            "E07" => Ok(Station::E07),
            "E08" => Ok(Station::E08),
            "E09" => Ok(Station::E09),
            "E10" => Ok(Station::E10),
            "F01" => Ok(Station::F01),
            "F02" => Ok(Station::F02),
            "F03" => Ok(Station::F03),
            "F04" => Ok(Station::F04),
            "F05" => Ok(Station::F05),
            "F06" => Ok(Station::F06),
            "F07" => Ok(Station::F07),
            "F08" => Ok(Station::F08),
            "F09" => Ok(Station::F09),
            "F10" => Ok(Station::F10),
            "F11" => Ok(Station::F11),
            "G01" => Ok(Station::G01),
            "G02" => Ok(Station::G02),
            "G03" => Ok(Station::G03),
            "G04" => Ok(Station::G04),
            "G05" => Ok(Station::G05),
            "J02" => Ok(Station::J02),
            "J03" => Ok(Station::J03),
            "K01" => Ok(Station::K01),
            "K02" => Ok(Station::K02),
            "K03" => Ok(Station::K03),
            "K04" => Ok(Station::K04),
            "K05" => Ok(Station::K05),
            "K06" => Ok(Station::K06),
            "K07" => Ok(Station::K07),
            "K08" => Ok(Station::K08),
            "N01" => Ok(Station::N01),
            "N02" => Ok(Station::N02),
            "N03" => Ok(Station::N03),
            "N04" => Ok(Station::N04),
            "N06" => Ok(Station::N06),
            _ => Err(StringIsNotStationError),
        }
    }
}

pub fn empty_or_station<'de, D>(deserializer: D) -> Result<Option<Station>, D::Error>
where
    D: Deserializer<'de>,
{
    match Station::deserialize(deserializer) {
        Ok(station) => Ok(Some(station)),
        Err(_) => Ok(None),
    }
}

/// An error incidating that the provided string is not a WMATA Station Code.
#[derive(Debug, Clone)]
pub struct StringIsNotStationError;

impl fmt::Display for StringIsNotStationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Provided string is not a valid station code.")
    }
}

impl error::Error for StringIsNotStationError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
}
