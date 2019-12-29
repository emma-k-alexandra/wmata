//! Responses from MetroBus related methods from the WMATA API.
use crate::{Route, Stop};
use chrono::{DateTime, FixedOffset};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BusPositions {
    /// See [`BusPosition`].
    pub bus_positions: Box<[BusPosition]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BusPosition {
    /// Date and time (Eastern Standard Time) of last position update.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub date_time: DateTime<FixedOffset>,
    /// Deviation, in minutes, from schedule. Positive values indicate that the bus is running late while negative ones are for buses running ahead of schedule.
    pub deviation: f64,
    /// Deprecated. Use the DirectionText for a customer-friendly description of direction.
    #[serde(rename = "DirectionNum")]
    pub direction_number: i32,
    /// General direction of the trip, not the bus itself (e.g.: NORTH, SOUTH, EAST, WEST).
    pub direction_text: String,
    /// Latitude of bus.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of bus.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// Base route name as shown on the bus. Note that the base route name could also refer to any variant, so a RouteID of 10A could refer to 10A, 10Av1, 10Av2, etc.
    #[serde(rename = "RouteID")]
    pub route: Route,
    /// Scheduled end date and time (Eastern Standard Time) of the bus's current trip.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub trip_end_time: DateTime<FixedOffset>,
    /// Destination of the bus.
    pub trip_headsign: String,
    /// Unique trip ID. This can be correlated with the data returned from the schedule-related methods.
    #[serde(rename = "TripID")]
    pub trip_id: String,
    /// Scheduled start date and time (Eastern Standard Time) of the bus's current trip.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub trip_start_time: DateTime<FixedOffset>,
    /// Unique identifier for the bus. This is usually visible on the bus itself.
    #[serde(rename = "VehicleID")]
    pub vehicle_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Routes {
    /// See [`Route`].
    pub routes: Box<[RouteResponse]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteResponse {
    /// Unique identifier for a given route variant. Can be used in various other bus-related methods.
    #[serde(rename = "RouteID")]
    pub route: Route,
    /// Descriptive name of the route variant.
    pub name: String,
    /// Denotes the route variant’s grouping – lines are a combination of routes which lie in the same corridor and which have significant portions of their paths along the same roadways.
    pub line_description: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stops {
    /// See [`Stop`].
    pub stops: Box<[StopResponse]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopResponse {
    /// 7-digit regional ID which can be used in various bus-related methods. If unavailable, the StopID will be 0 or NULL.
    #[serde(rename = "StopID")]
    pub stop: Option<Stop>,
    /// Stop name. May be slightly different from what is spoken or displayed in the bus.
    pub name: String,
    /// Latitude of stop.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of bus.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// String array of route variants which provide service at this stop. Note that these are not date-specific; any route variant which stops at this stop on any day will be listed.
    pub routes: Box<[Route]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Incidents {
    /// See [`Incident`]
    #[serde(rename = "BusIncidents")]
    pub incidents: Box<[Incident]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Incident {
    /// Date and time (Eastern Standard Time) of last update.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub date_updated: DateTime<FixedOffset>,
    /// Free-text description of the delay or incident.
    pub description: String,
    /// Unique identifier for an incident.
    #[serde(rename = "IncidentID")]
    pub incident_id: String,
    /// Free-text description of the incident type. Usually Delay or Alert but is subject to change at any time.
    pub incident_type: String,
    /// Array containing routes affected. Routes listed are usually identical to base route names (i.e.: not 10Av1 or 10Av2, but 10A), but may differ from what our bus methods return.
    pub routes_affected: Box<[Route]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathDetails {
    /// [`Route`] of the route.
    #[serde(rename = "RouteID")]
    pub route: Route,
    /// Descriptive name for the route.
    pub name: String,
    /// Structures describing path/stop information.
    /// Most routes will return content in both Direction0 and Direction1 elements, though a few will return NULL for Direction0 or for Direction1.
    /// 0 or 1 are binary properties. There is no specific mapping to direction, but a different value for the same route signifies that the route is in an opposite direction.
    #[serde(rename = "Direction0")]
    pub direction_zero: PathDirection,
    /// Structures describing path/stop information.
    /// Most routes will return content in both Direction0 and Direction1 elements, though a few will return NULL for Direction0 or for Direction1.
    /// 0 or 1 are binary properties. There is no specific mapping to direction, but a different value for the same route signifies that the route is in an opposite direction.
    #[serde(rename = "Direction1")]
    pub direction_one: PathDirection,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathDirection {
    /// Descriptive text of where the bus is headed. This is similar, but not necessarily identical, to what is displayed on the bus.
    pub trip_headsign: String,
    /// General direction of the route variant (NORTH, SOUTH, EAST, WEST, LOOP, etc.).
    pub direction_text: String,
    /// Warning: Deprecated. Use the DirectionText element to denote the general direction of the route variant.
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    /// See [`PathStop`]
    pub shape: Box<[PathShape]>,
    /// See [`Stop`]
    pub stops: Box<[StopRoutes]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathShape {
    /// Latitude of stop.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of stop.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// Order of the point in the sequence of PathStop.
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Predictions {
    /// See [`Prediction`].
    pub predictions: Box<[Prediction]>,
    /// Full name of the given StopID.
    pub stop_name: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Prediction {
    /// Denotes a binary direction (0 or 1) of the bus. There is no specific mapping to direction, but a different value for the same route signifies that the buses are traveling in opposite directions. Use the DirectionText element to show the actual destination of the bus.
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    /// Customer-friendly description of direction and destination for a bus.
    pub direction_text: String,
    /// Minutes until bus arrival at this stop.
    pub minutes: i32,
    #[serde(rename = "RouteID")]
    /// [`Route`] of the bus. Base route name as shown on the bus. This can be used in other bus-related methods. Note that all variants will be shown as their base route names (i.e.: 10Av1 and 10Av2 will be shown as 10A).
    pub route: Route,
    /// Trip identifier. This can be correlated with the data in our bus schedule information as well as bus positions.
    #[serde(rename = "TripID")]
    pub trip_id: String,
    /// Bus identifier. This can be correlated with results returned from bus positions.
    #[serde(rename = "VehicleID")]
    pub vehicle_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopSchedule {
    /// See [`Arrival`].
    #[serde(rename = "ScheduleArrivals")]
    pub arrivals: Box<[Arrival]>,
    /// See [`StopRoutes`].
    pub stop: StopRoutes,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Arrival {
    /// Date and time (Eastern Standard Time) when the bus is scheduled to stop at this location.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub schedule_time: DateTime<FixedOffset>,
    /// Denotes a binary direction (0 or 1) of the bus. There is no specific mapping to direction, but a different value for the same route signifies that the buses are traveling in opposite directions. Use the TripDirectionText element to show the actual destination of the bus.
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    /// Scheduled start date and time (Eastern Standard Time) for this trip.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub start_time: DateTime<FixedOffset>,
    /// Scheduled end date and time (Eastern Standard Time) for this trip.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub end_time: DateTime<FixedOffset>,
    /// [`Route`] of the bus. Bus route variant identifier (pattern). This variant can be used in several other bus methods which accept variants. Note that customers will never see anything other than the base route name, so variants 10A, 10Av1, 10Av2, etc. will be displayed as 10A on the bus.
    #[serde(rename = "RouteID")]
    pub route: Route,
    /// General direction of the trip (e.g.: NORTH, SOUTH, EAST, WEST).
    pub trip_direction_text: String,
    /// Destination of the bus.
    pub trip_headsign: String,
    /// Trip identifier. This can be correlated with the data in our bus schedule information as well as bus positions.
    #[serde(rename = "TripID")]
    pub trip_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopRoutes {
    /// 7-digit regional ID which can be used in various bus-related methods. If unavailable, the StopID will be 0 or NULL.
    #[serde(rename = "StopID")]
    pub stop: Option<Stop>,
    /// Stop name. May be slightly different from what is spoken or displayed in the bus.
    pub name: String,
    /// Latitude of stop.
    #[serde(rename = "Lat")]
    pub latitude: f64,
    /// Longitude of stop.
    #[serde(rename = "Lon")]
    pub longitude: f64,
    /// String array of route variants which provide service at this stop. Note that these are not date-specific; any route variant which stops at this stop on any day will be listed.
    pub routes: Box<[Route]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteSchedule {
    /// Descriptive name for the route.
    pub name: String,
    /// See [`RouteInfo`].
    #[serde(rename = "Direction0")]
    pub direction_zero: Box<[RouteInfo]>,
    /// See [`RouteInfo`].
    #[serde(rename = "Direction1")]
    pub direction_one: Box<[RouteInfo]>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteInfo {
    /// [`Route`] of the route. Bus route variant. This can be used in several other bus methods which accept variants.
    #[serde(rename = "RouteID")]
    pub route: Route,
    /// Warning: Deprecated. Use the TripDirectionText element to denote the general direction of the trip.
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    /// General direction of the trip (NORTH, SOUTH, EAST, WEST, LOOP, etc.).
    pub trip_direction_text: String,
    /// Descriptive text of where the bus is headed. This is similar, but not necessarily identical, to what is displayed on the bus.
    pub trip_headsign: String,
    /// Scheduled start date and time (Eastern Standard Time) for this trip. Will be in YYYY-MM-DDTHH:mm:ss format (e.g.: 2014-10-27T13:17:00).
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub start_time: DateTime<FixedOffset>,
    /// Scheduled end date and time (Eastern Standard Time) for this trip. Will be in YYYY-MM-DDTHH:mm:ss format (e.g.: 2014-10-27T13:17:00).
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub end_time: DateTime<FixedOffset>,
    /// See [`StopInfo`].
    pub stop_times: Box<[StopInfo]>,
    /// Unique trip ID. This can be correlated with the data returned from the schedule-related methods.
    #[serde(rename = "TripID")]
    pub trip_id: String,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopInfo {
    /// 7-digit regional ID which can be used in various bus-related methods. If unavailable, the StopID will be 0 or NULL.
    #[serde(rename = "StopID")]
    pub stop: Stop,
    /// Stop name. May be slightly different from what is spoken or displayed in the bus.
    pub stop_name: String,
    /// Order of the stop in the sequence of StopInfo.
    #[serde(rename = "StopSeq")]
    pub stop_sequence: i32,
    /// Scheduled departure date and time (Eastern Standard Time) from this stop.
    #[serde(deserialize_with = "crate::date::deserialize")]
    pub time: DateTime<FixedOffset>,
}
