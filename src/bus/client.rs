pub mod responses;
mod tests;

use crate::bus::route::RouteID;
use crate::bus::urls::URLs;
use crate::error::Error;
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
    // This isn't actually dead code,
    // but the compiler is very angry about it
    #[allow(dead_code)]
    pub fn new(api_key: &str) -> Self {
        Client {
            api_key: api_key.to_string(),
        }
    }
}

// These don't take Route IDs or Stop IDs
impl Client {
    pub fn routes(&self) -> Result<responses::Routes, Error> {
        self.fetch::<responses::Routes, Empty>(&URLs::Routes.to_string(), None)
    }

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
    pub fn next_buses(&self, stop_id: &str) -> Result<responses::Predictions, Error> {
        self.fetch(&URLs::NextBuses.to_string(), Some(&[("StopID", stop_id)]))
    }

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

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Client {
            api_key: s.to_string(),
        })
    }
}
