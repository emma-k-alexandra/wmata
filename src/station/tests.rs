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

#[test]
fn test_path_to_station() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.path(StationCode::A02, |x| {
        let x = x.unwrap();

        assert_eq!(x.path[1].distance_to_previous_station, 4178)
    });
}

#[test]
fn test_timings() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.timings(|x| {
        let x = x.unwrap();

        assert_eq!(x.station_times[0].code, "A01");
    })
}

#[test]
fn test_to_station() {
    let station = Station::new("9e38c3eab34c4e6c990828002828f5ed", StationCode::A01);

    station.to_station(StationCode::A02, |x| {
        let x = x.unwrap();

        assert_eq!(x.station_to_station_infos[0].source_station, "A01");
    })
}
