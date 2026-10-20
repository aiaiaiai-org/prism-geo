# prism-geo

Prism Geo is the one place in Prism that turns a position into a grid cell and measures distance on the globe.

**coordinates in → cell out → coordinates forgotten**

## What Prism Geo owns

- the grid: H3 at one resolution that the caller names;
- coordinates to a cell, with range checks that the grid library does not do;
- reading a cell identifier back, accepting only the grid's resolution;
- great-circle distance;
- the largest centre-to-vertex distance of a cell, so rounding a position to its cell never hides a nearby threat;
- strict canonical cell validation for non-Rust clients via a small stdin CLI.

Prism Geo does not own what a cell means, who is in it, a resolution of its own, persistence, transports, or accounts. It hard-codes no resolution: the owner of each contract (`0x1` for broadcasts, a product for its own grid) chooses it and passes it in.

## Use

```rust
use prism_geo::Grid;

let grid = Grid::new(6)?;                        // about 3 km across
let cell = grid.cell_around(50.4501, 30.5234)?;  // the coordinates are not kept
assert_eq!(grid.parse(&cell.to_string())?, cell);
```

For an external caller that already has an H3 cell, the `prism-geo` executable
provides a narrow validation boundary, without requiring a Rust binding:

```sh
printf '860335a97ffffff\n' | cargo run --quiet -p prism-geo --bin prism-geo -- validate-cell 6
# valid
```

The caller supplies the resolution explicitly (0–15). The cell is read from
**stdin, never argv**; the process prints `valid` and exits 0 on success.
Malformed requests produce `invalid_request` on stderr and exit 2; a
noncanonical, malformed or wrong-resolution cell produces `invalid_cell` on
stderr and exits 2. No error echoes the cell. Each invocation accepts just one
cell (optional single trailing LF); input is length-bounded. This is not a
geocoding service or general JSON wire protocol.

## Documentation

| Document | Purpose |
| --- | --- |
| [`architecture`](docs/architecture.md) | Ownership, what moves here, and from where |
| [`status`](docs/status.md) | What exists and what does not |
| [`roadmap`](docs/roadmap.md) | Order of the moves |

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features -- -D warnings
cargo test --locked --workspace --all-features
```

<!-- © 2026 aiaiaiai · aiaiaiai.org -->
