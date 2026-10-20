// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

use h3o::Resolution;
use thiserror::Error;

use crate::cell::{Cell, CellError};

/// A resolution is not one the grid has.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
#[error("grid resolution must be 0 to 15, got {0}")]
pub struct GridError(pub u8);

/// The grid a deployment works on: H3 at one resolution.
///
/// The resolution is chosen by whoever owns the contract (for broadcasts, `0x1`; for a product, the
/// product), and is passed in. Nothing here has a default.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Grid {
    resolution: Resolution,
}

impl Grid {
    /// A grid at an H3 resolution from 0 to 15.
    pub fn new(resolution: u8) -> Result<Self, GridError> {
        Resolution::try_from(resolution)
            .map(|resolution| Self { resolution })
            .map_err(|_| GridError(resolution))
    }

    /// The H3 resolution, 0 to 15.
    pub fn resolution(&self) -> u8 {
        u8::from(self.resolution)
    }

    /// The cell around a position. The coordinates are not kept.
    pub fn cell_around(&self, lat: f64, lon: f64) -> Result<Cell, CellError> {
        Cell::around(*self, lat, lon)
    }

    /// Reads a cell identifier, accepting only a cell of this grid's resolution.
    pub fn parse(&self, value: &str) -> Result<Cell, CellError> {
        Cell::parse(*self, value)
    }

    pub(crate) fn h3(&self) -> Resolution {
        self.resolution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_grid_accepts_only_h3_resolutions() {
        assert_eq!(Grid::new(6).unwrap().resolution(), 6);
        assert_eq!(Grid::new(0).unwrap().resolution(), 0);
        assert_eq!(Grid::new(15).unwrap().resolution(), 15);
        assert_eq!(Grid::new(16), Err(GridError(16)));
    }
}
