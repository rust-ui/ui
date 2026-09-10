---
name: strict-clippy-burndown
description: Get a Rust workspace to pass strict clippy (`-D warnings`) incrementally, using one full-project scan plus a compact tracker. Reusable across projects.
---

# Strict clippy burn-down

Goal: `cargo clippy --all-targets --all-features -- -D warnings` green, workspace-wide,
reached in controlled batches instead of one unreviewable mega-diff.

## Rules

- **Scan the whole project once.** One clippy run over the entire workspace produces one
  verbose dump (`CLIPPY_FULL_DUMP.txt`, often 10k+ lines). Never scan crate-by-crate.
- `cargo clippy --fix` is a **no-op** in macro-heavy crates (Leptos `view!`, Dioxus
  `rsx!`): the spans sit inside proc-macro output and are emitted `MaybeIncorrect`, so
  they are never auto-applied. Assume every fix is manual.
- Bare `cargo clippy` can show nothing when the workspace sets
  `[workspace.lints.rust] warnings = "allow"`. That policy suppresses clippy too.
- `-- -D warnings` turns lints into errors, which **halts at the first failing crate**,
  so dependent crates never get scanned. For the scan use `--force-warn <group>`
  instead: lints stay warnings, the build does not stop, every crate reports in one pass.
- If an RTK (or similar) shell hook is installed, it can strip `--fix` and mangle
  `-- -D warnings`. Run clippy through `rtk proxy` (or bypass the hook) so args survive.
- `touch` a source file in every crate before scanning; a warm clippy cache emits zero
  diagnostics.

## Scan command

```bash
touch src/main.rs src/lib.rs */src/lib.rs **/src/lib.rs 2>/dev/null
rtk proxy cargo clippy --all-targets --all-features \
  -- --force-warn clippy::all --force-warn clippy::pedantic \
     --force-warn clippy::nursery --force-warn clippy::cargo \
  > CLIPPY_FULL_DUMP.txt 2>&1
```

## Compact tracker

The dump is too big to keep in context. Reduce it to `CLIPPY_TRACKER.md`:
grouped by lint, one `- [ ] path:line:col` per site, count in each `## <lint> (n)`
header, one representative message as an HTML comment.

```bash
python3 __SKILLS_SOP/strict-clippy-burndown.py   # CLIPPY_FULL_DUMP.txt -> CLIPPY_TRACKER.md
```

`CLIPPY_FULL_DUMP.txt` and `CLIPPY_TRACKER.md` are gitignored.

## Working the tracker

- Section index: `grep -n '^## ' CLIPPY_TRACKER.md`.
- Sites for one lint:
  `awk '/^## <lint> /{f=1;next} /^## /{f=0} f&&/^- /{print $4}' CLIPPY_TRACKER.md`
  (the line is `- [ ] path:line:col`, so field 4 is the location).
- Split the debt:
  - **Blockers** = sites for lints NOT `allow`d in `Cargo.toml`. Only these fail `-D warnings`.
  - **Deferred** = sites for lints already `allow`d with a TODO. Not urgent.
- Path **Green gate fast** (default): fix the handful of correctness blockers in code,
  bucket the noisy style blockers as `allow` + `# TODO(strict-clippy)` in `Cargo.toml`,
  gate green in one session. Then remove one `allow` per later pass.
- Path **Clear all**: hand-fix every blocker, add no new `allow`.
- For a subtle lint pull its block on demand: `grep -A20 'path:line' CLIPPY_FULL_DUMP.txt`.
- As sites are fixed, delete their `- [ ]` lines and decrement the section count.
  Regenerate the tracker after each full re-scan.
- Re-scan (full workspace, 1-4 min) only after a batch, never per fix.

## Fix policy

- **Correctness / safety** (`cast_possible_wrap`, `cast_possible_truncation` on real
  data, `significant_drop_tightening`, `wrong_self_convention`, `ref_option`,
  `volatile_composites`, `unnecessary_get_then_check`, ...): fix the code.
- **Public-API shape** (`enum_variant_names` on route enums, `unused_self` on builder
  methods kept for call ergonomics): inline `#[allow(clippy::x)]` with a one-line
  reason comment on the item.
- **Bulk style, macro-body only** (`use_self`, `uninlined_format_args`,
  `default_trait_access`, `redundant_closure_for_method_calls`, `ignored_unit_patterns`,
  `match_same_arms`, ...): add to `[workspace.lints.clippy]` as `allow` with
  `# TODO(strict-clippy)` and the site count. Burn down one entry per later pass.
- After any `Cargo.toml` lint change: `cargo check`, then re-scan.

## Reference: clippy policy this repo runs

Lives in root `Cargo.toml` under `[workspace.lints.*]`. Member crates opt in with
`[lints] workspace = true`. Groups are set with negative `priority` so the explicit
per-lint policy below always wins.

### `[workspace.lints.clippy]` — groups

| Lint group | Level | Priority |
| --- | --- | --- |
| `all` | warn | -2 |
| `pedantic` | warn | -2 |
| `nursery` | warn | -2 |
| `cargo` | warn | -2 |

### `deny` — correctness, panics, safety (never relax)

`unwrap_used`, `expect_used`, `panic`, `todo`, `unimplemented`, `unreachable`,
`indexing_slicing`, `get_unwrap`, `unwrap_in_result`, `panic_in_result_fn`,
`await_holding_lock`, `await_holding_invalid_type`, `dbg_macro`,
`undocumented_unsafe_blocks`, `missing_safety_doc`, `enum_glob_use`, `infinite_loop`,
`large_stack_arrays`, `lossy_float_literal`, `float_cmp`, `mem_forget`, `exit`,
`disallowed_macros`, `disallowed_methods`, `disallowed_types`, `mutex_atomic`,
`as_underscore`.

### `[workspace.lints.rust]` — `deny`

`irrefutable_let_patterns`, `unused_must_use`, `non_ascii_idents`,
`let_underscore_lock`, `unit_bindings`, `macro_use_extern_crate`, `unused_lifetimes`.

`unsafe_code = "forbid"`.

### `[workspace.lints.rustdoc]`

`deny`: `broken_intra_doc_links`, `invalid_html_tags`, `invalid_rust_codeblocks`.
`warn`: `bare_urls`, `unescaped_backticks`.

### Temporary `allow` — each carries a TODO, remove as debt clears

- String building: `str_to_string`, `string_add`, `format_push_string`.
- Determinism: `iter_over_hash_type`.
- Style ratchet: `semicolon_if_nothing_returned`, `cast_possible_truncation`,
  `cast_sign_loss`, `cast_precision_loss`, `cast_lossless`, `redundant_clone`,
  `inefficient_to_string`, `cloned_instead_of_copied`, `manual_let_else`,
  `trivially_copy_pass_by_ref`, `explicit_iter_loop`, `from_over_into`,
  `fallible_impl_from`, `clone_on_ref_ptr`, `needless_borrow`, `needless_pass_by_value`,
  `unused_async`, `doc_markdown`, `unreadable_literal`, `derive_partial_eq_without_eq`,
  `needless_raw_string_hashes`.
- `nursery` noise kept silenced while the group stays `warn`: `missing_const_for_fn`,
  `suboptimal_flops`, `imprecise_flops`, `option_if_let_else`, `return_self_not_must_use`.
- Diagnostics output: `print_stderr` (allow), `print_stdout` (warn, `build.rs` needs it).

### Permanent `allow` — intentional project choices

`module_name_repetitions`, `missing_errors_doc`, `missing_panics_doc`,
`must_use_candidate`, `cognitive_complexity`, `multiple_crate_versions`,
`cargo_common_metadata`, `too_many_lines`, `allow_attributes`,
`allow_attributes_without_reason`.

### `[workspace.lints.rust]` — temporary `allow` (deployment mode)

`warnings`, `dead_code`, `unused_variables`, `unused_mut`, `single_use_lifetimes`,
`unused_qualifications`, `unreachable_pub`, `redundant_lifetimes`,
`trivial_numeric_casts`, `keyword_idents_2024`, `elided_lifetimes_in_paths`,
`redundant_imports`, `meta_variable_misuse`.

### Cargo.toml policy

- Lint policy lives only in `[workspace.lints.*]`.
- `deny` for correctness/security; `warn` for the style ratchet.
- A new `allow` needs a short reason and a removal task.
- A nested workspace (e.g. `leptos-ui/`) has its own lints; the root `cargo clippy`
  never descends into it.

## Done when

`rtk proxy cargo clippy --all-targets --all-features -- -D warnings` returns exit 0
(no `--keep-going` needed once green), and every `allow` added during the burn-down
carries a reason or a removal TODO.
