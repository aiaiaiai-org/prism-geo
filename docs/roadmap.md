# Roadmap

Each step is its own change and is accepted on its own.

1. **This repository.** Grid, cell, distance, CI.
2. **`h-ua-bot` switches** from `h-ua-core::geo` to Prism Geo with grid resolution 6 from configuration; the copy is deleted.
3. **Place the shared geometry types** (`Geometry`, `Position`, `Ring`, `CellId`, `CellSet`) so that covers can move without a cycle. Where they live is decided with `prism-signal`.
4. **Move cover, disc and expand** from `prism-signal-geo`, keeping its tests as the acceptance tests.
5. **A wire protocol** for callers outside Rust, only when a caller needs it: for example `prism-bot` turning a shared position into a cell before it leaves the transport.

Step 5 is the one that keeps precise coordinates out of client products. It is not started, and `prism-bot` does not depend on this crate.
