# Implementation status

## Exists

- `Grid`: an H3 resolution from 0 to 15, named by the caller.
- `Cell`: derived from a position with range checks, parsed back only at the grid's resolution, centre, distance, largest centre-to-vertex distance.
- `haversine_km`.
- `prism-geo validate-cell <resolution>`: one canonical H3 cell read from bounded stdin and verified using the same Rust Grid. The CLI never receives a coordinate or identity.
- Full CI: format, clippy, tests, docs, and the 1.85 MSRV.

## Does not exist

- Polygon cover, disc and neighbour expansion. They live in `prism-signal-geo` and are not moved yet.
- A general JSON wire protocol or a location/geocoding HTTP service.
- An integrated caller. `h-ua-bot`, `prism-signal` and the Ruby Hub still have their current adapters or copies. CLI availability alone does not migrate them.
