//! URLs of Rail-related endpoints
pub enum URLs {
    NextTrains,
    Information,
    ParkingInformation,
    Path,
    Timings,
    StationToStation,
    Lines,
    Entrances,
    Positions,
    Routes,
    Circuits,
    ElevatorAndEscalatorIncidents,
    Incidents,
    Stations,
}

impl ToString for URLs {
    fn to_string(&self) -> String {
        match self {
            URLs::NextTrains => {
                "https://api.wmata.com/StationPrediction.svc/json/GetPrediction".to_string()
            }
            URLs::Information => "https://api.wmata.com/Rail.svc/json/jStationInfo".to_string(),
            URLs::ParkingInformation => {
                "https://api.wmata.com/Rail.svc/json/jStationParking".to_string()
            }
            URLs::Path => "https://api.wmata.com/Rail.svc/json/jPath".to_string(),
            URLs::Timings => "https://api.wmata.com/Rail.svc/json/jStationTimes".to_string(),
            URLs::StationToStation => {
                "https://api.wmata.com/Rail.svc/json/jSrcStationToDstStationInfo".to_string()
            }
            URLs::Lines => "https://api.wmata.com/Rail.svc/json/jLines".to_string(),
            URLs::Entrances => "https://api.wmata.com/Rail.svc/json/jStationEntrances".to_string(),
            URLs::Positions => "https://api.wmata.com/TrainPositions/TrainPositions".to_string(),
            URLs::Routes => "https://api.wmata.com/TrainPositions/StandardRoutes".to_string(),
            URLs::Circuits => "https://api.wmata.com/TrainPositions/TrackCircuits".to_string(),
            URLs::ElevatorAndEscalatorIncidents => {
                "https://api.wmata.com/Incidents.svc/json/ElevatorIncidents".to_string()
            }
            URLs::Incidents => "https://api.wmata.com/Incidents.svc/json/Incidents".to_string(),
            URLs::Stations => "https://api.wmata.com/Rail.svc/json/jStations".to_string(),
        }
    }
}
