#[cfg(test)]
use super::*;

#[test]
fn test_constructor() {
    let client = Client::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_routes() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.routes().unwrap().routes.len(), 470);
}

#[test]
fn test_stops() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.stops(None).unwrap().stops.len(), 10299);
}

#[test]
fn test_stops_lat_long_radius() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .stops(Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426)))
            .unwrap()
            .stops
            .len(),
        58
    );
}

#[test]
fn test_positions_along() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.positions_along(None, None).is_ok());
}

#[test]
fn test_positions_along_with_route() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.positions_along(Some(Route::One0A), None).is_ok());
}

#[test]
fn test_positions_along_with_route_and_lat_long_radius() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client
        .positions_along(
            Some(Route::One0A),
            Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426))
        )
        .is_ok());
}

#[test]
fn test_incidents_along() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.incidents_along(None).is_ok());
}

#[test]
fn test_incidents_along_route() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.incidents_along(Some(Route::One0A)).is_ok());
}

#[test]
fn test_path() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(client.path(Route::One0A, None).unwrap().route, Route::One0A);
}

#[test]
fn test_path_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .path(Route::One0A, Some(Date::new(2019, 10, 1)))
            .unwrap()
            .route,
        Route::One0A
    );
}

#[test]
fn test_route_schedule() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .route_schedule(Route::One0A, None, false)
            .unwrap()
            .name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_route_schedule_with_variations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .route_schedule(Route::One0A, None, true)
            .unwrap()
            .name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_route_schedule_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .route_schedule(Route::One0A, Some(Date::new(2019, 10, 2)), true)
            .unwrap()
            .name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_stop_schedule() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .stop_schedule(Stop::new("1001195"), None)
            .unwrap()
            .stop
            .stop
            .unwrap(),
        Stop("1001195".to_string())
    );
}

#[test]
fn test_stop_schedule_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert_eq!(
        client
            .stop_schedule(Stop::new("1001195"), Some(Date::new(2019, 10, 2)))
            .unwrap()
            .stop
            .stop
            .unwrap(),
        Stop("1001195".to_string())
    );
}
