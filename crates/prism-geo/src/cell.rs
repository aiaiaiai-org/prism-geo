// © 2026 aiaiaiai · aiaiaiai.org
// SPDX-License-Identifier: Apache-2.0

use std::fmt;

use h3o::{CellIndex, LatLng};
use thiserror::Error;

use crate::distance::haversine_km;
use crate::grid::Grid;

/// A position or a cell could not be read.
#[derive(Clone, Copy, Debug, Eq, Error, PartialEq)]
pub enum CellError {
    /// Latitude or longitude is not a finite value inside its range.
    #[error("invalid coordinates")]
    InvalidCoordinates,
    /// A cell identifier does not parse, or is not of the grid's resolution.
    #[error("invalid cell")]
    InvalidCell,
}

/// A grid cell. It is all that is kept about where a position was.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct Cell {
    grid: Grid,
    index: CellIndex,
}

impl Cell {
    pub(crate) fn around(grid: Grid, lat: f64, lon: f64) -> Result<Self, CellError> {
        // The grid library only rejects non-finite values and wraps everything else, so a
        // latitude of 200 would become a cell somewhere. The range is checked here.
        if !(-90.0..=90.0).contains(&lat) || !(-180.0..=180.0).contains(&lon) {
            return Err(CellError::InvalidCoordinates);
        }
        let point = LatLng::new(lat, lon).map_err(|_| CellError::InvalidCoordinates)?;
        Ok(Self {
            grid,
            index: point.to_cell(grid.h3()),
        })
    }

    pub(crate) fn parse(grid: Grid, value: &str) -> Result<Self, CellError> {
        let index: CellIndex = value.parse().map_err(|_| CellError::InvalidCell)?;
        if index.resolution() == grid.h3() {
            Ok(Self { grid, index })
        } else {
            Err(CellError::InvalidCell)
        }
    }

    /// The grid this cell belongs to.
    pub fn grid(&self) -> Grid {
        self.grid
    }

    /// The centre of the cell as `(lat, lon)` in degrees.
    pub fn center(&self) -> (f64, f64) {
        let centre = LatLng::from(self.index);
        (centre.lat(), centre.lng())
    }

    /// Distance from the cell's centre to a point, in km.
    pub fn distance_km(&self, lat: f64, lon: f64) -> f64 {
        let (own_lat, own_lon) = self.center();
        haversine_km(own_lat, own_lon, lat, lon)
    }

    /// The largest distance from the cell's centre to any of its vertices, in km.
    ///
    /// Add it to a reach when matching, so rounding a position to its cell never hides something
    /// that is inside the reach.
    pub fn max_center_to_vertex_km(&self) -> f64 {
        self.index
            .boundary()
            .iter()
            .map(|vertex| self.distance_km(vertex.lat(), vertex.lng()))
            .fold(0.0, f64::max)
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.index.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const KYIV: (f64, f64) = (50.4501, 30.5234);

    fn grid(resolution: u8) -> Grid {
        Grid::new(resolution).unwrap()
    }

    #[test]
    fn a_cell_holds_its_position_and_forgets_the_exact_point() {
        let cell = grid(6).cell_around(KYIV.0, KYIV.1).unwrap();
        let (lat, lon) = cell.center();
        assert!(haversine_km(lat, lon, KYIV.0, KYIV.1) < cell.max_center_to_vertex_km());
        assert_ne!((lat, lon), KYIV);
    }

    #[test]
    fn the_same_position_gives_the_same_cell() {
        let grid = grid(6);
        assert_eq!(
            grid.cell_around(KYIV.0, KYIV.1),
            grid.cell_around(KYIV.0, KYIV.1)
        );
    }

    #[test]
    fn resolution_six_is_the_documented_size() {
        // About 3 km across, so a reach of 5 to 20 km is resolved.
        let reach = grid(6)
            .cell_around(KYIV.0, KYIV.1)
            .unwrap()
            .max_center_to_vertex_km();
        assert!((3.0..4.0).contains(&reach), "{reach}");
    }

    #[test]
    fn a_cell_round_trips_through_text() {
        let grid = grid(6);
        let cell = grid.cell_around(KYIV.0, KYIV.1).unwrap();
        assert_eq!(grid.parse(&cell.to_string()), Ok(cell));
    }

    #[test]
    fn a_cell_reads_back_as_lowercase_hex() {
        let text = grid(6).cell_around(KYIV.0, KYIV.1).unwrap().to_string();
        assert_eq!(text.len(), 15);
        assert!(text.starts_with("86"));
        assert!(
            text.chars()
                .all(|c| c.is_ascii_hexdigit() && !c.is_ascii_uppercase())
        );
    }

    #[test]
    fn only_cells_of_the_grids_resolution_parse() {
        let coarse = grid(3).cell_around(KYIV.0, KYIV.1).unwrap().to_string();
        assert_eq!(grid(6).parse(&coarse), Err(CellError::InvalidCell));
        assert_eq!(grid(6).parse("not a cell"), Err(CellError::InvalidCell));
    }

    #[test]
    fn invalid_coordinates_are_rejected() {
        for (lat, lon) in [
            (91.0, 0.0),
            (0.0, 181.0),
            (f64::NAN, 0.0),
            (0.0, f64::INFINITY),
        ] {
            assert_eq!(
                grid(6).cell_around(lat, lon),
                Err(CellError::InvalidCoordinates)
            );
        }
    }

    #[test]
    fn the_snap_tolerance_covers_the_whole_cell() {
        let cell = grid(6).cell_around(KYIV.0, KYIV.1).unwrap();
        let reach = cell.max_center_to_vertex_km();
        let index: CellIndex = cell.to_string().parse().unwrap();
        for vertex in index.boundary().iter() {
            assert!(cell.distance_km(vertex.lat(), vertex.lng()) <= reach + 1e-9);
        }
    }
}
