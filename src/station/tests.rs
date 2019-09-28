#[cfg(test)]
use super::*;

#[test]
fn test_next_trains() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.next_trains(|x| assert!(x.is_ok()));
}

#[test]
fn test_information() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.information(|x| {
        let x = x.unwrap();

        assert_eq!(x.code, "A01");
    });
}

#[test]
fn test_parking_information() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.parking_information(|x| {
        let x = x.unwrap();

        assert_eq!(x.stations_parking.len(), 0);
    });
}
