#[cfg(test)]
use super::*;

#[test]
fn test_lines() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.lines(|response| assert_eq!(response.unwrap().lines.len(), 6));
}

#[test]
fn test_entrances() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.entrances(1.0, 1.0, 1.0, |x| assert_eq!(x.unwrap().entrances.len(), 0));
}

#[test]
fn test_stations() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.stations(Some(LineCode::Red), |x| {
        assert_eq!(x.unwrap().stations.len(), 27)
    });
}

#[test]
fn test_all_stations() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.stations(None, |x| assert_eq!(x.unwrap().stations.len(), 95));
}

#[test]
fn test_station() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.station(Some(StationCode::A01), Some(StationCode::A02), |x| {
        assert_eq!(x.unwrap().station_to_station_infos.len(), 1);
    });
}

#[test]
fn test_station_one_station() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.station(Some(StationCode::A01), None, |x| {
        assert_eq!(x.unwrap().station_to_station_infos.len(), 93);
    });
}

#[test]
fn test_station_no_stations() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.station(None, None, |x| {
        assert_eq!(x.unwrap().station_to_station_infos.len(), 8922);
    });
}

#[test]
fn test_positions() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.positions(|_| assert!(true));
}

#[test]
fn test_routes() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.routes(|x| assert_eq!(x.unwrap().standard_routes.len(), 14));
}

#[test]
fn test_circuits() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.circuits(|x| assert_eq!(x.unwrap().track_circuits.len(), 3486));
}

#[test]
fn test_elevator_and_escalator_incidents() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.elevator_and_escalator_incidents(Some(StationCode::A01), |_| assert!(true))
}

#[test]
fn test_all_elevator_and_escalator_incidents() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.elevator_and_escalator_incidents(None, |_| assert!(true))
}

#[test]
fn test_incident() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.incidents(Some(StationCode::A01), |_| assert!(true));
}

#[test]
fn test_all_incidents() {
    let rail = "9e38c3eab34c4e6c990828002828f5ed".parse::<Rail>().unwrap();

    rail.incidents(None, |_| assert!(true));
}
