# icons

## When bumping the version

1. Update `Cargo.toml`
2. `cargo update` in workspace root
3. Commit + push `crates/icons` changes
4. `cargo publish` — wait for crates.io to confirm the version is live
5. `cargo update icons` + commit + push in each `crates/_starters/*` submodule
6. Update submodule refs in parent repo
