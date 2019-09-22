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
