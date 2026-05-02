# /create Page & Preset Gap Analysis vs shadcn/ui

> **When implementing any item below:**
> - Update `public/docs/changelog.md` with a user-facing entry at the top.
> - Check shadcn's tests for the equivalent feature: `packages/shadcn/src/commands/` and `packages/shadcn/src/preset/` in the shadcn repo.

Features confirmed in shadcn source code only.

## What we already have
- `/create` page with `theme_picker`, `radius_picker`, `color_theme_picker`, `font_picker`, `customizer`
- `preset.rs` — encode/decode preset codes (base62, version `'a'`, 15 bits)
- `?preset=<code>` query param on `/create` — share/restore state
- Preset URL generation

---

## 1. `ui init --preset <code>`

**Shadcn source**: `packages/shadcn/src/commands/init.ts`
```
.option("-p, --preset [name]", "use a preset configuration")
```

During `ui init`, accept a `--preset <code>` flag. Decode it via `decode_preset()` and pre-fill `base_color`, `color_theme`, font (if we add font to config), radius (if we add radius to config) — skipping their individual prompts.

**Files to touch**:
- `crates/ui-cli/src/command_init/_init.rs` — add `--preset` arg, decode + apply
- `crates/ui-cli/src/command_init/config.rs` — `UiConfig` may need `radius` + `font` fields to match

---

## 2. `ui apply <preset-code>` command

**Shadcn source**: `packages/shadcn/src/commands/apply.ts`

Apply a preset to an **existing** project without starting over:
- Decode the preset code
- Update `ui_config.toml` with new values
- Regenerate the CSS in `tailwind.css` (colors + radius)
- Re-install existing components (so RTL transforms etc. are re-applied)

Options:
```
ui apply <preset>           # apply everything
ui apply <preset> --only theme   # only update CSS vars/colors
ui apply <preset> --only font    # only update font
```

**Shadcn ref**: `APPLY_ONLY_VALUES = ["theme", "font"]`

**New file**: `crates/ui-cli/src/command_apply/`

---

## 3. `ui preset` subcommands

**Shadcn source**: `packages/shadcn/src/commands/preset.ts`

Three subcommands confirmed in source:

### 3a. `ui preset decode <code>`
Decode a preset code and print its human-readable values:
```
$ ui preset decode aG
theme:       neutral
radius:      0.5rem
color-theme: none
font:        inter
url:         https://rust-ui.com/create?preset=aG
```
With `--json` flag: output as JSON.

### 3b. `ui preset url <code>`
Print the `/create` URL for a preset code:
```
$ ui preset url aG
https://rust-ui.com/create?preset=aG
```

### 3c. `ui preset open <code>`
Open the preset in the browser (`https://rust-ui.com/create?preset=<code>`).

### 3d. `ui preset show`
**Shadcn source**: `preset.ts` — `show` subcommand reads project config and prints the current preset code + URL.
```
$ ui preset show
aG
https://rust-ui.com/create?preset=aG
```
Reads `ui_config.toml`, encodes current config into a preset code.

**New file**: `crates/ui-cli/src/command_preset/`

---

## 4. `/create` page — RTL toggle

**Shadcn source**: `init.ts` — `rtl: z.boolean().optional()` is part of the preset/init options.

The `/create` page should expose an **RTL toggle** so the generated preset URL can include `rtl=true` as a query param (or encoded in the preset bits). When `ui init --preset <code>` is run, RTL is applied automatically.

**Files**:
- `app/src/domain/create/preset.rs` — add `rtl: bool` bit to encoding (next available bit offset after font, currently bit 15)
- `app/src/domain/create/page_create.rs` — read `?preset=` RTL bit
- `app/src/domain/create/components/` — add an RTL toggle component (e.g. `rtl_toggle.rs`)

---

## 5. `ui add <local-path>` — Local File Support

**Shadcn source**: `packages/shadcn/src/commands/add.ts` / Jul 2025 changelog

The shadcn CLI accepts a local `.json` file path in addition to a component name or URL:
```
ui add ./my-component.json
ui add ./block.json
```

This enables:
- Testing registry items locally before publishing
- Private/proprietary components that never hit a remote registry
- Agent-generated components installed directly

**How it works**: detect if the argument is a path (starts with `./`, `/`, or `../`) and read the JSON file directly instead of fetching from `RustUIClient`.

**File**: `crates/ui-cli/src/command_add/_add.rs` + `registry.rs`
- In `_add.rs`: check if each component arg is a file path
- In `registry.rs`: add `RegistryComponent::from_local_file(path)` alongside `fetch_from_registry()`

---

## Implementation Order

- [ ] 1 — `ui init --preset <code>` flag
- [ ] 2 — `ui apply` command (`command_apply/`)
- [ ] 3a — `ui preset decode`
- [ ] 3b — `ui preset url`
- [ ] 3c — `ui preset open`
- [ ] 3d — `ui preset show`
- [ ] 4 — RTL bit in preset encoding + `/create` RTL toggle
- [ ] 5 — `ui add ./local-file.json` local path support

---

## Edge Cases & Tests

### 1 — `ui init --preset <code>`
- Valid preset → color/radius/font prompts skipped
- Invalid code (bad base62, wrong version, out-of-range index) → clear error
- Old 11-bit preset (no font bits) → font defaults to Inter, no crash
- `--preset` + `--yes` → zero prompts
- `--preset` + explicit `--rtl` flag → `--rtl` wins for RTL field
- Preset with RTL bit set → `rtl = true` written to config

```
command_init_preset_flag_is_registered
init_valid_preset_skips_color_prompt
init_invalid_preset_code_returns_error
init_old_preset_without_font_defaults_to_inter
init_preset_with_rtl_bit_sets_rtl_true
init_explicit_rtl_flag_overrides_preset
```

---

### 2 — `ui apply <preset>`
- No `ui_config.toml` → error "project not initialized"
- Invalid preset code → error before any files are touched
- `--only theme` → only CSS vars updated, font/reinstall skipped
- `--only font` → only font updated, no CSS regen or reinstall
- `--only` without a preset argument → error
- No installed components → skip reinstall, no error
- CSS input file not found → error with path shown
- Failure mid-way → backup restored (config not left in partial state)
- Running apply twice with same preset → idempotent

```
apply_requires_initialized_project
apply_invalid_preset_returns_error_before_writing
apply_only_theme_skips_font_and_reinstall
apply_only_font_skips_theme_and_reinstall
apply_only_without_preset_returns_error
apply_no_installed_components_does_not_error
apply_restores_backup_on_failure
apply_idempotent_when_run_twice
```

---

### 3 — `ui preset` subcommands

**decode**
- Valid code → all fields printed
- Invalid code → clear error
- Old 11-bit code → font defaults to Inter, no crash
- `--json` → valid JSON with all fields + url

```
preset_decode_valid_code_prints_fields
preset_decode_invalid_code_returns_error
preset_decode_old_11bit_code_defaults_font
preset_decode_json_outputs_valid_json
```

**url**
- Valid code → prints correct URL
- Invalid code → error (no URL printed)

```
preset_url_valid_code_prints_url
preset_url_invalid_code_returns_error
```

**open**
- Invalid code → error, browser not launched

```
preset_open_invalid_code_does_not_open_browser
```

**show**
- No `ui_config.toml` → error "project not initialized"
- Full config → correct code + URL printed
- Config with missing optional fields → encodes with defaults

```
preset_show_requires_initialized_project
preset_show_prints_code_and_url
preset_show_missing_optional_fields_uses_defaults
```

---

### 4 — RTL bit in `preset.rs`
- `encode(..., rtl: false)` → bit 15 is zero; old presets without RTL still decode correctly (backward compat)
- `encode(..., rtl: true)` → bit 15 set
- `decode_preset` on old code (no bit 15) → `rtl = false`, no crash
- Round-trip: encode → decode preserves all fields including RTL
- All combinations of fields don't overflow the bit layout

```
encode_rtl_true_sets_bit_15
encode_rtl_false_bit_15_is_zero
decode_old_preset_without_rtl_returns_false
rtl_round_trip_encode_decode
rtl_does_not_corrupt_other_fields
```

---

### 5 — `ui add ./local-file.json`
- `./button.json` → resolved relative to cwd
- `/absolute/path/button.json` → works
- File does not exist → clear error with path shown
- File exists but invalid JSON → clear error
- Valid JSON but missing required fields (`name`, `files`) → clear error
- Mixed args: `ui add button ./custom.json badge` → remote + local + remote all processed
- Local file + existing component + `force = false` → prompt shown
- Path starting with `./` → always treated as file, never as registry name

```
local_path_detected_for_dot_slash_prefix
local_path_detected_for_absolute_path
local_file_not_found_returns_error
local_file_invalid_json_returns_error
local_file_missing_required_fields_returns_error
local_file_installs_correctly
mixed_remote_and_local_args_both_processed
local_file_respects_force_flag
```
