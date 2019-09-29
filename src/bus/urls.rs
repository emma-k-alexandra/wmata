pub enum URLs {
    Routes,
    Stops,
    Incidents,
    Positions,
    PathDetails,
    RouteSchedule,
    NextBuses,
    StopSchedule,
}

impl ToString for URLs {
    fn to_string(&self) -> String {
        match self {
            URLs::Routes => "https://api.wmata.com/Bus.svc/json/jRoutes".to_string(),
            URLs::Stops => "https://api.wmata.com/Bus.svc/json/jStops".to_string(),
            URLs::Incidents => "https://api.wmata.com/Incidents.svc/json/BusIncidents".to_string(),
            URLs::Positions => "https://api.wmata.com/Bus.svc/json/jBusPositions".to_string(),
            URLs::PathDetails => "https://api.wmata.com/Bus.svc/json/jRouteDetails".to_string(),
            URLs::RouteSchedule => "https://api.wmata.com/Bus.svc/json/jRouteSchedule".to_string(),
            URLs::NextBuses => {
                "https://api.wmata.com/NextBusService.svc/json/jPredictions".to_string()
            }
            URLs::StopSchedule => "https://api.wmata.com/Bus.svc/json/jStopSchedul".to_string(),
        }
    }
}
