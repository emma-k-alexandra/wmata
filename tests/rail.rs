use wmata::{Line, MetroRail, Station};

#[test]
fn test_constructor() {
    let client = MetroRail::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_lines() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.lines().is_ok());
}

#[test]
fn test_station_to_station() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client
        .station_to_station(Some(Station::A01), Some(Station::A02))
        .is_ok());
}

#[test]
fn test_stations_on() {
    let client: MetroRail = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.stations_on(Some(Line::Red)).is_ok());
}
