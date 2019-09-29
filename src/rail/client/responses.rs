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
pub struct RailPredictions {
    pub trains: Box<[RailPrediction]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct RailPrediction {
    pub car: Option<String>,
    pub destination: String,
    pub destination_code: Option<String>,
    pub destination_name: String,
    pub group: String,
    pub line: String,
    pub location_code: String,
    pub location_name: String,
    #[serde(rename = "Min")]
    pub minutes: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationInformation {
    pub address: StationAddress,
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
pub struct StationAddress {
    pub city: String,
    pub state: String,
    pub street: String,
    pub zip: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationsParking {
    pub stations_parking: Box<[StationParking]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationParking {
    pub code: String,
    pub notes: String,
    pub all_day_parking: AllDayParking,
    pub short_term_parking: ShortTermParking,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AllDayParking {
    pub total_count: i32,
    pub rider_cost: f64,
    pub non_rider_cost: f64,
    pub saturday_rider_cost: f64,
    pub saturday_non_rider_cost: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ShortTermParking {
    pub total_count: i32,
    pub notes: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PathBetweenStations {
    pub path: Box<[Path]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Path {
    #[serde(rename = "DistanceToPrev")]
    pub distance_to_previous_station: i32,
    pub line_code: String,
    #[serde(rename = "SeqNum")]
    pub sequence_number: i32,
    pub station_code: String,
    pub station_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationTimings {
    pub station_times: Box<[StationTime]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationTime {
    pub code: String,
    pub station_name: String,
    // You're gonna love this
    pub monday: StationFirstLastTrains,
    pub tuesday: StationFirstLastTrains,
    pub wednesday: StationFirstLastTrains,
    pub thursday: StationFirstLastTrains,
    pub friday: StationFirstLastTrains,
    pub saturday: StationFirstLastTrains,
    pub sunday: StationFirstLastTrains,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StationFirstLastTrains {
    pub opening_time: String,
    pub first_trains: Box<[TrainTime]>,
    pub last_trains: Box<[TrainTime]>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct TrainTime {
    pub time: String,
    pub destination_station: String,
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
