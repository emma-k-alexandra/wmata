//! wmata is a simple interface to the [WMATA](https://wmata.com)'s API.
//!
//! # Design
//! wmata provides two main interfaces: [`MetroBus`] and [`MetroRail`].
//!
//! ## [`MetroBus`]
//! The interface to WMATA's MetroBus related methods
//!
//! ## [`MetroRail`]
//! The interface to WMATA's MetroRail related methods
//!

pub mod bus;
pub mod rail;

pub mod error;
pub mod traits;
pub mod types;

pub use bus::client::Client as MetroBus;
pub use rail::client::Client as MetroRail;

pub use types::RadiusAtLatLong;

pub use bus::route::Route;
pub use bus::stop::Stop;
pub use rail::line::Line;
pub use rail::station::Station;
