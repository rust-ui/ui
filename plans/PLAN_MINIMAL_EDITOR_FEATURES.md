# PLAN: Minimal editor — feature matrix

Companion to `PLAN_MINIMAL_TIPTAP_EDITOR.md` (architecture/phasing/source-study
detail lives there). This file is just the feature inventory: every feature
either reference repo ships, plus its current status in `dioxus-ui`'s
`domain::test::editor` demo. ✅ = implemented and verified in `TestPage` demo.

| Feature | Status | Notes |
|---|---|---|
| Bold | ✅ | `FormatAction::Bold`, toolbar toggle + Cmd/Ctrl+B native |
| Italic | ✅ | `FormatAction::Italic` |
| Underline | ✅ | `FormatAction::Underline` |
| Strikethrough | ✅ | `FormatAction::Strikethrough` |
| Heading 1/2 | ✅ | `FormatAction::Heading(1\|2)` toolbar buttons |
| Heading 3-6 | ❌ | `FormatActionMeta` already covers levels 3-6, no toolbar button wired |
| Heading dropdown (single control, all levels) | ❌ | Aslam97 uses one dropdown, not flat H1/H2 buttons |
| Bullet list | ✅ | `FormatAction::BulletList` |
| Ordered list | ✅ | `FormatAction::OrderedList` |
| Task list (checkboxes) | ❌ | no `execCommand` equivalent, needs hand-built DOM |
| Code (inline) | ✅ | `FormatAction::Code` (`formatBlock` → `pre`, not true inline `<code>`) |
| Code block w/ syntax highlighting | ❌ | `syntect` already a dep, not wired to editor |
| Blockquote | ❌ | planned Phase 5 |
| Horizontal rule | ❌ | planned Phase 5 |
| Clear formatting | ✅ | `FormatAction::ClearFormatting` |
| Text color picker | ❌ | needs `foreColor`/CSS span wrap, no stored-marks quirk to port (Option B is immune) |
| Link insert/edit/remove | ❌ | needs `sanitize_url` port first (security-critical, see main plan §1.6) |
| Link bubble menu (hover-to-edit) | ❌ | no selection-anchored popover primitive exists yet |
| Image insert (URL) | ❌ | Phase 4 |
| Image upload (file picker) | ❌ | Phase 4/6, needs upload pipeline |
| Image drag/drop + paste | ❌ | Phase 6 |
| Image node view (resize handles, hover actions) | ❌ | deferred indefinitely, no `execCommand` equivalent |
| Align left/center/right/justify | ❌ | not in Aslam97's shipped toolbar either, optional |
| Subscript/superscript | ❌ | not in Aslam97's shipped toolbar either, optional |
| Undo/redo (native browser) | ✅ | Cmd/Ctrl+Z native `contenteditable` behavior, not yet Playwright-verified |
| Undo/redo (toolbar buttons) | ❌ | no dedicated buttons in Aslam97's shipped toolbar |
| Paste plain text | ✅ | native `contenteditable` behavior |
| Paste HTML (sanitized) | ⚠️ | `sanitize_element` exists and runs on every `input` event, but paste-specific flow not yet Playwright-verified |
| Paste Markdown → nodes | ❌ | `markdown-paste` extension, Aslam97 markdown-output mode only |
| HTML output (live) | ✅ | shown in demo's HTML output panel |
| Markdown output mode | ❌ | out of scope per main plan §3.4 (`pulldown-cmark` has no HTML→Markdown) |
| JSON output mode | ❌ | explicitly out of scope, no document model to serialize |
| Placeholder text | ✅ | `data-placeholder` + `empty:before:content-[...]` |
| Disabled state | ✅ | `contenteditable="false"` via `disabled` prop |
| Initial content (seeded HTML/Markdown) | ✅ | SSR'd via `dangerous_inner_html` |
| HTML sanitizer (allowlist tags/attrs) | ✅ | `sanitize_element`/`unwrap_element`/`sanitize_attrs` in `use_editor.rs` |
| URL sanitizer (`javascript:`/`data:` blocklist) | ❌ | not yet ported, needed before Link/Image ship |
| Active/pressed toolbar state sync | ✅ | `selectionchange` listener → `EditorState` signal |
| Platform-aware shortcut tooltips (⌘ vs Ctrl) | ❌ | cheap to port via `navigator().platform()`, not started |
| Throttled `on_change` | ❌ | fires on every `input` event today, no debounce |
| Toolbar overflow dropdown (main/overflow split) | ❌ | all actions currently flat, no `main_action_count` cutoff |
| Mobile-responsive toolbar (floating/bottom-sheet) | ❌ | Phase 6, secondary reference only |
| Slash command menu (`/` block picker) | ❌ | Phase 6, secondary reference only, cheaper than bubble menu |
| Resizable measured container (`--editor-width` CSS var) | ❌ | Phase 5/6 nice-to-have, low priority |
| Table (GFM) | ❌ | explicit v2/Phase-5 stretch, Aslam97 gates it behind markdown-output mode too |
| WASM release + SSR/native (iOS) safety | ✅ | all browser calls gated `#[cfg(target_arch = "wasm32")]` |

## Legend

- ✅ implemented and works in the `/test-page` demo
- ⚠️ partially there (code exists, not fully verified or not feature-complete)
- ❌ not started

See `PLAN_MINIMAL_TIPTAP_EDITOR.md` for phasing (§4), source study (§1), and
what's reusable from the existing `dioxus-ui`/`registry` components (§2).
