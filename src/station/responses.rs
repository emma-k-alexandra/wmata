use serde::{Deserialize, Serialize};

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
