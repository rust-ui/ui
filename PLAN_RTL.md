# rust-ui Gap Analysis vs shadcn/ui Changelog

> **When implementing any item below:**
> - Update `public/docs/changelog.md` with a user-facing entry at the top.
> - Check shadcn's tests for the equivalent feature: `packages/shadcn/src/utils/transformers/` and `packages/shadcn/src/commands/` in the shadcn repo.

Only features confirmed in shadcn source code are listed, with direct file references.

---

## 1. RTL — CSS Class Transform on `ui add`

**Shadcn source**: `packages/shadcn/src/utils/transformers/transform-rtl.ts`

When `rtl = true` in config, shadcn transforms physical CSS classes to logical equivalents when writing component files. We have no equivalent.

### 1a. `rtl` field in `UiConfig`
**File**: `crates/ui-cli/src/command_init/config.rs`

Add `rtl: bool` (default `false`, `serde(default)`).

### 1b. `--rtl` / `--no-rtl` flags on `ui init`
**Shadcn ref**: `init.ts` line `.option("--rtl", "enable RTL support.")`
**File**: `crates/ui-cli/src/command_init/_init.rs`

Add `--rtl` flag + prompt (`? Enable RTL support? (y/N)`) after color prompts. Skip prompt with `--yes` (defaults to `false`).

### 1c. RTL transform applied during `ui add`
**Shadcn ref**: `transform-rtl.ts` — `RTL_MAPPINGS`, `RTL_TRANSLATE_X_MAPPINGS`, `RTL_REVERSE_MAPPINGS`, `RTL_SWAP_MAPPINGS`
**File**: `crates/ui-cli/src/command_add/registry.rs` → `then_write_to_file_to()`

New function `apply_rtl_transforms(content: &str) -> String` called before `write_component_file()` when `rtl = true`.

Full mapping table (in application order — order matters to avoid partial matches):

| Physical | Logical |
|---|---|
| `-ml-` | `-ms-` |
| `-mr-` | `-me-` |
| `ml-` | `ms-` |
| `mr-` | `me-` |
| `pl-` | `ps-` |
| `pr-` | `pe-` |
| `-left-` | `-start-` |
| `-right-` | `-end-` |
| `left-` | `start-` |
| `right-` | `end-` |
| `inset-l-` | `inset-inline-start-` |
| `inset-r-` | `inset-inline-end-` |
| `rounded-tl-` | `rounded-ss-` |
| `rounded-tr-` | `rounded-se-` |
| `rounded-bl-` | `rounded-es-` |
| `rounded-br-` | `rounded-ee-` |
| `rounded-l-` | `rounded-s-` |
| `rounded-r-` | `rounded-e-` |
| `border-l-` | `border-s-` |
| `border-r-` | `border-e-` |
| `border-l` | `border-s` |
| `border-r` | `border-e` |
| `text-left` | `text-start` |
| `text-right` | `text-end` |
| `scroll-ml-` | `scroll-ms-` |
| `scroll-mr-` | `scroll-me-` |
| `scroll-pl-` | `scroll-ps-` |
| `scroll-pr-` | `scroll-pe-` |
| `float-left` | `float-start` |
| `float-right` | `float-end` |
| `clear-left` | `clear-start` |
| `clear-right` | `clear-end` |
| `origin-top-left` | `origin-top-start` |
| `origin-top-right` | `origin-top-end` |
| `origin-bottom-left` | `origin-bottom-start` |
| `origin-bottom-right` | `origin-bottom-end` |
| `origin-left` | `origin-start` |
| `origin-right` | `origin-end` |

Additional transforms (add `rtl:` variant, not direct replace):
- `translate-x-{n}` → also add `rtl:-translate-x-{n}`
- `-translate-x-{n}` → also add `rtl:translate-x-{n}`
- `space-x-` → also add `rtl:space-x-reverse`
- `divide-x-` → also add `rtl:divide-x-reverse`
- `cursor-w-resize` ↔ `cursor-e-resize` swap

### 1d. Inline-start / inline-end side styles in components
**Shadcn ref**: `transform-rtl.ts` — `RTL_LOGICAL_SIDE_SLIDE_MAPPINGS`

Add `data-[side=inline-start]` and `data-[side=inline-end]` animation classes alongside existing `data-[side=left]`/`data-[side=right]` in these component files:
- `app_crates/registry/src/ui/tooltip.rs`
- `app_crates/registry/src/ui/popover.rs`
- `app_crates/registry/src/ui/context_menu.rs`
- `app_crates/registry/src/ui/dropdown_menu.rs`
- `app_crates/registry/src/ui/hover_card.rs`
- `app_crates/registry/src/ui/menubar.rs`
- `app_crates/registry/src/ui/select.rs`

Pattern:
```
data-[side=left]:slide-in-from-right-2
data-[side=right]:slide-in-from-left-2
+ data-[side=inline-start]:slide-in-from-right-2
+ data-[side=inline-end]:slide-in-from-left-2
```

### 1e. RTL docs page
**File**: `public/docs/rtl.md`

New doc page explaining: enable via `rtl = true` in `ui_config.toml` or `ui init --rtl`, what transforms are applied, how to use `DirectionProvider`.

---

## 2. `ui info` — Show `rtl` field

**Shadcn source**: `packages/shadcn/src/commands/info.ts`

Shadcn's `info` exposes the full config including `rtl`. Ours (`command_info/_info.rs`) outputs `base_color`, `base_path`, `workspace`, `installed` but **not** `rtl`.

**File**: `crates/ui-cli/src/command_info/_info.rs`

Add `rtl: bool` to `InfoData` struct and display it in both human and JSON output.

---

## 3. `ui docs <component>` — Component-specific docs

**Shadcn source**: `packages/shadcn/src/commands/docs.ts`

Shadcn's `docs` command takes component names and prints their specific doc/API links:
```
shadcn docs button
→ button
  - docs     https://ui.shadcn.com/docs/components/button
  - api      https://www.radix-ui.com/...
```

Our `ui docs` (`command_docs/_docs.rs`) just opens `https://rust-ui.com` in the browser — no component argument.

**File**: `crates/ui-cli/src/command_docs/_docs.rs`

Add optional `<component>` argument. When provided, print the direct URL to `https://rust-ui.com/docs/components/{component}` instead of opening the homepage. With `--json` flag, output JSON.

---

## Implementation Order

- [ ] 1a — `rtl: bool` in `UiConfig`
- [ ] 1b — `--rtl` flag + prompt in `ui init`
- [ ] 1c — `apply_rtl_transforms()` in `registry.rs`
- [ ] 1d — inline-start/end classes in 7 component files
- [ ] 1e — `public/docs/rtl.md`
- [ ] 2  — `rtl` field in `ui info` output
- [ ] 3  — `ui docs <component>` argument

---

## Edge Cases & Tests

### 1a — `rtl` in `UiConfig`
- Old `ui_config.toml` without `rtl` field → must deserialize without error, default `false`
- Round-trip: serialize → deserialize preserves value

```
rtl_defaults_to_false_when_missing_from_toml
rtl_true_parses_correctly
rtl_round_trips_through_toml_serialization
```

---

### 1b — `--rtl` / `--no-rtl` flags
- `--rtl` → `rtl = true`, no prompt
- `--no-rtl` → `rtl = false`, no prompt
- Neither + `--yes` → default `false`, no prompt

```
command_init_rtl_flag_is_registered
command_init_no_rtl_flag_is_registered
command_init_rtl_defaults_false_with_yes_flag
```

---

### 1c — `apply_rtl_transforms()`

**Ordering** (critical — wrong order causes partial matches):
- `-ml-` must match before `ml-`
- `rounded-tl-` before `rounded-l-`
- `border-l-` before `border-l` (bare)
- `origin-top-left` before `origin-left`

**Must NOT transform** (partial word matches):
- `email` contains `ml` → untouched
- `outline` contains `right` → untouched

**Tailwind variant prefixes must be preserved**:
- `sm:ml-4` → `sm:ms-4`
- `hover:ml-4` → `hover:ms-4`
- `dark:ml-4` → `dark:ms-4`

**Arbitrary values**:
- `ml-[24px]` → `ms-[24px]`
- `left-[var(--x)]` → `start-[var(--x)]`

**Additive transforms (`rtl:` variants)**:
- `space-x-4` → keep + add `rtl:space-x-reverse` (only if not already present)
- `divide-x-2` → keep + add `rtl:divide-x-reverse`
- `translate-x-4` → keep + add `rtl:-translate-x-4`
- Already has `rtl:space-x-reverse` → do NOT add a duplicate

**Idempotency** (run twice → same output, no double-transform):
- `ms-4` must NOT become `me-4` on second pass

```
transforms_ml_to_ms
transforms_negative_ml_to_negative_ms
transforms_rounded_tl_before_rounded_l
transforms_border_l_bare_and_with_value
transforms_origin_top_left_before_origin_left
transforms_text_left_to_text_start
preserves_sm_responsive_prefix
preserves_hover_variant_prefix
handles_arbitrary_value_brackets
does_not_match_partial_word_email
adds_rtl_space_x_reverse
does_not_duplicate_rtl_space_x_reverse
idempotent_on_already_transformed_content
empty_input_returns_empty
```

**Integration with `then_write_to_file_to`**:
- `rtl = false` → file written unchanged
- `rtl = true` → transforms applied before write
- `WriteOutcome::Skipped` → no transform (file not touched)

```
rtl_false_writes_content_unchanged
rtl_true_applies_transforms_before_write
rtl_true_skipped_file_not_transformed
```

---

### 1d — Inline-start/end in 7 components
- Classes added alongside `data-[side=left/right]`, not replacing them
- No duplicate classes if already present

```
// per component:
popover_has_inline_start_and_end_slide_classes
tooltip_has_inline_start_and_end_slide_classes
// ... same pattern for all 7
```

---

### 2 — `ui info` RTL field
- `rtl = true` shown in human + JSON output
- `rtl = false` shown (not hidden)

```
info_includes_rtl_true
info_includes_rtl_false
info_json_includes_rtl_field
```

---

### 3 — `ui docs <component>`
- No argument → opens homepage (existing behavior unchanged)
- Known component → prints `https://rust-ui.com/docs/components/{name}`
- Underscores normalized to hyphens: `button_group` → `button-group` in URL
- `--json` → `{ "component": "button", "url": "..." }`
- Multiple components → one URL per line

```
docs_no_arg_opens_homepage
docs_with_component_prints_url
docs_normalizes_underscore_to_hyphen
docs_json_flag_outputs_json
docs_multiple_components_prints_all
```
