use tokio_test::block_on;
use wmata::{MetroBus, Route};

#[test]
fn test_constructor() {
    let client = MetroBus::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_routes() {
    let client: MetroBus = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let routes = block_on(async { client.routes().await });

    assert!(routes.is_ok());
}

#[test]
fn test_positions_along() {
    let client: MetroBus = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let positions = block_on(async { client.positions_along(Some(Route::One0A), None).await });

    assert!(positions.is_ok());
}

#[test]
fn test_path() {
    let client: MetroBus = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();
    let route_schedule = block_on(async { client.route_schedule(Route::A2, None, false).await });

    assert!(route_schedule.is_ok());
}
