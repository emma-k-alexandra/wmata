//! Traits used to define Bus-related method behavior
use crate::{
    bus::{client::responses, urls::URLs},
    error::Error,
    requests::{Fetch, Request as WMATARequest},
    Date, RadiusAtLatLong, Route, Stop,
};
use async_trait::async_trait;

#[async_trait]
pub trait NeedsRoute: Fetch {
    async fn positions_along(
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

        let query: Vec<(&str, String)> = query
            .iter()
            .map(|(key, value)| (key.as_str(), value.clone()))
            .collect();

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::Positions.to_string(),
                Some(query),
            ))
            .await
        } else {
            self.fetch::<responses::BusPositions>(WMATARequest::new(
                &api_key,
                &URLs::Positions.to_string(),
                None,
            ))
            .await
        }
    }

    async fn incidents_along(
        &self,
        route: Option<Route>,
        api_key: &str,
    ) -> Result<responses::Incidents, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Incidents.to_string(),
            route.map(|r| vec![("Route", r.to_string())]),
        ))
        .await
    }

    async fn path(
        &self,
        route: Route,
        date: Option<Date>,
        api_key: &str,
    ) -> Result<responses::PathDetails, Error> {
        let mut query = vec![("RouteID", route.to_string())];

        if let Some(date) = date {
            query.push(("Date", date.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::PathDetails.to_string(),
            Some(query),
        ))
        .await
    }

    async fn route_schedule(
        &self,
        route: Route,
        date: Option<Date>,
        including_variations: bool,
        api_key: &str,
    ) -> Result<responses::RouteSchedule, Error> {
        let mut query = vec![("RouteID", route.to_string())];

        if let Some(date) = date {
            query.push(("Date", date.to_string()));
        }

        if including_variations {
            query.push(("IncludingVariations", including_variations.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::RouteSchedule.to_string(),
            Some(query),
        ))
        .await
    }
}

#[async_trait]
pub trait NeedsStop: Fetch {
    async fn next_buses(
        &self,
        stop: &Stop,
        api_key: &str,
    ) -> Result<responses::Predictions, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::NextBuses.to_string(),
            Some(vec![("StopID", stop.0.to_string())]),
        ))
        .await
    }

    async fn stop_schedule(
        &self,
        stop: &Stop,
        date: Option<Date>,
        api_key: &str,
    ) -> Result<responses::StopSchedule, Error> {
        let mut query = vec![("StopID", stop.0.to_string())];

        if let Some(date) = date {
            query.push(("Date", date.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::StopSchedule.to_string(),
            Some(query),
        ))
        .await
    }
}
