# Architecture

## Why this exists

Geometry was written twice. `h-ua-bot` has `h-ua-core::geo` (coordinates to an H3 resolution 6 cell, distance, the snap tolerance). `prism-signal` has `prism-signal-geo` (polygon covers, discs, neighbour expansion, a grid profile). They agree on H3 and disagree on everything else, including the resolution: the `0x1` profile names 7 and 8, while the Hub's alert contract fixes 6. Nothing owns that disagreement.

Prism Geo owns it. A client that turns a position into a cell does so here, so precise coordinates need to exist only until the cell is derived, at the edge that received them.

## Shape

```text
caller (names the resolution) ──▶ Grid ──▶ Cell ──▶ text id / centre / distance
```

The crate is pure. It reads no clock, file, or network, and holds no state.

## Dependency direction

- Prism Geo depends on no other Prism component.
- `prism-signal`, `prism-bot` and client products depend on Prism Geo, never the reverse.
- Prism Geo never learns what a cell is used for. Words such as alert, subscriber or threat do not appear in it.

## What moves here

| From | What | State |
| --- | --- | --- |
| `h-ua-bot` `h-ua-core::geo` | `Cell::around`, parse, centre, distance, snap tolerance | done here; `h-ua-bot` switches in its own change |
| `prism-signal` `prism-signal-geo` | polygon cover, disc, expand | **not moved**. It is built on `prism-signal-core` types (`Geometry`, `CellId`, `CellSet`) that fusion, bridge, protocol and runtime share, so it moves in its own change after those types are placed |

## Invariants

1. A cell is derived from a position and the position is not kept.
2. Coordinates outside their range are rejected, never wrapped into a cell somewhere else.
3. A cell parses only on a grid of its own resolution.
4. Equal input gives equal output.
