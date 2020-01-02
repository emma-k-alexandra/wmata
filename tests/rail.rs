use tokio_test::block_on;
use wmata::{Line, MetroRail, Station};

#[test]
fn test_constructor() {
    let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_lines() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let lines = block_on(async { client.lines().await });

    assert!(lines.is_ok());
}

#[test]
fn test_station_to_station() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let station_to_station = block_on(async {
        client
            .station_to_station(Some(Station::A01), Some(Station::A02))
            .await
    });

    assert!(station_to_station.is_ok());
}

#[test]
fn test_stations_on() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stations = block_on(async { client.stations_on(Some(Line::Red)).await });

    assert!(stations.is_ok());
}
