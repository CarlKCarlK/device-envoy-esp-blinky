# Project Layout

This repository has two ways to use blinky:

- `src/main.rs`: default quick-start for `esp32-c6-devkitc-1-n8`
- `examples/<chip>/<board>/blinky.rs`: board-specific variants copied from `device-envoy`

The example files are generated copies from:

- `../device-envoy/crates/device-envoy-esp/examples/*/*/blinky.rs`

Do not hand-edit files under `examples/`; re-sync them instead.

## Sync Command

```bash
./scripts/sync-examples.sh
```

That command updates:

- `examples/*/*/blinky.rs`
- generated `[[example]]` entries in `Cargo.toml`
- `projects/board-matrix.md`
