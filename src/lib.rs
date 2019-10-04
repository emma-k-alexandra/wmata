//! wmata is a simple interface to the [WMATA](https://wmata.com)'s API.
//!
//! # Design
//! wmata provides two main interfaces: [`BusClient`] and [`RailClient`].
//!
//! ## [`BusClient`]
//! The interface to WMATA's MetroBus related methods
//!
//! ## [`RailClient`]
//! The interface to WMATA's MetroRail related methods
//!

pub mod bus;
pub mod rail;

pub mod error;
pub mod traits;
mod types;

pub use bus::client::Client as BusClient;
pub use rail::client::Client as RailClient;

pub use bus::route::RouteID;
pub use rail::line::LineCode;
pub use rail::station::StationCode;
