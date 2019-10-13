//! MetroBus client. Contains the client for fetching data from
//! the WMATA API and data structures returned from those endpoint calls.
pub mod responses;
mod tests;

use crate::bus::route::Route;
use crate::bus::stop::Stop;
use crate::bus::traits::{NeedsRoute, NeedsStop};
use crate::bus::urls::URLs;
use crate::error::Error;
use crate::traits::Fetch;
use crate::types::{RadiusAtLatLong, Request as WMATARequest};
use std::str::FromStr;

/// MetroBus client. Used to fetch MetroBus-related information from the WMATA API.
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
    /// use wmata::MetroBus;
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    pub fn new(api_key: &str) -> Self {
        Client {
            key: api_key.to_string(),
        }
    }
}

// These don't take Route IDs or Stop IDs
impl Client {
    /// List of all bus route variants.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d6a?)
    ///
    /// # Examples
    /// ```
    /// use wmata::MetroBus;
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.routes().is_ok());
    /// ```
    pub fn routes(&self) -> Result<responses::Routes, Error> {
        self.fetch::<responses::Routes>(WMATARequest::new(
            &self.key,
            &URLs::Routes.to_string(),
            None,
        ))
    }

    /// Nearby bus stops based on latitude, longitude, and radius.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d6d?)
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, RadiusAtLatLong};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stops(Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426))).is_ok());
    /// ```
    pub fn stops(
        &self,
        radius_at_lat_long: Option<RadiusAtLatLong>,
    ) -> Result<responses::Stops, Error> {
        if let Some(radius_at_lat_long) = radius_at_lat_long {
            self.fetch(WMATARequest::new(
                &self.key,
                &URLs::Stops.to_string(),
                Some(
                    radius_at_lat_long
                        .to_query()
                        .iter()
                        .map(|(key, value)| (key.as_str(), value.clone()))
                        .collect(),
                ),
            ))
        } else {
            self.fetch::<responses::Stops>(WMATARequest::new(
                &self.key,
                &URLs::Stops.to_string(),
                None,
            ))
        }
    }
}

impl NeedsRoute for Client {}

// Overwriting NeedsRoute methods
impl Client {
    /// Bus positions for the given route around a given lat/long.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d68?)
    ///
    /// # Example
    /// ```
    /// use wmata::{MetroBus, Route, RadiusAtLatLong};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.positions_along(
    ///     Some(Route::A2),
    ///     Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426))
    /// ).is_ok());
    /// ```
    pub fn positions_along(
        &self,
        route: Option<Route>,
        radius_at_lat_long: Option<RadiusAtLatLong>,
    ) -> Result<responses::BusPositions, Error> {
        <Self as NeedsRoute>::positions_along(&self, route, radius_at_lat_long, &self.key)
    }

    /// Reported bus incidents/delays for a given route.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763641281d83086473f232/operations/54763641281d830c946a3d75)
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, Route};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.incidents_along(Some(Route::A2)).is_ok());
    /// ```
    pub fn incidents_along(&self, route: Option<Route>) -> Result<responses::Incidents, Error> {
        <Self as NeedsRoute>::incidents_along(&self, route, &self.key)
    }

    /// For an optional given date, returns the set of ordered latitude/longitude
    /// points along a route variant along with the list of stops served.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d69?)
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, Route};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path(Route::A2, None).is_ok());
    /// ```
    /// With a date
    /// ```
    /// use wmata::{MetroBus, Route};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.path(Route::A2, Some("2019-10-02")).is_ok());
    /// ```
    pub fn path(&self, route: Route, date: Option<&str>) -> Result<responses::PathDetails, Error> {
        <Self as NeedsRoute>::path(&self, route, date, &self.key)
    }

    /// Schedules for a given route variant for an optional given date.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d6b?)
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Variations
    /// Whether or not to include variations if a base route is specified in Route.
    /// For example, if B30 is specified and IncludingVariations is set to true,
    /// data for all variations of B30 such as B30v1, B30v2, etc. will be returned.
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, Route};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.route_schedule(Route::A2, None, false).is_ok());
    /// ```
    ///
    /// with date and variations
    /// ```
    /// use wmata::{MetroBus, Route};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.route_schedule(Route::A2, Some("2019-10-02"), true).is_ok());
    /// ```
    pub fn route_schedule(
        &self,
        route: Route,
        date: Option<&str>,
        including_variations: bool,
    ) -> Result<responses::RouteSchedule, Error> {
        <Self as NeedsRoute>::route_schedule(&self, route, date, including_variations, &self.key)
    }
}

impl NeedsStop for Client {}

// Overwriting NeedsStop methods
impl Client {
    /// Next bus arrivals at a given stop.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/5476365e031f590f38092508/operations/5476365e031f5909e4fe331d)
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, Stop};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.next_buses(Stop::new("1001195")).is_ok());
    /// ```
    pub fn next_buses(&self, stop: Stop) -> Result<responses::Predictions, Error> {
        <Self as NeedsStop>::next_buses(&self, &stop, &self.key)
    }

    /// Buses scheduled at a stop for an optional given date.
    /// [WMATA Documentation](https://developer.wmata.com/docs/services/54763629281d83086473f231/operations/5476362a281d830c946a3d6c?)
    ///
    /// # Date
    /// Date is in YYYY-MM-DD format.
    /// ***Omit date for current date***
    ///
    /// # Examples
    /// ```
    /// use wmata::{MetroBus, Stop};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stop_schedule(Stop::new("1001195"), None).is_ok());
    /// ```
    ///
    /// with date
    /// ```
    /// use wmata::{MetroBus, Stop};
    ///
    /// let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");
    /// assert!(client.stop_schedule(Stop::new("1001195"), Some("2019-10-02")).is_ok());
    /// ```
    pub fn stop_schedule(
        &self,
        stop: Stop,
        date: Option<&str>,
    ) -> Result<responses::StopSchedule, Error> {
        <Self as NeedsStop>::stop_schedule(&self, &stop, date, &self.key)
    }
}

impl FromStr for Client {
    type Err = Error;

    /// Converts a string into a MetroBus Client.
    ///
    /// # Examples
    /// ```
    /// use wmata::MetroBus;
    /// let client: MetroBus = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    ///
    /// assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client { key: s.to_string() })
    }
}
