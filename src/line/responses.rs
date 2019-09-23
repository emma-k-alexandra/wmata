use serde::{Deserialize, Serialize};

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