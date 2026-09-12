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
git ls-files '*.rs' | xargs touch          # bust the clippy cache in every crate
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
- ALWAYS update `CLIPPY_TRACKER.md` in the SAME batch as the code edits, before
  committing: collapse the finished section to `## <lint> (0) — DONE`, add a
  `- [x] <lint> — <approach>` line under `### Fixed in code`, drop the lint from the
  `### Bucketed to Cargo.toml` summary, refresh the STATUS date. The tracker is the
  only record of intent (it is gitignored, not in diff history) — a stale tracker is
  a silent loss of context for the next session.
- `cargo clippy --fix` CANNOT fix bucket lints in this repo: `[workspace.lints.rust]
  warnings = "allow"` squashes clippy warnings and `cargo fix` does not forward the
  trailing `-- -D warnings`, so `--fix` sees nothing. Hand-edit, or scoped `perl`.
- Re-scan (full workspace, 1-4 min) only after a batch, never per fix.
- One lint per commit during burn-down: `chore(clippy): burn down <lint>`. Keeps each
  diff reviewable and lets a regression be bisected to a single lint.

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

## Reference: starter lint policy

Paste into a fresh client's root `Cargo.toml`, then relax entries as that repo needs.
Member crates opt in with `[lints] workspace = true`. Groups carry negative `priority`
so the explicit per-lint policy always wins. A nested workspace has its own lints;
the root `cargo clippy` never descends into it.

Policy shape:
- **`deny`**: correctness, panics, safety. Never relaxed, even in the burn-down.
- **temporary `allow`**: every entry carries a `# TODO`, removed as debt clears.
- **permanent `allow`**: intentional project choices, no TODO.

```toml
[workspace.lints.clippy]
# Groups at negative priority so per-lint policy below wins.
all = { level = "warn", priority = -2 }
pedantic = { level = "warn", priority = -2 }
nursery = { level = "warn", priority = -2 }
cargo = { level = "warn", priority = -2 }

# deny: correctness / panics / safety — never relax
unwrap_used = "deny"
expect_used = "deny"
panic = "deny"
todo = "deny"
unimplemented = "deny"
unreachable = "deny"
indexing_slicing = "deny"
get_unwrap = "deny"
unwrap_in_result = "deny"
panic_in_result_fn = "deny"
await_holding_lock = "deny"
await_holding_invalid_type = "deny"
dbg_macro = "deny"
undocumented_unsafe_blocks = "deny"
missing_safety_doc = "deny"
enum_glob_use = "deny"
infinite_loop = "deny"
large_stack_arrays = "deny"
lossy_float_literal = "deny"
float_cmp = "deny"
mem_forget = "deny"
exit = "deny"
disallowed_macros = "deny"
disallowed_methods = "deny"
disallowed_types = "deny"
mutex_atomic = "deny"
as_underscore = "deny"

# temporary allow — each needs a # TODO and a removal task
str_to_string = "allow"            # TODO: literal .to_string() -> .to_owned()
string_add = "allow"               # TODO: string-building cleanup
format_push_string = "allow"       # TODO: string-building cleanup
iter_over_hash_type = "allow"      # TODO: make unordered iteration deterministic
semicolon_if_nothing_returned = "allow"
cast_possible_truncation = "allow"
cast_sign_loss = "allow"
cast_precision_loss = "allow"
cast_lossless = "allow"
redundant_clone = "allow"
inefficient_to_string = "allow"
cloned_instead_of_copied = "allow"
manual_let_else = "allow"
trivially_copy_pass_by_ref = "allow"
explicit_iter_loop = "allow"
from_over_into = "allow"
fallible_impl_from = "allow"
clone_on_ref_ptr = "allow"
needless_borrow = "allow"
needless_pass_by_value = "allow"
unused_async = "allow"
doc_markdown = "allow"             # TODO: clean doc markup, then re-enable
unreadable_literal = "allow"
derive_partial_eq_without_eq = "allow"
needless_raw_string_hashes = "allow"
# nursery noise, silenced while the group stays warn
missing_const_for_fn = "allow"
suboptimal_flops = "allow"
imprecise_flops = "allow"
option_if_let_else = "allow"
return_self_not_must_use = "allow"
print_stdout = "warn"             # build.rs needs Cargo directive output
print_stderr = "allow"           # TODO: structured logging

# permanent allow — intentional project choices
module_name_repetitions = "allow"
missing_errors_doc = "allow"
missing_panics_doc = "allow"
must_use_candidate = "allow"
cognitive_complexity = "allow"
multiple_crate_versions = "allow"
cargo_common_metadata = "allow"
too_many_lines = "allow"
allow_attributes = "allow"
allow_attributes_without_reason = "allow"

[workspace.lints.rust]
warnings = "allow"               # TODO: deployment mode; drop to surface all debt
unsafe_code = "forbid"
dead_code = "allow"
unused_variables = "allow"
unused_mut = "allow"
irrefutable_let_patterns = "deny"
unused_must_use = "deny"
non_ascii_idents = "deny"
let_underscore_lock = "deny"
unit_bindings = "deny"
macro_use_extern_crate = "deny"
unused_lifetimes = "deny"
single_use_lifetimes = "allow"
unused_qualifications = "allow"
unreachable_pub = "allow"
redundant_lifetimes = "allow"
trivial_numeric_casts = "allow"
keyword_idents_2024 = "allow"
elided_lifetimes_in_paths = "allow"
redundant_imports = "allow"
meta_variable_misuse = "allow"

[workspace.lints.rustdoc]
broken_intra_doc_links = "deny"
invalid_html_tags = "deny"
invalid_rust_codeblocks = "deny"
bare_urls = "warn"
unescaped_backticks = "warn"
```

The burn-down bucket (style lints trapped in macro bodies) goes in a separate block
below this one, each `= "allow"` with `# TODO(strict-clippy)` and a site count.

## Post-gate hardening

After strict Clippy is green, ratchet compiler and dependency policy separately:

1. Remove `[workspace.lints.rust] warnings = "allow"`, then remove broad Rust allows
   (`dead_code`, `unused_variables`, `unused_mut`, and similar) one at a time. Fix
   real warnings at source; use narrow, reasoned local exceptions only for generated
   or intentionally platform-specific code.
2. Audit duplicate dependencies with `cargo tree -d --workspace --all-features`.
   Align direct pins and features where APIs permit, then re-run checks and tests.
   Keep `multiple_crate_versions` only for unavoidable transitive or platform-stack
   conflicts, with the affected dependency families documented.
3. Test suspected macro false positives by removing the lint exception and running
   the strict gate. Restore a narrow documented exception only when the diagnostic
   comes exclusively from an external macro expansion and no source-level fix exists.

## Done when

`rtk proxy cargo clippy --all-targets --all-features -- -D warnings` returns exit 0
(no `--keep-going` needed once green), and every `allow` added during the burn-down
carries a reason or a removal TODO.
