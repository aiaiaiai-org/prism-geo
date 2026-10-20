# Roadmap

Each step is its own change and is accepted on its own.

1. **Foundation.** Grid, cell, distance, CI (merged).
2. **Strict cell validation CLI.** A non-Rust caller can validate one canonical H3 cell at an explicitly named resolution without leaking it through argv; no Hub integration yet (this change).
3. **`h-ua-bot` switches** from `h-ua-core::geo` to Prism Geo with grid resolution 6 from its configuration; the copy is deleted.
4. **Place the shared geometry types** (`Geometry`, `Position`, `Ring`, `CellId`, `CellSet`) so that covers can move without a cycle. Where they live is decided with `prism-signal`.
5. **Move cover, disc and expand** from `prism-signal-geo`, keeping its tests as the acceptance tests.
6. **Hub integration.** Keep user/workspace subscriptions, auth, alert matching and delivery in Hub. Move H3-specific parsing to a Geo-backed adapter after the executable is packaged in the Hub image and covered by exact-head integration tests. Keep the existing SQL H3 constraint as defense in depth until an explicit schema transition.
7. **A general wire protocol** for non-Rust geo operations, only when proven necessary; separate from the narrow cell-validation CLI.

Precise coordinates must not enter Hub. No migration should silently change resolution 6, accepted API fields, persisted cells, or the current authorization checks.
