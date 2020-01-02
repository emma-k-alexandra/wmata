//! Tests for MetroRail Client
#[cfg(test)]
use super::*;

#[cfg(test)]
use tokio_test::block_on;

#[test]
fn test_constructor() {
    let client = Client::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_lines() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let lines = block_on(async { client.lines().await });

    assert_eq!(lines.unwrap().lines.len(), 6);
}

#[test]
fn test_entrances() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let entrances = block_on(async { client.entrances(RadiusAtLatLong::new(1, 1.0, 1.0)).await });

    assert_eq!(entrances.unwrap().entrances.len(), 0);
}

#[test]
fn test_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stations = block_on(async { client.stations_on(Some(Line::Red)).await });

    assert_eq!(stations.unwrap().stations.len(), 27);
}

#[test]
fn test_all_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stations = block_on(async { client.stations_on(None).await });

    assert_eq!(stations.unwrap().stations.len(), 95);
}

#[test]
fn test_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let station_to_station = block_on(async {
        client
            .station_to_station(Some(Station::A01), Some(Station::A02))
            .await
    });

    assert_eq!(
        station_to_station.unwrap().station_to_station_infos.len(),
        1
    );
}

#[test]
fn test_station_one_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let station_to_station =
        block_on(async { client.station_to_station(Some(Station::A01), None).await });

    assert_eq!(
        station_to_station.unwrap().station_to_station_infos.len(),
        93
    );
}

#[test]
fn test_station_no_stations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let station_to_station = block_on(async { client.station_to_station(None, None).await });

    assert_eq!(
        station_to_station.unwrap().station_to_station_infos.len(),
        8922
    );
}

#[test]
fn test_positions() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let positions = block_on(async { client.positions().await });

    assert!(positions.is_ok());
}

#[test]
fn test_routes() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let routes = block_on(async { client.routes().await });

    assert_eq!(routes.unwrap().standard_routes.len(), 14);
}

#[test]
fn test_circuits() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let circuits = block_on(async { client.circuits().await });

    assert_eq!(circuits.unwrap().track_circuits.len(), 3486);
}

#[test]
fn test_elevator_and_escalator_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async {
        client
            .elevator_and_escalator_incidents_at(Some(Station::A01))
            .await
    });

    assert!(incidents.is_ok());
}

#[test]
fn test_all_elevator_and_escalator_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async { client.elevator_and_escalator_incidents_at(None).await });

    assert!(incidents.is_ok());
}

#[test]
fn test_incident() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async { client.incidents_at(Some(Station::A01)).await });

    assert!(incidents.is_ok());
}

#[test]
fn test_all_incidents() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async { client.incidents_at(None).await });

    assert!(incidents.is_ok());
}

#[test]
fn test_next_trains() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let next_trains = block_on(async { client.next_trains(Station::A01).await });

    assert!(next_trains.is_ok());
}

#[test]
fn test_information() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let station_information = block_on(async { client.station_information(Station::A01).await });

    assert_eq!(station_information.unwrap().station, Station::A01);
}

#[test]
fn test_parking_information() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let parking_information = block_on(async { client.parking_information(Station::A01).await });

    assert_eq!(parking_information.unwrap().stations_parking.len(), 0);
}

#[test]
fn test_path_to_station() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let path = block_on(async { client.path_from(Station::A01, Station::A02).await });

    assert_eq!(path.unwrap().path[1].distance_to_previous_station, 4178);
}

#[test]
fn test_timings() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let timings = block_on(async { client.timings(Station::A01).await });

    assert_eq!(timings.unwrap().station_times[0].station, Station::A01);
}

#[test]
fn test_station_name() {
    let station = Station::A01;

    assert_eq!(station.name(), "Metro Center");
}

#[test]
fn test_station_lines() {
    let station = Station::N01;

    assert_eq!(station.lines(), &[Line::Silver])
}
