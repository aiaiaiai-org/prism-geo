// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

//! Deterministic grid cells for Prism.
//!
//! This crate is the one place that turns coordinates into grid cells and measures distance on
//! the globe. It is pure: no I/O, no clock, no state. It knows nothing about what a cell is used
//! for, and it hard-codes no resolution. A [`Grid`] names the resolution, and the caller reads it
//! from configuration bound to the contract that defines it.

mod cell;
mod distance;
mod grid;

pub use cell::{Cell, CellError};
pub use distance::{EARTH_RADIUS_KM, haversine_km};
pub use grid::{Grid, GridError};
