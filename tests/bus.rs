use wmata::{BusClient, RouteID};

#[test]
fn test_constructor() {
    let client = BusClient::new("9e38c3eab34c4e6c990828002828f5ed");

    assert_eq!(client.api_key, "9e38c3eab34c4e6c990828002828f5ed");
}

#[test]
fn test_routes() {
    let client: BusClient = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.routes().is_ok());
}

#[test]
fn test_positions_along() {
    let client: BusClient = "9e38c3eab34c4e6c990828002828f5ed".parse().unwrap();

    assert!(client.positions_along(Some(RouteID::One0A), None, None, None).is_ok());
}