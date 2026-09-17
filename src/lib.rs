//! # iso-286
//!
//! {{ env!("CARGO_PKG_DESCRIPTION") }}
//!
//! Originally written for a calculator on [Engineering Tools](https://bell-jamie.github.io/)
//!
//! ## Usage
//!
//! ```
//! use iso_286::{limits, grades, hole_deviations, shaft_deviations};
//!
//! let tol = limits(10.0, "H7").unwrap();
//! let g = grades();
//! let holes = hole_deviations();
//! let shafts = shaft_deviations();
//! ```
//!
//! ## References
//!
//! - [ISO Standards](https://www.iso.org/obp/ui)
//! - [ISO 286-1:2010](https://www.iso.org/obp/ui/#iso:std:iso:286:-1:ed-2:v1:en)
//! - [ISO 286-2:2010](https://www.iso.org/obp/ui/#iso:std:iso:286:-2:ed-2:v1:en)

mod lookup;
mod tables;
mod wasm;

pub use lookup::{
    Error, Feature, Match, Tolerance, find_preferred, grades, hole_deviations,
    hole_preferred_tolerances, limits, list_closest, list_preferred, shaft_deviations,
    shaft_preferred_tolerances,
};
