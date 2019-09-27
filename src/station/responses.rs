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
