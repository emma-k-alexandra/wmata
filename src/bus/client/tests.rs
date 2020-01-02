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
fn test_routes() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let routes = block_on(async { client.routes().await });

    assert_eq!(routes.unwrap().routes.len(), 470);
}

#[test]
fn test_stops() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stops = block_on(async { client.stops(None).await });

    assert_eq!(stops.unwrap().stops.len(), 10299);
}

#[test]
fn test_stops_lat_long_radius() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stops = block_on(async {
        client
            .stops(Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426)))
            .await
    });

    assert_eq!(stops.unwrap().stops.len(), 58);
}

#[test]
fn test_positions_along() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let positions = block_on(async { client.positions_along(None, None).await });

    assert!(positions.is_ok());
}

#[test]
fn test_positions_along_with_route() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let positions = block_on(async { client.positions_along(Some(Route::One0A), None).await });

    assert!(positions.is_ok());
}

#[test]
fn test_positions_along_with_route_and_lat_long_radius() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let positions = block_on(async {
        client
            .positions_along(
                Some(Route::One0A),
                Some(RadiusAtLatLong::new(1000, 38.8817596, -77.0166426)),
            )
            .await
    });

    assert!(positions.is_ok());
}

#[test]
fn test_incidents_along() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async { client.incidents_along(None).await });

    assert!(incidents.is_ok());
}

#[test]
fn test_incidents_along_route() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let incidents = block_on(async { client.incidents_along(Some(Route::One0A)).await });

    assert!(incidents.is_ok());
}

#[test]
fn test_path() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let path = block_on(async { client.path(Route::One0A, None).await });

    assert_eq!(path.unwrap().route, Route::One0A);
}

#[test]
fn test_path_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let path = block_on(async {
        client
            .path(Route::One0A, Some(Date::new(2019, 10, 1)))
            .await
    });

    assert_eq!(path.unwrap().route, Route::One0A);
}

#[test]
fn test_route_schedule() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let route_schedule = block_on(async { client.route_schedule(Route::One0A, None, false).await });

    assert_eq!(
        route_schedule.unwrap().name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_route_schedule_with_variations() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let route_schedule = block_on(async { client.route_schedule(Route::One0A, None, true).await });

    assert_eq!(
        route_schedule.unwrap().name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_route_schedule_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let route_schedule = block_on(async {
        client
            .route_schedule(Route::One0A, Some(Date::new(2019, 10, 2)), true)
            .await
    });

    assert_eq!(
        route_schedule.unwrap().name,
        "10A - PENTAGON - HUNTINGTON STA"
    );
}

#[test]
fn test_stop_schedule() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stop_schedule = block_on(async { client.stop_schedule(Stop::new("1001195"), None).await });

    assert_eq!(
        stop_schedule.unwrap().stop.stop.unwrap(),
        Stop("1001195".to_string())
    );
}

#[test]
fn test_stop_schedule_with_date() {
    let client: Client = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let stop_schedule = block_on(async {
        client
            .stop_schedule(Stop::new("1001195"), Some(Date::new(2019, 10, 2)))
            .await
    });

    assert_eq!(
        stop_schedule.unwrap().stop.stop.unwrap(),
        Stop("1001195".to_string())
    );
}
