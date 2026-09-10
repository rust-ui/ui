# Plan — Cargo.toml strictness

## Current state (2026-09-10)

- Groups `all` + `pedantic` + `nursery` + `cargo` enabled at `warn`, priority `-2`.
- Safety lints kept at `deny` (unwrap, panic, indexing_slicing, float_cmp, ...).
- `deploy_prod.sh` gate is strict: `cargo clippy --all-targets --all-features -- -D warnings`.
  Local bare `cargo clippy` is NOT equivalent (no `-D warnings`, hits build cache),
  so it can pass while deploy fails. Always reproduce with the deploy command.
- Noisy `nursery` members silenced (group stays `warn`):
  `missing_const_for_fn`, `suboptimal_flops`, `imprecise_flops`,
  `option_if_let_else`, `return_self_not_must_use`.
  Removed ~117 registry errors (469 -> 352). Revisit each per step 6.

## Remaining debt (under `-D warnings`)

| Lint | Where | Fix |
| --- | --- | --- |
| `use_self` | registry, app_routes (bulk) | `cargo clippy --fix` |
| `redundant_else`, `if_not_else` | registry | `cargo clippy --fix` |
| `match_bool`, `match_same_arms` | registry | `cargo clippy --fix` |
| `map_unwrap_or`, `redundant_closure_for_method_calls` | registry | `cargo clippy --fix` |
| `uninlined_format_args`, `manual_midpoint` | registry | `cargo clippy --fix` |
| `enum_variant_names` | app_routes (`ChartRoutes`, `HooksRoutes`) | `allow` with reason: route enum names are the public API |
| `unused_self` | app_routes (`to_route`) | `allow` with reason: kept for method-call ergonomics |

Run the auto-fixer through `rtk proxy` so the RTK hook does not strip `--fix`:
```bash
rtk proxy cargo clippy --fix --workspace --all-targets --all-features --allow-dirty
```

## Priorities

1. Keep current `deny` safety lints green.
2. Reproduce with the deploy command, not bare `cargo clippy`:
   ```bash
   cargo clippy --all-targets --all-features -- -D warnings
   ```
3. Auto-fix mechanical debt, then hand-fix the rest; rerun until zero.
4. Keep `clippy::cargo` green; fix manifest metadata and dependency issues.
5. Burn down `clippy::pedantic` debt by crate.
6. Review `clippy::nursery` suggestions one by one; drop the temporary member
   `allow`s as each area is cleaned.
7. Remove temporary `allow` entries and TODO comments as debt disappears.

## Cargo.toml policy

- Keep lint policy in `[workspace.lints.clippy]`.
- Prefer `deny` for correctness/security; use `warn` for style ratchet.
- Add new `allow` only with a short reason and a removal task.
- `leptos-ui/` is a separate workspace with its own lints; the root
  `cargo clippy` never descends into it. Nothing to exclude.

## Done when

- `fmt`, Clippy (`-D warnings`, all targets, all features), and tests pass workspace-wide.
- No unexplained lint `allow` remains.
- Strict groups enabled without exceptions lacking justification.
