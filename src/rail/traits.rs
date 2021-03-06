//! Traits for sending requests based on Lines and Stations
use crate::{
    error::Error,
    rail::{client::responses, urls::URLs},
    requests::{Fetch, Request as WMATARequest},
    Line, Station,
};
use async_trait::async_trait;

#[async_trait]
pub trait NeedsLine: Fetch {
    async fn stations_on(
        &self,
        line: Option<Line>,
        api_key: &str,
    ) -> Result<responses::Stations, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Stations.to_string(),
            line.map(|l| vec![("LineCode", l.to_string())]),
        ))
        .await
    }
}

#[async_trait]
pub trait NeedsStation: Fetch {
    async fn station_to_station(
        &self,
        from_station: Option<Station>,
        to_destination_station: Option<Station>,
        api_key: &str,
    ) -> Result<responses::StationToStationInfos, Error> {
        let mut query = vec![];

        if let Some(from_station) = from_station {
            query.push(("FromStationCode", from_station.to_string()));
        }

        if let Some(to_destination_station) = to_destination_station {
            query.push(("ToStationCode", to_destination_station.to_string()));
        }

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::StationToStation.to_string(),
                Some(query),
            ))
            .await
        } else {
            self.fetch::<responses::StationToStationInfos>(WMATARequest::new(
                &api_key,
                &URLs::StationToStation.to_string(),
                None,
            ))
            .await
        }
    }

    async fn elevator_and_escalator_incidents_at(
        &self,
        station: Option<Station>,
        api_key: &str,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::ElevatorAndEscalatorIncidents.to_string(),
            station.map(|s| vec![("StationCode", s.to_string())]),
        ))
        .await
    }

    async fn incidents_at(
        &self,
        station: Option<Station>,
        api_key: &str,
    ) -> Result<responses::RailIncidents, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Incidents.to_string(),
            station.map(|s| vec![("StationCode", s.to_string())]),
        ))
        .await
    }

    async fn next_trains(
        &self,
        station: Station,
        api_key: &str,
    ) -> Result<responses::RailPredictions, Error> {
        self.fetch::<responses::RailPredictions>(WMATARequest::new(
            &api_key,
            &[URLs::NextTrains.to_string(), station.to_string()].join("/"),
            None,
        ))
        .await
    }

    async fn station_information(
        &self,
        station_code: Station,
        api_key: &str,
    ) -> Result<responses::StationInformation, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Information.to_string(),
            Some(vec![("StationCode", station_code.to_string())]),
        ))
        .await
    }

    async fn parking_information(
        &self,
        station_code: Station,
        api_key: &str,
    ) -> Result<responses::StationsParking, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::ParkingInformation.to_string(),
            Some(vec![("StationCode", station_code.to_string())]),
        ))
        .await
    }

    async fn path_from(
        &self,
        from_station: Station,
        to_station: Station,
        api_key: &str,
    ) -> Result<responses::PathBetweenStations, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Path.to_string(),
            Some(vec![
                ("FromStationCode", from_station.to_string()),
                ("ToStationCode", to_station.to_string()),
            ]),
        ))
        .await
    }

    async fn timings(
        &self,
        station_code: Station,
        api_key: &str,
    ) -> Result<responses::StationTimings, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Timings.to_string(),
            Some(vec![("StationCode", station_code.to_string())]),
        ))
        .await
    }
}
