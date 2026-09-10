# Plan — Cargo.toml strictness

## Priorities

1. Keep current `deny` safety lints green.
2. Run fast loop:
   ```bash
   cargo clippy
   ```
3. Fix every Clippy error/warning; rerun until zero.
4. Re-enable `clippy::cargo` first; fix manifest metadata and dependency issues.
5. Re-enable `clippy::pedantic`; fix API/style debt by crate.
6. Re-enable `clippy::nursery` last; review each suggestion manually.
7. Remove temporary `allow` entries and TODO comments as debt disappears.

## Cargo.toml policy

- Keep lint policy in `[workspace.lints.clippy]`.
- Prefer `deny` for correctness/security; use `warn` for style ratchet.
- Add new `allow` only with short reason and removal task.
- CI can later expand command to workspace, all targets, and all features.

## Done when

- `fmt`, Clippy, and tests pass workspace-wide.
- No unexplained lint `allow` remains.
- Strict groups enabled without exceptions lacking justification.
