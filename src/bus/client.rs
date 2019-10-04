//! MetroBus client. Contains the client for fetching data from
//! the WMATA API and data structures returned from those endpoint calls.
pub mod responses;
mod tests;

use crate::bus::route::RouteID;
use crate::bus::urls::URLs;
use crate::error::Error;
use crate::traits::{ApiKey, Fetch};
use crate::types::Empty;
use std::str::FromStr;

/// MetroBus client. Used to fetch MetroBus-related information from the WMATA API.
pub struct Client {
    /// The WMATA API key to use for all requests routed through this client.
    pub key: String,
}

impl ApiKey for Client {
    /// Returns the API key contained in this Client.
    ///
    /// # Example
    /// ```
    /// use wmata::{BusClient, traits::ApiKey};
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    ///
    /// assert_eq!(client.api_key(), "9e38c3eab34c4e6c990828002828f5ed");
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
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    // This isn't actually dead code,
    // but the compiler is very angry about it
    #[allow(dead_code)]
    pub fn new(api_key: &str) -> Self {
        Client {
            key: api_key.to_string(),
        }
    }
}

// These don't take Route IDs or Stop IDs
impl Client {
    /// List of all bus route variants.
    ///
    /// # Examples
    /// ```
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.routes().is_ok());
    /// ```
    pub fn routes(&self) -> Result<responses::Routes, Error> {
        self.fetch::<responses::Routes, Empty>(&URLs::Routes.to_string(), None)
    }

    /// Nearby bus stops based on latitude, longitude, and radius.
    ///
    /// # Examples
    /// ```
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stops(Some(38.8817596), Some(-77.0166426), Some(1000)).is_ok());
    /// ```
    pub fn stops(
        &self,
        latitude: Option<f64>,
        longitude: Option<f64>,
        radius: Option<u32>,
    ) -> Result<responses::Stops, Error> {
        let mut query = vec![];

        if let Some(latitude) = latitude {
            query.push(("Lat", latitude.to_string()));
        }

        if let Some(longitude) = longitude {
            query.push(("Lon", longitude.to_string()));
        }

        if let Some(radius) = radius {
            query.push(("Radius", radius.to_string()));
        }

        if !query.is_empty() {
            self.fetch(&URLs::Stops.to_string(), Some(&query))
        } else {
            self.fetch::<responses::Stops, Empty>(&URLs::Stops.to_string(), None)
        }
    }
}

// These take RouteIDs
impl Client {
    /// Bus positions for the given route around a given lat/long.
    ///
    /// # Example
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.positions_along(
    ///     Some(RouteID::A2),
    ///     Some(38.8817596),
    ///     Some(-77.0166426),
    ///     Some(1000)
    /// ).is_ok());
    /// ```
    pub fn positions_along(
        &self,
        route: Option<RouteID>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        radius: Option<u32>,
    ) -> Result<responses::BusPositions, Error> {
        let mut query = vec![];

        if let Some(route) = route {
            query.push(("RouteID", route.to_string()));
        }

        if let Some(latitude) = latitude {
            query.push(("Lat", latitude.to_string()));
        }

        if let Some(longitude) = longitude {
            query.push(("Lon", longitude.to_string()));
        }

        if let Some(radius) = radius {
            query.push(("Radius", radius.to_string()));
        }

        if !query.is_empty() {
            self.fetch(&URLs::Positions.to_string(), Some(&query))
        } else {
            self.fetch::<responses::BusPositions, Empty>(&URLs::Positions.to_string(), None)
        }
    }

    /// Reported bus incidents/delays for a given route.
    ///
    /// # Examples
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.incidents_along(Some(RouteID::A2)).is_ok());
    /// ```
    pub fn incidents_along(&self, route: Option<RouteID>) -> Result<responses::Incidents, Error> {
        let mut query = vec![];

        if let Some(route) = route {
            query.push(("Route", route.to_string()));
        }

        if !query.is_empty() {
            self.fetch(&URLs::Incidents.to_string(), Some(&query))
        } else {
            self.fetch::<responses::Incidents, Empty>(&URLs::Incidents.to_string(), None)
        }
    }

    /// For an optional given date, returns the set of ordered latitude/longitude
    /// points along a route variant along with the list of stops served.
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Examples
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path(RouteID::A2, None).is_ok());
    /// ```
    /// With a date
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path(RouteID::A2, Some("2019-10-02")).is_ok());
    /// ```
    pub fn path(
        &self,
        route: RouteID,
        date: Option<&str>,
    ) -> Result<responses::PathDetails, Error> {
        let mut query = vec![("RouteID", route.to_string())];

        if let Some(date) = date {
            query.push(("Date", date.to_string()));
        }

        self.fetch(&URLs::PathDetails.to_string(), Some(&query))
    }

    /// Schedules for a given route variant for an optional given date.
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Variations
    /// Whether or not to include variations if a base route is specified in RouteID.
    /// For example, if B30 is specified and IncludingVariations is set to true,
    /// data for all variations of B30 such as B30v1, B30v2, etc. will be returned.
    ///
    /// # Examples
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.route_schedule(RouteID::A2, None, false).is_ok());
    /// ```
    ///
    /// with date and variations
    /// ```
    /// use wmata::{BusClient, RouteID};
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.route_schedule(RouteID::A2, Some("2019-10-02"), true).is_ok());
    /// ```
    pub fn route_schedule(
        &self,
        route: RouteID,
        date: Option<&str>,
        including_variations: bool,
    ) -> Result<responses::RouteSchedule, Error> {
        let mut query = vec![("RouteID", route.to_string())];

        if let Some(date) = date {
            query.push(("Date", date.to_string()));
        }

        if including_variations {
            query.push(("IncludingVariations", including_variations.to_string()));
        }

        self.fetch(&URLs::RouteSchedule.to_string(), Some(&query))
    }
}

// These take Stop IDs
impl Client {
    /// Next bus arrivals at a given stop.
    ///
    /// # Examples
    /// ```
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.next_buses("1001195").is_ok());
    /// ```
    pub fn next_buses(&self, stop_id: &str) -> Result<responses::Predictions, Error> {
        self.fetch(&URLs::NextBuses.to_string(), Some(&[("StopID", stop_id)]))
    }

    /// Buses scheduled at a stop for an optional given date.
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Examples
    /// ```
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stop_schedule("1001195", None).is_ok());
    /// ```
    ///
    /// with date
    /// ```
    /// use wmata::BusClient;
    ///
    /// let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stop_schedule("1001195", Some("2019-10-02")).is_ok());
    /// ```
    pub fn stop_schedule(
        &self,
        stop_id: &str,
        date: Option<&str>,
    ) -> Result<responses::StopSchedule, Error> {
        let mut query = vec![("StopID", stop_id)];

        if let Some(date) = date {
            query.push(("Date", date));
        }

        self.fetch(&URLs::StopSchedule.to_string(), Some(&query))
    }
}

impl FromStr for Client {
    type Err = Error;

    /// Converts a string into a MetroBus Client.
    ///
    /// # Examples
    /// ```
    /// use wmata::BusClient;
    /// let client: BusClient = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    ///
    /// assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client { key: s.to_string() })
    }
}
