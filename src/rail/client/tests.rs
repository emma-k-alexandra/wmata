//! Tests for MetroRail Client
#[cfg(test)]
use super::*;

#[test]
fn test_constructor() {
    let client = Client::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.api_key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_lines() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.lines().unwrap().lines.len(), 6);
}

#[test]
fn test_entrances() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.entrances(1.0, 1.0, 1.0).unwrap().entrances.len(), 0);
}

#[test]
fn test_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .stations_on(Some(LineCode::Red))
            .unwrap()
            .stations
            .len(),
        27
    );
}

#[test]
fn test_all_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.stations_on(None).unwrap().stations.len(), 95);
}

#[test]
fn test_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .station_to_station(Some(StationCode::A01), Some(StationCode::A02))
            .unwrap()
            .station_to_station_infos
            .len(),
        1
    );
}

#[test]
fn test_station_one_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .station_to_station(Some(StationCode::A01), None)
            .unwrap()
            .station_to_station_infos
            .len(),
        93
    );
}

#[test]
fn test_station_no_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .station_to_station(None, None)
            .unwrap()
            .station_to_station_infos
            .len(),
        8922
    );
}

#[test]
fn test_positions() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.positions().is_ok());
}

#[test]
fn test_routes() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.routes().unwrap().standard_routes.len(), 14);
}

#[test]
fn test_circuits() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.circuits().unwrap().track_circuits.len(), 3486);
}

#[test]
fn test_elevator_and_escalator_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client
        .elevator_and_escalator_incidents_at(Some(StationCode::A01))
        .is_ok());
}

#[test]
fn test_all_elevator_and_escalator_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.elevator_and_escalator_incidents_at(None).is_ok());
}

#[test]
fn test_incident() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.incidents_at(Some(StationCode::A01)).is_ok());
}

#[test]
fn test_all_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.incidents_at(None).is_ok());
}

#[test]
fn test_next_trains() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.next_trains(StationCode::A01).is_ok());
}

#[test]
fn test_information() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client.station_information(StationCode::A01).unwrap().code,
        "A01"
    );
}

#[test]
fn test_parking_information() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .parking_information(StationCode::A01)
            .unwrap()
            .stations_parking
            .len(),
        0
    );
}

#[test]
fn test_path_to_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .path_from(StationCode::A01, StationCode::A02)
            .unwrap()
            .path[1]
            .distance_to_previous_station,
        4178
    );
}

#[test]
fn test_timings() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client.timings(StationCode::A01).unwrap().station_times[0].code,
        "A01"
    );
}
