use crate::bus::client::responses;
use crate::bus::route::Route;
use crate::bus::stop::Stop;
use crate::bus::urls::URLs;
use crate::error::Error;
use crate::traits::Fetch;
use crate::types::{RadiusAtLatLong, Request as WMATARequest};

pub trait NeedsRoute: Fetch {
    fn positions_along(
        &self,
        route: Option<Route>,
        radius_at_lat_long: Option<RadiusAtLatLong>,
        api_key: &str,
    ) -> Result<responses::BusPositions, Error> {
        let mut query = vec![];

        if let Some(route) = route {
            query.push(("Route".to_string(), route.to_string()));
        }

        if let Some(radius_at_lat_long) = radius_at_lat_long {
            query.append(&mut radius_at_lat_long.to_query());
        }

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::Positions.to_string(),
                Some(query),
            ))
        } else {
            self.fetch::<responses::BusPositions>(WMATARequest::new(
                &api_key,
                &URLs::Positions.to_string(),
                None,
            ))
        }
    }

    fn incidents_along(
        &self,
        route: Option<Route>,
        api_key: &str,
    ) -> Result<responses::Incidents, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Incidents.to_string(),
            route.map(|r| vec![("Route".to_string(), r.to_string())]),
        ))
    }

    fn path(
        &self,
        route: Route,
        date: Option<&str>,
        api_key: &str,
    ) -> Result<responses::PathDetails, Error> {
        let mut query = vec![("RouteID".to_string(), route.to_string())];

        if let Some(date) = date {
            query.push(("Date".to_string(), date.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::PathDetails.to_string(),
            Some(query),
        ))
    }

    fn route_schedule(
        &self,
        route: Route,
        date: Option<&str>,
        including_variations: bool,
        api_key: &str,
    ) -> Result<responses::RouteSchedule, Error> {
        let mut query = vec![("RouteID".to_string(), route.to_string())];

        if let Some(date) = date {
            query.push(("Date".to_string(), date.to_string()));
        }

        if including_variations {
            query.push((
                "IncludingVariations".to_string(),
                including_variations.to_string(),
            ));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::RouteSchedule.to_string(),
            Some(query),
        ))
    }
}

pub trait NeedsStop: Fetch {
    fn next_buses(&self, stop: &Stop, api_key: &str) -> Result<responses::Predictions, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::NextBuses.to_string(),
            Some(vec![("StopID".to_string(), stop.0.to_string())]),
        ))
    }

    fn stop_schedule(
        &self,
        stop: &Stop,
        date: Option<&str>,
        api_key: &str,
    ) -> Result<responses::StopSchedule, Error> {
        let mut query = vec![("StopID".to_string(), stop.0.to_string())];

        if let Some(date) = date {
            query.push(("Date".to_string(), date.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::StopSchedule.to_string(),
            Some(query),
        ))
    }
}
