use crate::error::Error;
use crate::rail::client::responses;
use crate::rail::urls::URLs;
use crate::traits::Fetch;
use crate::types::{Request as WMATARequest};
use crate::LineCode;
use crate::StationCode;

pub trait NeedsLineCode: Fetch {
    fn stations_on(
        &self,
        line: Option<LineCode>,
        api_key: &str,
    ) -> Result<responses::Stations, Error> {
        let mut query = vec![];

        if let Some(line) = line {
            query.push(("LineCode".to_string(), line.to_string()));
        }

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::Stations.to_string(),
                Some(query),
            ))
        } else {
            self.fetch::<responses::Stations>(WMATARequest::new(
                &api_key,
                &URLs::Stations.to_string(),
                None,
            ))
        }
    }
}

pub trait NeedsStationCode: Fetch {
    fn station_to_station(
        &self,
        from_station: Option<StationCode>,
        to_destination_station: Option<StationCode>,
        api_key: &str,
    ) -> Result<responses::StationToStationInfos, Error> {
        let mut query = vec![];

        if let Some(from_station) = from_station {
            query.push(("FromStationCode".to_string(), from_station.to_string()));
        }

        if let Some(to_destination_station) = to_destination_station {
            query.push((
                "ToStationCode".to_string(),
                to_destination_station.to_string(),
            ));
        }

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::StationToStation.to_string(),
                Some(query),
            ))
        } else {
            self.fetch::<responses::StationToStationInfos>(WMATARequest::new(
                &api_key,
                &URLs::StationToStation.to_string(),
                None,
            ))
        }
    }

    fn elevator_and_escalator_incidents_at(
        &self,
        station: Option<StationCode>,
        api_key: &str,
    ) -> Result<responses::ElevatorAndEscalatorIncidents, Error> {
        let mut query = vec![];

        if let Some(station) = station {
            query.push(("StationCode".to_string(), station.to_string()));
        }

        if !query.is_empty() {
            self.fetch(WMATARequest::new(
                &api_key,
                &URLs::ElevatorAndEscalatorIncidents.to_string(),
                Some(query),
            ))
        } else {
            self.fetch::<responses::ElevatorAndEscalatorIncidents>(WMATARequest::new(
                &api_key,
                &URLs::ElevatorAndEscalatorIncidents.to_string(),
                None,
            ))
        }
    }

    fn incidents_at(
        &self,
        station: Option<StationCode>,
        api_key: &str,
    ) -> Result<responses::RailIncidents, Error> {
        let mut query = vec![];

        if let Some(station) = station {
            query.push(("StationCode".to_string(), station.to_string()));
        }

        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Incidents.to_string(),
            Some(query),
        ))
    }

    fn next_trains(
        &self,
        station_code: StationCode,
        api_key: &str,
    ) -> Result<responses::RailPredictions, Error> {
        self.fetch::<responses::RailPredictions>(WMATARequest::new(
            &api_key,
            &[URLs::NextTrains.to_string(), station_code.to_string()].join("/"),
            None,
        ))
    }

    fn station_information(
        &self,
        station_code: StationCode,
        api_key: &str,
    ) -> Result<responses::StationInformation, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Information.to_string(),
            Some(vec![("StationCode".to_string(), station_code.to_string())]),
        ))
    }

    fn parking_information(
        &self,
        station_code: StationCode,
        api_key: &str,
    ) -> Result<responses::StationsParking, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::ParkingInformation.to_string(),
            Some(vec![("StationCode".to_string(), station_code.to_string())]),
        ))
    }

    fn path_from(
        &self,
        from_station: StationCode,
        to_station: StationCode,
        api_key: &str,
    ) -> Result<responses::PathBetweenStations, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Path.to_string(),
            Some(vec![
                ("FromStationCode".to_string(), from_station.to_string()),
                ("ToStationCode".to_string(), to_station.to_string()),
            ]),
        ))
    }

    fn timings(
        &self,
        station_code: StationCode,
        api_key: &str,
    ) -> Result<responses::StationTimings, Error> {
        self.fetch(WMATARequest::new(
            &api_key,
            &URLs::Timings.to_string(),
            Some(vec![("StationCode".to_string(), station_code.to_string())]),
        ))
    }
}