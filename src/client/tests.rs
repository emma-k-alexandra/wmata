#[cfg(test)]
use super::*;

#[test]
fn test_lines() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(rail.lines().unwrap().lines.len(), 6);
}

#[test]
fn test_entrances() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(rail.entrances(1.0, 1.0, 1.0).unwrap().entrances.len(), 0);
}

#[test]
fn test_stations() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        rail.stations(Some(LineCode::Red)).unwrap().stations.len(),
        27
    );
}

#[test]
fn test_all_stations() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(rail.stations(None).unwrap().stations.len(), 95);
}

#[test]
fn test_station() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        rail.station(Some(StationCode::A01), Some(StationCode::A02))
            .unwrap()
            .station_to_station_infos
            .len(),
        1
    );
}

#[test]
fn test_station_one_station() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        rail.station(Some(StationCode::A01), None)
            .unwrap()
            .station_to_station_infos
            .len(),
        93
    );
}

#[test]
fn test_station_no_stations() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        rail.station(None, None)
            .unwrap()
            .station_to_station_infos
            .len(),
        8922
    );
}

#[test]
fn test_positions() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(rail.positions().is_ok());
}

#[test]
fn test_routes() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(rail.routes().unwrap().standard_routes.len(), 14);
}

#[test]
fn test_circuits() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(rail.circuits().unwrap().track_circuits.len(), 3486);
}

#[test]
fn test_elevator_and_escalator_incidents() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(rail
        .elevator_and_escalator_incidents(Some(StationCode::A01))
        .is_ok());
}

#[test]
fn test_all_elevator_and_escalator_incidents() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(rail.elevator_and_escalator_incidents(None).is_ok());
}

#[test]
fn test_incident() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(rail.incidents(Some(StationCode::A01)).is_ok());
}

#[test]
fn test_all_incidents() {
    let rail: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(rail.incidents(None).is_ok());
}
