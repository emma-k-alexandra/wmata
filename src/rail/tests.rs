#[cfg(test)]
use super::*;

#[test]
fn test_lines() {
    let rail = Rail {
        api_key: "9e38c3eab34c4e6c990828002828f5ed".to_string(),
    };

    rail.lines(|x| assert_eq!(x.lines.len(), 6))
}

#[test]
fn test_entrances() {
    let rail = Rail {
        api_key: "9e38c3eab34c4e6c990828002828f5ed".to_string(),
    };

    rail.entrances(1.0, 1.0, 1.0, |x| assert_eq!(x.entrances.len(), 0))
}

#[test]
fn test_stations() {
    let rail = Rail {
        api_key: "9e38c3eab34c4e6c990828002828f5ed".to_string(),
    };

    rail.stations(Some(LineCode::Red), |x| assert_eq!(x.stations.len(), 27))
}

#[test]
fn test_all_stations() {
    let rail = Rail {
        api_key: "9e38c3eab34c4e6c990828002828f5ed".to_string(),
    };

    rail.stations(None, |x| assert_eq!(x.stations.len(), 95))
}
