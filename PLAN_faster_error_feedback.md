# Faster error feedback loop

## Problem

`cargo clippy -p <crate> --no-deps` is the documented check (AGENTS.md) but still
slow on first run per session (cold sccache entries for the dep graph: dioxus,
tw_merge macros, etc). Investigated `mcp__ide__getDiagnostics` as a shortcut.

## Findings

- `mcp__ide__getDiagnostics` only returns diagnostics for files VS Code has
  open/tracked, and only from extensions actually wired to rust-analyzer's
  diagnostics channel. In this workspace it only surfaced `tailwindcss`
  extension warnings (canonical class suggestions), not rust-analyzer
  compile errors — confirmed by injecting a real undefined-fn error into
  `pagination.rs` and seeing zero diagnostics reported.
- Opening files via `code <path>` does make VS Code track them
  (`linesInFile` appears in the response) but rust-analyzer's own
  `checkOnSave` still has to run a real `cargo check`/`clippy` under the
  hood, which is what's actually slow.
- `ps aux` showed rust-analyzer running its own `rustc`/clippy invocation
  (`--crate-name app_domain ... --warn=clippy::pedantic ...`, heavy deny
  list) concurrently with an unrelated `dx serve --platform ios` build.
  Both compete for the same `target/` and sccache, so neither finishes
  fast. This is very likely why diagnostics never landed within the test
  window, not a tool wiring issue.

## Plan

1. Don't run `dx serve` (any platform) and a fresh `cargo clippy`/rust-analyzer
   check back-to-back — let one finish before starting the other, they fight
   over the same target dir + sccache.
2. For a quick single-file/module error check, prefer `cargo check -p <crate>`
   (note: no `--no-deps` flag on `check`, that's clippy-only) over full
   clippy — fewer lints, still catches real compile errors.
3. Treat `mcp__ide__getDiagnostics` as tailwind-class-lint support only in
   this workspace, not a rust-analyzer substitute, until proven otherwise
   with a clean (no concurrent build) test.
4. Re-test `getDiagnostics` for real rust-analyzer errors once no other
   cargo/dx build is running, to confirm whether it's a genuine channel gap
   or just contention.
5. Keep `cargo clippy -p <crate> --no-deps` (background) as the source of
   truth before commit, per AGENTS.md; sccache warms after the first run so
   subsequent checks on the same crate are fast.

## Status

Open — step 4 not yet retried cleanly (blocked by concurrent `dx serve --platform ios` build at time of writing).
