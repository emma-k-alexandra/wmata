use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Lines {
    pub lines: Box<[Line]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Line {
    pub line_code: String,
    pub display_name: String,
    pub start_station_code: String,
    pub end_station_code: String,
    #[serde(rename = "InternalDestination1")]
    pub first_internal_destination: String,
    #[serde(rename = "InternalDestination2")]
    pub second_internal_destination: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationEntrances {
    pub entrances: Box<[StationEntrance]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationEntrance {
    pub description: String,
    #[serde(rename = "ID")]
    pub id: String,
    pub latitude: f64,
    pub longitude: f64,
    pub name: String,
    #[serde(rename = "StationCode1")]
    pub first_station_code: String,
    #[serde(rename = "StationCode2")]
    pub second_station_code: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Stations {
    pub stations: Box<[Station]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Station {
    pub address: Address,
    pub code: String,
    #[serde(rename = "Lat")]
    pub latitude: f64,
    #[serde(rename = "Lon")]
    pub longitude: f64,
    #[serde(rename = "LineCode1")]
    pub first_line_code: String,
    #[serde(rename = "LineCode2")]
    pub second_line_code: Option<String>,
    #[serde(rename = "LineCode3")]
    pub third_line_code: Option<String>,
    #[serde(rename = "LineCode4")]
    pub fourth_line_code: Option<String>,
    pub name: String,
    #[serde(rename = "StationTogether1")]
    pub first_station_together: String,
    #[serde(rename = "StationTogether2")]
    pub second_station_together: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Address {
    pub city: String,
    pub state: String,
    pub street: String,
    pub zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationToStationInfos {
    pub station_to_station_infos: Box<[StationToStationInfo]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationToStationInfo {
    pub composite_miles: f64,
    pub destination_station: String,
    pub rail_fare: RailFare,
    pub rail_time: i32,
    pub source_station: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailFare {
    pub off_peak_time: f64,
    pub peak_time: f64,
    pub senior_disabled: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainPositions {
    pub train_positions: Box<[TrainPosition]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainPosition {
    pub train_id: String,
    pub train_number: String,
    pub car_count: i32,
    #[serde(rename = "DirectionNum")]
    pub direction_number: i32,
    pub circuit_id: i32,
    pub destination_station_code: Option<String>,
    pub line_code: Option<String>,
    pub seconds_at_location: i32,
    pub service_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardRoutes {
    pub standard_routes: Box<[StandardRoute]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StandardRoute {
    pub line_code: String,
    #[serde(rename = "TrackNum")]
    pub track_number: i32,
    pub track_circuits: Box<[TrackCircuitWithStation]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuitWithStation {
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
    pub circuit_id: i32,
    pub station_code: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuits {
    pub track_circuits: Box<[TrackCircuit]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackCircuit {
    pub track: i32,
    pub circuit_id: i32,
    pub neighbors: Box<[TrackNeighbor]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrackNeighbor {
    pub neighbor_type: String,
    pub circuit_ids: Box<[i32]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ElevatorAndEscalatorIncidents {
    #[serde(rename = "ElevatorIncidents")]
    pub incidents: Box<[ElevatorAndEscalatorIncident]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ElevatorAndEscalatorIncident {
    pub unit_name: String,
    pub unit_type: String,
    pub unit_status: Option<String>,
    pub station_code: String,
    pub station_name: String,
    pub location_description: String,
    pub symptom_code: Option<String>,
    pub time_out_of_service: String,
    pub symptom_description: String,
    pub display_order: f64,
    #[serde(rename = "DateOutOfServ")]
    pub date_out_of_service: String,
    pub date_updated: String,
    pub estimated_return_to_service: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailIncidents {
    pub incidents: Box<[RailIncident]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailIncident {
    #[serde(rename = "IncidentID")]
    pub incident_id: String,
    pub description: String,
    pub start_location_full_name: Option<String>,
    pub end_location_full_name: Option<String>,
    pub passenger_delay: f64,
    pub delay_severity: Option<String>,
    pub incident_type: String,
    pub emergency_text: Option<String>,
    pub lines_affected: String,
    pub date_updated: String,
}
