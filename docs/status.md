# Implementation status

## Exists

- `Grid`: an H3 resolution from 0 to 15, named by the caller.
- `Cell`: derived from a position with range checks, parsed back only at the grid's resolution, centre, distance, largest centre-to-vertex distance.
- `haversine_km`.
- Full CI: format, clippy, tests, docs, and the 1.85 MSRV.

## Does not exist

- Polygon cover, disc and neighbour expansion. They live in `prism-signal-geo` and are not moved yet.
- A wire protocol. A stateless `prism-geo.v1` request/response for non-Rust callers is not designed.
- Any caller. `h-ua-bot` and `prism-signal` still use their own copies.
