use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BusPositions {
    pub bus_positions: Box<[BusPosition]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BusPosition {
    pub block_number: String,
    pub date_time: String,
    pub deviation: f64,
    #[serde(rename = "DirectionNum")]
    pub direction_number: i32,
    pub direction_text: String,
    #[serde(rename = "Lat")]
    pub latitude: f64,
    #[serde(rename = "Lon")]
    pub longitude: f64,
    #[serde(rename = "RouteID")]
    pub route_id: String,
    pub trip_end_time: String,
    pub trip_headsign: String,
    #[serde(rename = "TripID")]
    pub trip_id: String,
    pub trip_start_time: String,
    #[serde(rename = "VehicleID")]
    pub vehicle_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Routes {
    pub routes: Box<[Route]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Route {
    #[serde(rename = "RouteID")]
    pub route_id: String,
    pub name: String,
    pub line_description: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stops {
    pub stops: Box<[Stop]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stop {
    #[serde(rename = "StopID")]
    pub stop_id: Option<String>,
    pub name: String,
    #[serde(rename = "Lat")]
    pub latitude: f64,
    #[serde(rename = "Lon")]
    pub longitude: f64,
    pub routes: Box<[String]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Incidents {
    #[serde(rename = "BusIncidents")]
    pub incidents: Box<[Incident]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Incident {
    pub date_updated: String,
    pub description: String,
    #[serde(rename = "IncidentID")]
    pub incident_id: String,
    pub incident_type: String,
    pub routes_affected: Box<[String]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathDetails {
    #[serde(rename = "RouteID")]
    pub route_id: String,
    pub name: String,
    #[serde(rename = "Direction0")]
    pub direction_zero: PathDirection,
    #[serde(rename = "Direction1")]
    pub direction_one: PathDirection,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathDirection {
    pub trip_headsign: String,
    pub direction_text: String,
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    pub shape: Box<[PathStop]>,
    pub stops: Box<[Stop]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathStop {
    #[serde(rename = "Lat")]
    pub latitude: f64,
    #[serde(rename = "Lon")]
    pub longitude: f64,
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Predictions {
    pub predictions: Box<[Prediction]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Prediction {
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    pub direction_text: String,
    pub minutes: i32,
    #[serde(rename = "RouteID")]
    pub route_id: String,
    #[serde(rename = "TripID")]
    pub trip_id: String,
    #[serde(rename = "VehicleID")]
    pub vehicle_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopSchedule {
    #[serde(rename = "ScheduleArrivals")]
    pub arrivals: Box<[Arrival]>,
    pub stop: StopRoutes,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Arrival {
    pub schedule_time: String,
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    pub start_time: String,
    pub end_time: String,
    #[serde(rename = "RouteID")]
    pub route_id: String,
    pub trip_direction_text: String,
    pub trip_headsign: String,
    #[serde(rename = "TripID")]
    pub trip_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopRoutes {
    #[serde(rename = "StopID")]
    pub stop_id: String,
    pub name: String,
    #[serde(rename = "Lat")]
    pub latitude: f64,
    #[serde(rename = "Lon")]
    pub longitude: f64,
    pub routes: Box<[String]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteSchedule {
    pub name: String,
    #[serde(rename = "Direction0")]
    pub direction_zero: Box<[RouteInfo]>,
    #[serde(rename = "Direction1")]
    pub direction_one: Box<[RouteInfo]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RouteInfo {
    #[serde(rename = "RouteID")]
    pub route_id: String,
    #[serde(rename = "DirectionNum")]
    pub direction_number: String,
    pub trip_direction_text: String,
    pub trip_headsign: String,
    pub start_time: String,
    pub end_time: String,
    pub stop_times: Box<[StopInfo]>,
    #[serde(rename = "TripID")]
    pub trip_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StopInfo {
    #[serde(rename = "StopID")]
    pub stop_id: String,
    pub stop_name: String,
    #[serde(rename = "StopSeq")]
    pub stop_sequence: i32,
    pub time: String,
}
