# PLAN: Rich text editor component (`Editor`), ported from shadcn-minimal-tiptap

Status: DRAFT v3, not started. Written for [issue #43](https://github.com/rust-ui/ui/issues/43)
("Does it support rich text editing?"), which `@r2hu1` volunteered to pick up
(comment thread already answered: pointed them at the primary reference below
and gave the go-ahead).

> **Current scope — demo only:** make working editor demos on `TestPage`.
> Ignore `__registry__` integration, public registry entries, docs Markdown,
> generated files, install manifests, and changelog for now.

Goal: ship a shadcn-style rich text editor primitive in `dioxus-ui`, matching
the toolbar/feature surface of the reference repos below as closely as
Dioxus/browser architecture allows. Where the source relies on Tiptap/
ProseMirror internals with no Rust equivalent, document the gap here instead
of silently inventing something different (same rule as
[[PLAN_SIDENAV_BLOCKS_VERBATIM_PORT]]).

**Scope lock**: demo only. Build against `src/domain/test/routing/test_pages.rs`
(`TestPage`, route `/test-page`) — add `DemoEditor` under
`src/domain/test/demos/` and mount it on `TestPage`. Do not add component to
public registry, docs pages, generated registry files, manifests, or changelog
yet. First prove editor works end to end in demo.

## References

- **Primary**: [Aslam97/shadcn-minimal-tiptap](https://github.com/Aslam97/shadcn-minimal-tiptap)
  (1799 stars, 111 forks, active). Cloned locally at
  `__TMP/shadcn-minimal-tiptap/` (gitignored, not committed — see `.gitignore:101`).
  All file paths below are relative to
  `__TMP/shadcn-minimal-tiptap/src/components/minimal-tiptap/` unless noted.
- **Secondary**: [ehtisham-afzal/tiptap-shadcn](https://github.com/ehtisham-afzal/tiptap-shadcn)
  (150 stars, 36 forks, active, default branch `master`). Cloned locally at
  `__TMP/tiptap-shadcn/`. Used **only** to fill three specific gaps the
  primary reference doesn't address well: the floating/bubble toolbar,
  mobile-responsive toolbar collapse, and one-file-per-button granularity as
  an alternative to Aslam97's one-file-per-section granularity. File paths
  below are relative to `__TMP/tiptap-shadcn/components/tiptap/` unless noted.
  This repo is *not* the structural basis for the port — Aslam97 is — it's
  cited only where explicitly called out.

---

## 0. The one decision that gates everything else

`shadcn-minimal-tiptap` is a thin styling/toolbar layer over **Tiptap**,
itself a thin layer over **ProseMirror** — a full JS document-model engine
(schema, transactions, plugins, collab, node views). There is no Rust/WASM
port of ProseMirror. Three ways forward:

**A. JS interop — bundle real Tiptap/ProseMirror, bridge with wasm-bindgen.**
Mount a `<div>`, call into the actual npm package via `js-sys`/`wasm-bindgen`
externs (like a CodeMirror or Monaco wrapper). Full fidelity, full extension
ecosystem (including the exact StarterKit/Image/CodeBlockLowlight/Markdown
extensions Aslam97 configures — see §1.3), collab-ready. Cost: introduces an
npm/JS build dependency into a crate whose entire pitch is "copy-paste Rust
source, no JS toolchain." Every other component in
`app_crates/registry/src/ui/*.rs` is JS-free except small inline `<script>`
snippets (`popover.rs`, `dropdown_menu.rs`) — this would be the first
component requiring a bundler step for consumers.

**B. Native contenteditable + `document.execCommand`.** Cheapest, matches the
existing inline-`<script>` pattern (`popover.rs` already injects vanilla JS
via `dangerous_inner_html` for click-outside/positioning — same trick works
for exec-command dispatch). `execCommand` is deprecated but every evergreen
browser still implements the commands this editor needs (bold/italic/
underline/strikethrough/insertUnorderedList/insertOrderedList/undo/redo/
formatBlock/createLink/insertImage). No document-model, no schema validation,
no collab, no structured JSON output — output is raw HTML read from
`.innerHTML`.

**C. Native contenteditable + a small hand-rolled Rust document model.**
Middle ground: don't touch `execCommand`, manipulate the DOM directly via
`web-sys` `Selection`/`Range`, keep a `Signal<Vec<Block>>` as source of truth,
re-render to contenteditable HTML on change. Gives structured output (can
serialize to Markdown via `pulldown-cmark`, already a `dioxus-ui` dep — see
§2) without pulling in JS. Most work, but keeps the crate JS-toolchain-free
and gives a real "model" story a future collab feature could build on. Markdown
conversion remains future scope; `pulldown-cmark` does not provide general
HTML-to-Markdown conversion.

**Recommendation: B for a strictly scoped web/WASM v1 MVP, structured as
composable shadcn-style primitives.** Keep command dispatch behind an
`EditorHandle`, so toolbar primitives never know whether implementation uses
`execCommand` today or Selection/Range operations later. Do not promise native
iOS parity until Phase 0 proves it; `execCommand` and DOM APIs are browser
runtime dependencies. Option A remains only valid route for exact Tiptap
parity, but violates this repo's no-JS-toolchain/copy-paste promise.

**Shadcn-like contract:** primitives are independently importable and
composable; editor shell composes them but does not hide them. Consumers can
render `EditorContent`, `EditorToolbar`, `ToolbarSection`, and action buttons
separately, provide custom sections/actions, and replace image/link behavior.
Demo must compose primitives exactly like consumer app would.

This is a call for the user/maintainer to confirm before implementation
starts — flagging it here rather than picking silently (see §7, open
question 1). Phase 0 must pass before B is accepted.

---

## 1. Source study — `Aslam97/shadcn-minimal-tiptap` (full read, not just tree)

### 1.1 File tree (all files listed have been read in full)

```
minimal-tiptap.tsx                       <- top-level <MinimalTiptapEditor>
types.ts                                 <- FormatAction, LinkProps, ShouldShowProps
utils.ts                                 <- isUrl/sanitizeUrl, getOutput, file validation
hooks/
  use-minimal-tiptap.ts                  <- useEditor() config, extension list, output modes
  use-tiptap-editor.ts                   <- context accessor (useCurrentEditor + useEditorState)
  use-container-size.ts                  <- ResizeObserver for the measured wrapper
  use-throttle.ts                        <- throttles onUpdate callback
  use-theme.ts                           <- light/dark flag for color-picker dark labels
components/
  toolbar-section.tsx                    <- generic <ToolbarSection actions activeActions mainActionCount>
  toolbar-button.tsx                     <- Toggle + Tooltip wrapper, isActive -> bg-accent
  shortcut-key.tsx                       <- renders platform kbd symbols in tooltips
  measured-container.tsx                 <- exposes --{name}-width/height CSS vars
  spinner.tsx                            <- image-upload loading spinner (not yet read in full, low-risk)
  section/
    one.tsx                              <- Heading dropdown (levels 1-6 configurable)
    two.tsx                              <- Bold/Italic/Underline/Strike/Code/ClearFormatting via ToolbarSection
    three.tsx                            <- Text color popover (3 palettes x 7 swatches)
    four.tsx                             <- Ordered/Bullet list via ToolbarSection
    five.tsx                             <- Link + Image + CodeBlock/Blockquote/HR via ToolbarSection
  bubble-menu/link-bubble-menu.tsx       <- floating menu, shows when selection is inside a link
  link/
    link-edit-block.tsx                  <- form (url, text, open-in-new-tab)
    link-edit-popover.tsx                <- toggles edit-block vs "view link" state
    link-popover-block.tsx               <- read-only link preview (open/edit/remove buttons)
  image/
    image-edit-dialog.tsx                <- Dialog wrapping ImageEditBlock
    image-edit-block.tsx                 <- URL form + hidden file input, calls editor.commands.setImages
extensions/
  index.ts                               <- re-exports all extensions below
  code-block-lowlight/                   <- CodeBlockLowlight (syntax highlighting via lowlight)
  color/                                 <- Color mark (works with @tiptap/extension-text-style)
  file-handler/                          <- FileHandler: paste/drop -> validated image insert
  horizontal-rule/                       <- custom HorizontalRule node (replaces StarterKit's)
  image/                                 <- custom Image node: upload pipeline, node view, actions
    (hooks/use-drag-resize.ts, hooks/use-image-actions.ts — not yet read line-by-line;
     referenced structurally from use-minimal-tiptap.ts's Image.configure() call, §1.4)
  markdown-paste/                        <- MarkdownPaste: parses pasted markdown text into nodes
  reset-marks-on-enter/                  <- ResetMarksOnEnter: Enter key drops active marks
  unset-all-marks/                       <- UnsetAllMarks: "clear formatting" command
styles/index.css + styles/partials/*.css <- --mt-accent-* CSS custom properties, typography/list/code styles
```

### 1.2 The `FormatAction` pattern (`types.ts:20-28`) — the single most important type to port

```ts
export interface FormatAction {
  label: string
  icon?: React.ReactNode
  action: (editor: Editor) => void
  isActive: (editor: Editor) => boolean
  canExecute: (editor: Editor) => boolean
  shortcuts: string[]
  value: string
}
```

Every toolbar button (bold, italic, list, blockquote, ...) is *data*, not a
bespoke component. `components/toolbar-section.tsx` (`ToolbarSection`, full
source read) takes `actions: FormatAction[]`, `activeActions?: string[]`
(ordered allow-list — this both filters *and* orders, see
`toolbar-section.tsx:39-51`: `sortedActions` is filtered by
`activeActions.includes(...)` then sorted by `activeActions.indexOf(...)`),
and `mainActionCount?: number`. It splits the sorted+filtered list into
`mainActions` (rendered as bare `ToolbarButton`s) and `dropdownActions`
(rendered as `DropdownMenuItem`s behind a chevron trigger, whose own
`isActive` highlight is `dropdownActions.some(a => a.isActive(editor))`).

This is implementation-agnostic — it works identically whether `action`
dispatches `editor.chain().focus().toggleBold().run()` (Tiptap) or
`document.execCommand('bold')` (this port's Option B). Port behavior as
first-class Rust types before writing toolbar sections. Keep commands
declarative/testable; do not copy React closures literally:

```rust
// app_crates/registry/src/ui/editor/format_action.rs
pub enum FormatAction {
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Code,
    ClearFormatting,
    Heading(u8),
    OrderedList,
    BulletList,
    CodeBlock,
    Blockquote,
    HorizontalRule,
}

pub struct FormatActionMeta {
    pub label: &'static str,
    pub shortcuts: &'static [&'static str],
    pub value: &'static str,
}
```

Metadata stays pure. Dispatch receives `EditorHandle` separately. Active and
enabled state comes from `EditorState`, updated by DOM `selectionchange` and
after commands. This keeps toolbar primitives composable and unit-testable.

### 1.3 Output formats and the extension list actually shipped (`hooks/use-minimal-tiptap.ts`, full read)

Corrects the plan v1's assumption of "html only." `UseMinimalTiptapEditorProps.output`
is `"html" | "json" | "text" | "markdown"` (line 29). `getOutput()`
(`utils.ts:43-57`) switches on it: `json` → `editor.getJSON()`, `markdown` →
`editor.getMarkdown()` (guarded `editor.isEmpty ? "" : ...`), `html` → same
empty guard + `getHTML()`, default → `getText()`.

Extensions always loaded (`createExtensions`, lines 58-190):
`StarterKit` (configured per-node: blockquote/bulletList/orderedList/paragraph
get `class="..."` HTMLAttributes for styling hooks; `codeBlock: false` and
`horizontalRule: false` because custom versions replace them; `link` configured
with `enableClickSelection: true, openOnClick: false`; `code` gets
`spellcheck="false"`; `dropcursor` gets a custom width/class), custom `Image`
(upload pipeline, see §1.6), `FileHandler` (drag/drop + paste, see §1.6),
`Color` + `TextStyle` (text color mark pair), `Selection` (`@tiptap/extensions`
— keeps the selection visually highlighted even when the editor loses focus,
relevant because the Link/Image popovers steal focus), `Typography` (smart
quotes/dashes/fractions), `UnsetAllMarks`, `HorizontalRule` (custom),
`ResetMarksOnEnter`, `CodeBlockLowlight`, `Placeholder`.

Extensions loaded **only when `output === "markdown"`** (lines 192-214):
`Markdown` (`@tiptap/markdown`, GFM enabled), `TaskList`/`TaskItem` (nested
checkboxes), `TableKit` (resizable GFM tables), `MarkdownPaste`. This is a
real behavioral fork, not a cosmetic one: tables and task lists literally do
not exist in the editor unless you opted into markdown output. **Decision
needed for the Rust port** (§7, open question 4): do we gate table/task-list
support behind an `output` prop the same way, or always include them? Given
Option B has no Tiptap schema at all, "extensions" in the Rust port aren't
pluggable node types — they're toolbar buttons + `execCommand`/manual DOM
manipulation, so the honest port of this fork is "only show the Table/
TaskList toolbar buttons when a `markdown`-flavoured value is requested,"
which is cheap to do since Option B has no cost to always parsing/serializing
  via a future Markdown adapter (§2) — recommend simplifying by always exposing task
lists (trivial `<ul data-type="taskList">` + checkbox `<input>` markup,
`execCommand` has no native command for these so they'd be hand-built DOM
manipulation regardless of the output-format gate) and treating full GFM
tables as an explicit v2/Phase-5 stretch goal, not v1.

### 1.4 Image extension: upload pipeline, validation, and UX (`use-minimal-tiptap.ts:90-181`, `utils.ts` file-validation helpers)

`Image.configure()` takes: `allowedMimeTypes: string[]` (supports wildcards
like `"image/*"`, checked in `utils.ts:171-174`
`checkTypeAndSize`: `allowedMimeTypes.includes(mimeType) ||
allowedMimeTypes.includes(`${mimeType.split("/")[0]}/*`)`), `maxFileSize`
(bytes), `allowBase64: boolean`, `uploadFn: (file) => Promise<string>`
(pluggable — falls back to `fakeuploader` which base64-encodes after a fake
3s delay if the consumer doesn't provide one), and four callback hooks:
`onToggle(editor, files, pos)` (creates a `blob:` object URL + random id per
file and inserts image nodes immediately, i.e. **optimistic insert before
upload completes** — the real uploaded URL presumably swaps in later via the
node's own internal state, not shown in this file), `onImageRemoved({id, src})`,
`onValidationError(errors)` (fires a `sonner` toast per error), and
`onActionSuccess`/`onActionError` for three built-in per-image actions
(`copyImage`, `copyLink`, `download` — mapped to human labels for toast
copy). `FileHandler.configure()` mirrors the same validation options for
drag-drop (`onDrop`) and paste (`onPaste`), converting to base64 via
`fileToBase64` and inserting at the drop position.

`image/image-edit-dialog.tsx` + `image/image-edit-block.tsx` (both read in
full) are the toolbar-triggered manual insert path (separate from
drag/drop/paste): a `Dialog` wrapping a form with a URL `<Input type="url"
required>` + Submit button (`editor.commands.setImages([{ src: link }])`)
and a full-width "Upload from your computer" button that clicks a hidden
`<input type="file" accept="image/*" multiple>` (`onChange` → 
`editor.commands.setImages(filesArray.map(file => ({ src: file })))`, then
closes the dialog). No progress UI in this component — upload progress, if
any, lives inside the node view (unread file, `extensions/image/`).

**Toast dependency**: Aslam97 uses `sonner` for all validation/action
feedback. `dioxus-ui` needs its own toast primitive for parity — check
whether one already exists in `app_crates/registry/src/ui/` before adding a
new dependency; reuse existing `toast_custom` primitive, no new dependency.

**Rust port scope decision**: full node-view actions (copy image/copy
link/download buttons overlaid on a hovered image, resize drag handles) are
the single most complex piece of the entire surface and have zero
`execCommand` equivalent — they're bespoke Tiptap NodeView/React portal
machinery. Recommend deferring the full node view to Phase 6 (§4) and
shipping Phase 4's image support as: insert `<img>` via `execCommand
('insertImage', url)` or manual DOM insert at the caret, no in-place resize
handles, no per-image action toolbar. Document this explicitly as a known gap
(§6).

### 1.5 Text color picker (`components/section/three.tsx`, full read)

Three curated palettes (`COLORS` array, lines 32-72), 7 swatches each,
referencing CSS custom properties (`var(--mt-accent-bold-blue)` etc. — these
live in `styles/index.css`/`styles/partials/*.css`, not yet read line-by-line
but the naming convention is confirmed via usage here: `--mt-accent-{name}`,
`--mt-accent-{name}-subtler}`). Selection UI is `ToggleGroup type="single"`
inside a `Popover`, with a `CheckIcon` overlay on the selected swatch (colour
computed via `palette.inverse`, `hsl(var(--background))` or
`hsl(var(--foreground))` depending on palette, so the checkmark stays
legible against both light and dark swatches).

The interesting non-obvious behavior (lines 155-172, `handleColorChange`):
before applying the new color, it explicitly clears `editor.state.storedMarks`
for the `textStyle` mark type via a direct `editor.view.dispatch(...
removeStoredMark(...))`, **then** applies the new color inside a
`setTimeout(fn, 0)`. This works around a Tiptap/ProseMirror quirk:
`storedMarks` are marks that apply to the *next typed character* even with no
active selection, and stacking a stored-mark removal + a chained `setColor`
in the same synchronous tick can race with ProseMirror's own transaction
scheduling. **This entire quirk is Tiptap/ProseMirror-internal and does not
exist in Option B** — `execCommand('foreColor', false, value)` (or CSS-based
manual span wrapping) has no stored-marks concept at all. Document as a
"gap that isn't actually a gap" — Option B is architecturally immune to this
specific bug class, not missing a workaround for it.

Dark-mode swatch labels: `useTheme()` (unread hook, but usage here is
self-explanatory — a boolean flag) swaps `color.label` for `color.darkLabel`
when set (only the "White"/"Black" swatch in Palette 3 has one, line 63).

### 1.6 Link popover + bubble menu (`components/link/*.tsx`, `components/bubble-menu/link-bubble-menu.tsx`, all read in full)

`link-edit-popover.tsx` toggles between two child views based on whether the
current selection/cursor is already inside a link: `link-popover-block.tsx`
(read-only preview: shows the URL, an "open" external-link button, an "edit"
button that flips to the edit form, a "remove" button calling
`editor.chain().extendMarkRange('link').unsetLink().run()` —
`extendMarkRange` expands the selection to cover the whole existing link mark
before removing it, so removing while the cursor is merely *inside* the link
text, not selecting all of it, still works) and `link-edit-block.tsx` (form:
URL input, optional display-text input, "open in new tab" switch, on submit
calls `editor.chain().focus().extendMarkRange('link').setLink({ href:
sanitizeUrl(url), target })...run()`).

**Security-critical**: `utils.ts:59-109`, `isUrl()`/`sanitizeUrl()`. Must be
ported byte-faithful into the Rust component, not simplified:

- `isUrl(text, { requireHostname, allowBase64 })`: rejects any text
  containing a newline outright. Parses via the JS `URL` constructor (Rust
  equivalent: the `url` crate, already likely available transitively —
  verify, or hand-roll the equivalent protocol check without pulling a new
  dep). Explicitly blocklists `javascript:`, `file:`, `vbscript:` protocols
  always, plus `data:` **unless** `allowBase64` is set, in which case `data:`
  is allowed only if it matches `/^data:image\/[a-z]+;base64,/` (image MIME
  types only, still base64-encoded, not raw markup).
- `sanitizeUrl(url, { allowBase64 })`: returns `undefined` for empty input;
  for `data:image` inputs, only passes through if `isUrl` accepts it under
  the base64 rules above; otherwise returns the url unchanged if `isUrl`
  accepts it, **or** if it matches `/^(\/|#|mailto:|sms:|fax:|tel:)/`
  (relative paths, fragments, and these specific safe schemes are allowed
  without going through full URL parsing), **or else prefixes `https://`**
  onto bare-looking input (e.g. typing `example.com` becomes
  `https://example.com`).

This is XSS/protocol-injection prevention — a link or image `src` accepted
without this filter is a stored-XSS vector (`javascript:` URLs execute on
click in some contexts; unsanitized `data:` URLs bypass CSP image-src
allowlisting). **Port as a standalone, unit-tested Rust function
(`sanitize_url(url: &str, allow_base64: bool) -> Option<String>`) before
wiring it into any popover/dialog submit handler.** This is the one piece of
"business logic" in the whole surface that must not be approximated.

`bubble-menu/link-bubble-menu.tsx`: uses Tiptap's `<BubbleMenu>` (from
`@tiptap/react`, itself built on `tippy.js` positioning) with a `shouldShow`
predicate checking `editor.isActive('link')` and viewport-position math to
render `link-popover-block.tsx`'s content floating next to the selected
link, without needing to click a toolbar button first. **No Dioxus/browser
primitive for this exists in the repo** (`popover.rs`'s CSS-anchor
positioning anchors to a *fixed trigger element*, not to a dynamic text
selection range) — see §1.7 for how the secondary reference approaches the
same problem, and §6 for the documented gap/fallback.

### 1.7 What the secondary reference (`ehtisham-afzal/tiptap-shadcn`) adds for the gaps above

Read in full: `components/tiptap/extensions/floating-menu.tsx`,
`components/tiptap/extensions/floating-toolbar.tsx`,
`components/tiptap/toolbars/mobile-toolbar-group.tsx`.

- **`extensions/floating-menu.tsx`** — a Notion-style **slash command** menu
  (`/` at start of an empty line opens a searchable `Command` palette listing
  block types: Text, Heading 1-3, Bullet/Numbered list, Code Block, Image,
  Horizontal Rule, Quote, inline Code, alignment commands), built on Tiptap's
  `<FloatingMenu>` (line-start-only positioning, much simpler than
  selection-anchored bubble menus — `shouldShow` just checks
  `$from.parent.textBetween(...)` for a leading `/`). This is a **different
  feature** from Aslam97's link-bubble-menu (slash-command insertion vs.
  hover-a-link editing), not a drop-in replacement, but it is a much more
  Dioxus-portable pattern: it positions relative to the *current line start*,
  not an arbitrary selection rectangle, so it could be approximated with a
  fixed-position element anchored to the contenteditable's caret via
  `document.getSelection().getRangeAt(0).getBoundingClientRect()` read once
  per keystroke rather than needing a live-tracking floating-UI library. Not
  in scope for v1 parity with Aslam97, but worth flagging as a *cheaper*
  floating-UI feature to attempt first if the maintainer wants any floating
  affordance in an early phase (see §6).
- **`extensions/floating-toolbar.tsx`** — resolves the **mobile-responsive
  toolbar** question Aslam97 doesn't address at all: on screens
  `<= 640px` (`useMediaQuery("(max-width: 640px)")`), the whole toolbar is
  replaced by a `<BubbleMenu>` that shows whenever `editor.isFocused` (not
  tied to a text selection at all — this is a trick, "bubble menu" is
  (ab)used here purely as "float a full toolbar above the on-screen keyboard
  while the editor has focus", not for selection-based editing), containing
  the *same* toolbar buttons but wrapped in a horizontally-scrolling
  `ScrollArea`. Above 640px it renders `null` and the normal fixed toolbar is
  used instead (this file doesn't show that side, it's presumably composed
  by a parent). **Portable pattern for Dioxus**: a `use_media_query`-style
  hook (check `app_crates/registry/src/hooks/` for an existing one — likely
  candidate given `use_random`/`use_container_size`-style hooks already
  exist per the sidenav plan precedent) driving a conditional render between
  the normal toolbar and a horizontally-scrollable one; no floating-UI
  library needed since it's anchored to viewport edge, not to selection.
- **`toolbars/mobile-toolbar-group.tsx`** — a `Drawer`-based (bottom sheet)
  overflow pattern: a labeled button opens a `Drawer` listing
  `MobileToolbarItem`s (full-width rows, `active` state highlights via
  `bg-accent`, `onClick` closes the drawer after a 100ms delay so the click
  visually registers first). This is the mobile equivalent of Aslam97's
  desktop dropdown-overflow (`mainActionCount` cutoff, §1.2) — same
  data-driven idea (`FormatAction[]` slice into "always visible" vs
  "overflow"), different overflow container (`Drawer` instead of
  `DropdownMenu`) for touch ergonomics. `dioxus-ui` needs to check whether it
  has a `Drawer`/bottom-sheet primitive already (likely, given the breadth of
  `app_crates/registry/src/ui/*.rs` — reuse existing `Drawer` primitive.
- **File granularity comparison**: this repo puts one component per toolbar
  *button* (`toolbars/bold.tsx`, `toolbars/headings.tsx`, etc., ~15 files)
  versus Aslam97's one file per toolbar *section* (5 files). Not read in
  detail (out of scope — the structural decision below in §3 follows
  Aslam97), but noted as a legitimate alternative if the Rust port ends up
  wanting per-button testability; recommendation is still to mirror Aslam97's
  section-level grouping since it maps directly to the `FormatAction[]` +
  `ToolbarSection` pattern already documented in §1.2, and fewer files
  matches this codebase's existing per-component (not per-button) file
  convention (see `toolbar.rs`, `toggle_group.rs`: one file, several related
  components).

### 1.8 Misc details worth preserving

- **Platform-aware shortcuts** (`utils.ts:24-38`): `isMacOS()` checks
  `navigator.platform === "MacIntel"`; `shortcutKeyMap` translates `mod` →
  ⌘/Ctrl, `alt` → ⌥/Alt, `shift` → ⇧ (always ⇧, no platform branch) for
  tooltip display via `shortcut-key.tsx`. Cheap to port 1:1 via `web-sys`
  `window().navigator().platform()`.
- **Throttled `onUpdate`** (`hooks/use-throttle.ts`, referenced not yet read
  line-by-line, but usage in `use-minimal-tiptap.ts:228-236` is clear):
  wraps the `onChange` callback so rapid typing doesn't fire the consumer's
  callback on every keystroke. Port as a `use_throttle`-style hook or inline
  debounce in `EditorContent`'s input handler — check `app_crates/registry/
  src/hooks/` for an existing throttle/debounce hook before writing one.
- **`MeasuredContainer`** (`components/measured-container.tsx`, full read):
  wraps the whole editor in a `ResizeObserver`-backed element exposing
  `--{name}-width`/`--{name}-height` CSS custom properties (used for
  responsive internal layout via pure CSS, no JS media query needed for
  *internal* editor layout, distinct from the *page-level* mobile breakpoint
  §1.7 addresses). Minor, low-priority to port — flag as Phase 5/6 nice-to-have.
- **`useTiptapEditor`** (`hooks/use-tiptap-editor.ts`, full read): a context
  accessor letting nested toolbar components either receive an explicit
  editor prop or fall back to a React context value
  (`useCurrentEditor`/`useEditorState` from `@tiptap/react`). Rust/Dioxus
  equivalent: `provide_context`/`use_context` for an `EditorHandle` struct —
  already an established pattern in this codebase (compound components like
  `popover.rs`'s `PopoverContext`, `toggle_group.rs`'s group context).

---

## 2. What already exists in this repo (reuse, don't rebuild)

Checked `app_crates/registry/src/ui/`:

- **`toolbar.rs`** (read in full, 200 lines) — `Toolbar`, `ToolbarList`,
  `ToolbarItem`, `ToolbarButton` (`ToolbarButtonVariant::{Default,Ghost,
  Outline}`), `ToolbarLink`, `ToolbarToggleGroup` (context-based; `multiple:
  bool` prop currently `#[allow(dead_code)]`, unused — worth wiring up if
  this editor needs it, e.g. for a hypothetical "bold+italic simultaneously"
  toggle group, though Aslam97's own sections are all independent toggles,
  not grouped multi-select, so this may stay unused), `ToolbarToggleItem`
  (`pressed`/`disabled`/`onclick`, `data-state="on"/"off"`), `ToolbarSeparator`
  (`mcp__rust-ui__search_components` returns nothing for "toolbar") — reuse
  directly in test demo.
  Maps directly onto `ToolbarSection`'s `mainActions` output (§1.2).
- **`popover.rs`** (read in full, 159 lines) — CSS-anchor-positioned,
  atomic-counter unique IDs, vanilla-JS click-outside/escape via inline
  `<script>`. Reuse verbatim for the Link edit popover (§1.6) and the color
  picker popover (§1.5). **Cannot** be reused as-is for the link
  bubble-menu's selection-anchored positioning (§1.6/§6) — that needs a
  dynamic anchor rect, not a fixed trigger element's CSS anchor-name.
- **`dialog.rs`** — reuse for the Image edit dialog (§1.4).
- **`dropdown_menu.rs`** (partial read: `DropdownMenuAlign`,
  `DropdownMenuPosition`, `DropdownMenuContext`, uses `icons::{Check,
  ChevronRight}`) — reuse for the Heading dropdown (section one) and the
  `ToolbarSection` overflow dropdown (§1.2's `dropdownActions` rendering).
- **`toggle_group.rs`** (read in full, 153 lines) — `ToggleGroupVariant::
  {Default,Outline}`, `ToggleGroupOrientation`, context-shared spacing,
  `ToggleGroupItem` (`role="radio"`). Candidate for the color-picker's
  swatch grid (§1.5, matches Aslam97's own use of shadcn `ToggleGroup` there)
  — use this rather than `ToolbarToggleGroup` for that one spot since the
  color swatches aren't toolbar buttons, they're a `role="radio"` single-select
  grid inside a popover, matching Aslam97's exact choice of primitive there.
- **`textarea.rs`**, **`input_prompt.rs`** — not directly reusable (plain
  `<textarea>`, no contenteditable), but `input_prompt.rs`
  (`InputPromptTextarea`: `Signal<String>`-bound, `oninput: move |ev|
  value.set(ev.value())`, `onkeydown` intercepts Enter) is the closest
  existing precedent for "auto-growing text box driven by a `Signal<String>`
  with keydown interception" — same shape this editor's state binding needs,
  minus the contenteditable-specific plumbing (§3.3).
- **`icons` crate** (`crates/icons/src/common/icon_type.rs`, grepped) — full
  lucide set already available via `features = ["dioxus", "dioxus_animated"]`:
  `Bold`, `Italic`, `Underline`, `Strikethrough`, `Code`, `Heading1..6`,
  `Quote`, `List`, `ListOrdered`, `Link`, `Link2`, `Image`, `ImagePlus`,
  `Minus`, `SeparatorHorizontal`, `AlignLeft/Center/Right/Justify`,
  `Subscript`, `Superscript`, `Undo`/`Undo2`, `Redo`/`Redo2`, `Eraser`
  (clear formatting), `ChevronDown`, `Check`, `Table`, `Type`, `Upload`,
  `WrapText`, `X`, `Trash`, `Plus`, `Highlighter`, `IndentDecrease/Increase`.
  Every icon both reference repos use has a match here — no new icon
  additions needed, including for the deferred table/task-list features.
- **`pulldown-cmark`** is already a root `Cargo.toml` dependency, but Markdown
  conversion is not part of demo MVP. Revisit only with a real
  HTML↔Markdown adapter and explicit wire format (§3.4).
- **`syntect`** is already a root dep — reuse for the code-block extension
  instead of porting `lowlight` (with the live-highlighting caveat, §6).
- No `use_eval`/JS-interop pattern exists anywhere in the codebase today
  beyond the inline `<script>` tags in `popover.rs`/`dropdown_menu.rs`
  (grepped `app_crates`, `crates`, `leptos-ui`, zero hits for `use_eval`).
  Confirms **Option B** (§0) is the only approach with local precedent;
  Option A would be a first for this codebase.
- `leptos-ui` has no toolbar/editor equivalent — this is not a leptos→dioxus
  port like the sidenav work, it's new on both sides. No cross-parity
  constraint from that direction.
- Existing `toast_custom`, `use_media_query`, `Drawer`, and throttle-related
  patterns can be reused if stretch features are later requested. No new
  dependency needed for demo MVP.

---

## 3. Proposed component: `ui/editor.rs` (or `ui/editor/` module — see below)

**Naming confirmed: `ui/editor`, component `Editor`.** Not `Tiptap`/
`MinimalTiptap` — there's no Tiptap dependency under Option B, so that name
would misrepresent what's shipped.

Given the file count implied by §1.2's `FormatAction` type,
`ToolbarSection`-equivalent, and 5 section compositions, this likely wants a
directory module (`ui/editor/mod.rs` + `format_action.rs` + `content.rs` +
`toolbar/{mod,heading,formatting,color,lists,insert}.rs` +
`link_popover.rs` + `image_dialog.rs`) rather than one flat `editor.rs`, to
stay consistent with this registry's file-per-concern convention seen in
multi-part components — confirm against how the largest existing
`ui/*.rs` file is organized before committing to a directory split (not yet
checked; if every other 150+-line component in this registry is still a
single flat file, follow that instead — don't introduce the first directory
module without precedent).

### 3.1 Composable hooks

Hooks extract browser/editor complexity from visual components. Public hooks
must stay small and independently usable; `Editor` only composes them.

```text
use_editor()              -> EditorHandle + context provider
use_editor_state()        -> reactive EditorState snapshot
use_editor_selection()    -> save/restore selection + selectionchange sync
use_editor_command()      -> focus, execute command, refresh state
use_editor_content()      -> mount contenteditable, input/paste bridge,
                             HTML sanitization, external initial content
use_editor_history()      -> undo/redo capability and command dispatch
use_editor_throttle()     -> optional on_change throttling
```

Suggested internal split:

```text
ui/editor/
  mod.rs                  public exports + Editor composition
  context.rs              EditorHandle, context provider/accessor
  state.rs                EditorState, active marks/block, disabled/focus
  actions.rs              FormatAction, metadata, command mapping
  selection.rs            Range snapshot/restore, selectionchange listener
  content.rs              contenteditable lifecycle and DOM bridge
  sanitizer.rs            sanitize_url, sanitize_html, file policy
  hooks.rs                public hook facades
  toolbar.rs              composable toolbar primitives
  sections.rs             default shadcn-like sections
  link.rs                 link popover primitive
  image.rs                image URL primitive
```

Hooks own setup and cleanup. Components render state and call hook commands.
No `web_sys::window()`/DOM FFI on SSR or native paths; browser calls gated by
target. Avoid hook explosion: add hook only when it owns lifecycle, state, or
reusable behavior; pure label/class helpers stay plain functions.

### 3.2 Public API sketch

```rust
#[component]
pub fn Editor(
    #[props(default)] initial_html: String,
    #[props(into, optional)] placeholder: Option<String>,
    #[props(into, optional)] class: Option<String>,
    #[props(default = false)] disabled: bool,
    #[props(optional)] on_change: Option<EventHandler<String>>,
) -> Element
```

`Editor` convenience composition. Expose primitives separately:

```rust
EditorContent
EditorToolbar
ToolbarSection
EditorToolbarButton
```

Consumers can compose custom toolbars, sections, and actions. `Editor` owns editing
state; `on_change` emits sanitized HTML. Defer controlled `value` API until
synchronization semantics are tested. Never rewrite `.innerHTML` every render:
it loses caret, selection, IME composition, and native undo history.

Composed internally, mirroring `minimal-tiptap.tsx:28-69`'s `Toolbar`
component structure **exactly** (section order, separators, and
`main_action_count` values below are load-bearing — this is the toolbar's
actual shipped composition, not a paraphrase):

```
Editor
├─ EditorToolbar
│   ├─ SectionOne     -- Heading dropdown, levels [1,2,3,4,5,6]
│   ├─ ToolbarSeparator
│   ├─ SectionTwo     -- actions: [bold, italic, underline, strikethrough,
│   │                    code, clear_formatting], main_action_count = 3
│   │                    (bold/italic/underline always visible as buttons;
│   │                    code + clear_formatting overflow into a
│   │                    "More formatting" dropdown)
│   ├─ ToolbarSeparator
│   ├─ SectionThree   -- text color popover, always fully visible, no overflow
│   ├─ ToolbarSeparator
│   ├─ SectionFour    -- actions: [ordered_list, bullet_list],
│   │                    main_action_count = 0 (BOTH inside a "Lists" dropdown,
│   │                    neither is ever a bare button)
│   ├─ ToolbarSeparator
│   └─ SectionFive    -- LinkEditPopover + ImageEditDialog rendered directly
│                        (not FormatActions, they're stateful popovers/dialogs),
│                        THEN actions: [code_block, blockquote, horizontal_rule],
│                        main_action_count = 0 (all three inside an
│                        "Insert elements" dropdown)
    └─ EditorContent       -- contenteditable div + JS bridge (§3.3)
    └─ (Phase 6, optional) LinkBubbleMenu — floats near link-text selection
```

Note Aslam97's actual `Toolbar` has **no** dedicated align/subscript/
superscript/undo/redo section in the shipped composition despite those
existing as importable extensions elsewhere in the package — plan v1
incorrectly assumed they were wired into the default toolbar. **Only include
them in the Rust port's default toolbar if the maintainer wants a superset
of Aslam97's actual shipped UI** — otherwise Phase 3 (§4) should treat
align/subscript/superscript/undo/redo as optional, not-yet-composed
`FormatAction`s a consumer could add via a `custom_sections` slot, closer to
parity with what's actually shipped than plan v1's guess.

### 3.3 `EditorContent` — the part with no existing precedent

A `div[contenteditable]` backed by internal state. On mount, use a WASM-only
DOM bridge (`document::eval` or `web-sys`; choose one in Phase 0) that:

- Sets `.innerHTML` from sanitized `initial_html`.
- On `input`, reads sanitized `.innerHTML` into internal state. Dioxus 0.7 has no
  native two-way contenteditable binding (unlike `<textarea>`'s `oninput`
  value payload) — needs either a `document::eval` round-trip channel or a
  `web-sys` listener registered from a `use_effect`, mirroring how
  `input_prompt.rs`'s `oninput` works for `<textarea>` but without a native
  `FormData`-style value on the event. **This is the first concrete unknown
  to spike before committing to the full component** (Phase 0, §4).
- Wires toolbar buttons to `document.execCommand('bold')` etc., dispatched
via the bridge, then re-syncs active
  states (bold/italic/heading-level) so toggle buttons show pressed/unpressed
  correctly. In Tiptap this active-state sync is free (`editor.isActive(...)`
  reads live ProseMirror state, wired to React re-renders via
  `useEditorState`, §1.8's `useTiptapEditor`) — this port has to reimplement
  it manually by re-querying `document.queryCommandState('bold')` (the
  `execCommand`-family's own state-query API, deprecated like `execCommand`
 itself but still universally implemented) after every selection change or
 toolbar action.

Before toolbar command: preserve/restore `Selection`/`Range`, prevent
`mousedown` from stealing focus, then execute command. Do not set
`.innerHTML` after every command. Sanitize initial HTML, pasted HTML, and
emitted HTML through one allowlist sanitizer. DOM bridge calls must be gated
for browser/WASM; SSR and native iOS paths must not call browser-only APIs.

### 3.4 Value format: HTML vs Markdown vs JSON

Aslam97 supports `html | json | text | markdown` (§1.3). Port only sanitized
HTML in v1 (matches `execCommand` output). Do not add Markdown conversion in
v1: `pulldown-cmark` parses/renders Markdown, but does not provide general
HTML-to-Markdown conversion. Markdown also cannot losslessly represent
features such as `<u>underline</u>`. A `json` output mode has no meaning under
Option B (there's no
ProseMirror document tree to serialize) — skip it entirely, don't fake a
JSON shape that implies structure this port doesn't have.

---

## 4. Phasing

**Phase 0 — spike (no component yet).** Mount bare contenteditable. Prove
input bridge, selection/range preservation, toolbar `mousedown`,
`execCommand`, undo/redo, IME typing, paste sanitization, and active-state
updates. Test dev/release WASM, SSR/hydration, and native iOS compile/runtime.
Gate all browser APIs behind the correct target cfg. Blocks everything below.

**Phase 1 — editor primitives + MVP toolbar.** Build `EditorHandle`,
`EditorState`, declarative `FormatAction` metadata, `EditorContent`,
`EditorToolbar`, `ToolbarSection`, and `EditorToolbarButton` as independently
composable pieces. Then SectionOne (heading) and SectionTwo
(bold/italic/underline/strike/code/clear-formatting, `main_action_count = 3`)
using them, all dispatched via native `execCommand`. Ships as `Editor` +
`EditorToolbar` with these two sections only.

**Phase 2 — Lists.** SectionFour (list dropdown, `main_action_count = 0`).

**Phase 3 — Link + sanitizer.** Port `sanitize_url`/`is_url` per §1.6, plus
`sanitize_html` allowlist. Unit-test blocked protocols and dangerous HTML
before wiring handlers. Then `LinkEditPopover` (reuse `popover.rs`).

**Phase 4 — Image URL.** `ImageEditDialog` (reuse `dialog.rs`) inserts only
sanitized remote URLs. File upload, `FileReader`, drag/drop, paste, backend
callback, resize, and per-image actions remain stretch scope. No fake upload
behavior in v1.

**Phase 5 — Color, Insert-elements.** SectionThree (text color popover,
3 palettes × 7 swatches, `toggle_group.rs`-based swatch grid per §2 — the
`storedMarks`/`setTimeout(0)` workaround from §1.5 is **not needed** under
Option B, document why in §6 rather than porting a no-op). SectionFive's
insert-elements dropdown (code block, blockquote, horizontal rule,
`main_action_count = 0`). Code block: reuse `syntect` for highlighting the
rendered `<pre><code>` (read-only/re-render-on-blur, not live-as-you-type —
document as a gap, §6).

**Phase 6 — optional editor features, not current scope.** Link bubble-menu on
selection (§1.6/§6). Full image node view (resize handles, hover action
buttons, drag/drop/paste-to-image per §1.4/§1.7). Mobile-responsive toolbar
collapse (§1.7's `floating-toolbar.tsx`/`mobile-toolbar-group.tsx` patterns —
needs a `use_media_query` hook and either a `Drawer` primitive or the
existing overflow-dropdown reused for touch too). Task lists / GFM tables
(§1.3). Slash-command floating menu (§1.7's `floating-menu.tsx` — noted as
the cheaper floating-UI feature to attempt if any floating affordance is
wanted before the link-bubble-menu is tackled).

No public registry/docs integration in this plan. Revisit only after demo
works and maintainer explicitly requests integration.

---

## 5. Security-critical logic that must be ported verbatim, not approximated

- `sanitize_url` / `is_url` (§1.6, source: `utils.ts:59-109`). Blocks
  `javascript:`, `file:`, `vbscript:` always; blocks `data:` unless an
  explicit base64-image allowlist mode is on, and even then only accepts
  `data:image/*;base64,...`. Falls back to prefixing `https://` on bare
  input rather than rejecting it outright (matches Aslam97's UX choice — a
  user typing `example.com` into the link field shouldn't get an error).
  Write this as a standalone, independently unit-tested function before any
  UI wires into it.
- Image MIME/size validation (§1.4, `checkTypeAndSize`/`filterFiles` in
  `utils.ts:163-233`): wildcard MIME matching (`image/*`), max-file-size
  enforcement, and separate handling for already-base64 vs URL vs `File`
  inputs. Even in the Phase 4 URL-only scope, the URL path still must run
  through `sanitize_url` before being accepted as an `<img src>` — an
  unsanitized image URL is as much an XSS vector as an unsanitized link
  href.

Add `sanitize_html` as separate security boundary. Sanitize initial content,
pasted content, externally supplied content, and emitted HTML. Allow only
document tags/attributes intentionally supported by v1; remove event
attributes, unsafe `style`, SVG/iframe/embed, and unsafe link/image protocols.
Unit-test sanitizer independently from UI.

---

## 6. Dioxus / browser limitations to document (fill in further during implementation, per the verbatim-port convention)

- No ProseMirror schema = no structural validation (e.g. nothing stops
  invalid nesting if `execCommand` allows it in a given browser). Tiptap's
  schema forbids this by construction; this port can't.
- No collaborative editing story (ProseMirror's transaction/plugin system
  has no equivalent here). Out of scope; don't advertise it as a future
  feature without committing to Option C (§0) first.
- `execCommand`/`queryCommandState` are deprecated (still shipped everywhere
  as of 2026, MDN marks them non-standard) — flag as a known future-removal
  risk, not an immediate blocker.
- Live syntax highlighting inside an editable code block (Tiptap/lowlight,
  via a ProseMirror node view) vs this port's read-only/re-render-on-blur
  highlighting via `syntect` — `execCommand` gives no hook for highlighting
  as you type inside a `contenteditable` region without fighting the
  browser's own caret/undo handling.
- Link bubble-menu (§1.6): Tiptap's `<BubbleMenu>` anchors to a live
  `Selection`/`getClientRects()` rectangle via `tippy.js`. No Dioxus/browser
  primitive in this repo does dynamic-selection-anchored positioning
  (`popover.rs`'s CSS anchor positioning is trigger-element-anchored, not
  selection-anchored). Phase 6 stretch goal only; may end up
  Dioxus-limited to "toolbar only, edit an existing link by placing the
  caret in it and clicking the Link toolbar button" instead of "a menu pops
  up automatically over the link." Document the final call here once
  attempted.
- The `storedMarks`/`setTimeout(0)` race workaround in Aslam97's color
  picker (§1.5) is Tiptap/ProseMirror-transaction-scheduling-specific and
  has no analog under Option B — not a gap, just doesn't apply; noted so a
  future contributor doesn't go looking for a bug that isn't there.
- Full image node view (resize handles, hover action buttons, optimistic
  blob-URL insert-before-upload-completes) is Tiptap NodeView/React-portal
  machinery with no `execCommand` equivalent. Phase 6 stretch; v1 image
  support is insert-only, no in-place manipulation.
- Table/task-list support is conditionally loaded in Aslam97 based on
  `output === "markdown"` (§1.3) — a real Tiptap schema distinction that
  doesn't map cleanly onto Option B, where there's no schema to conditionally
  extend, only toolbar buttons to conditionally show. Recommend simplifying
  per §1.3 rather than reproducing the exact conditional-extension-loading
  behavior.

### 6.1 Shadcn-style acceptance criteria for demo

- `Editor` convenience composition, not only public entry point.
- `EditorContent`, `EditorToolbar`, `ToolbarSection`, and toolbar button
  independently importable.
- Commands/state/metadata separated from rendering.
- Consumers can replace toolbar sections and provide custom actions.
- Shared `ui/*` primitives reused; no duplicate Button/Popover/Dialog code.
- No hidden backend, upload service, global singleton, or required app state.
- Public props small, explicit, stable; no `Signal<String>` requirement for
  basic uncontrolled usage.

### 6.2 Quality gate

Before each phase handoff:

```bash
cargo fmt --all -- --check
cargo clippy -p registry --no-deps -- -D warnings
cargo test -p registry --lib
```

Run Clippy in background, scoped to touched crate(s), per `AGENTS.md`; wait
for process exit and inspect output. Widen
validation only when public API requires it. Zero warnings required; no
`#[allow]` without documented reason. CSS/comment/format-only changes may
skip Clippy.

Behavior gate: typing, selection, undo/redo, IME, paste sanitization, URL
sanitization, disabled state, SSR, WASM release, and native iOS checks pass in
`TestPage` demo.

---

## 7. Open questions for the user before implementation starts

1. Confirm **Option B** (§0) as web/WASM MVP over A/C, or override.
2. ~~Component naming~~ — **resolved: `ui/editor`, component `Editor`** (§3).
3. File layout: use `ui/editor/` directory module. Keep public surface in
   `mod.rs`; split state, content, actions, toolbar, sanitizer, link, image.
4. Markdown is out of v1. Revisit only after choosing real HTML↔Markdown
   converter and wire format.
5. Toolbar composition scope (§3.1): ship only Aslam97's actual 5-section
   toolbar (no align/subscript/superscript/undo/redo by default), or
   deliberately ship a superset since those actions exist as portable
   `FormatAction`s regardless?
6. Should `r2hu1` be pointed at this doc directly (a comment on #43 linking
   the plan) before they start writing code, so the Option A/B/C call and
   phasing land before implementation does?
7. Existing toast, media-query, and drawer primitives exist; reuse them only
   when stretch features start. No new dependency for v1.
8. Contributor must run scoped Clippy with `-D warnings` before handoff.

---

## Appendix: file-by-file port mapping

| Reference file (Aslam97, unless noted) | Proposed Rust target | Phase |
|---|---|---|
| `types.ts` (`FormatAction`) | `ui/editor/format_action.rs` (or top of `ui/editor.rs`) | 1 |
| `utils.ts` (`isUrl`/`sanitizeUrl`) | `ui/editor/sanitize_url.rs`, unit-tested | 3 |
| `utils.ts` (`getOutput`) | `Editor`'s `on_change` value construction | 1 |
| `utils.ts` (`getShortcutKey(s)`, `isMacOS`) | `ui/editor/shortcut.rs` | 6 |
| `components/toolbar-section.tsx` | `ui/editor/toolbar_section.rs` (generic `ToolbarSection` over `FormatAction`) | 1 |
| `components/toolbar-button.tsx` | reuse `toolbar.rs`'s `ToolbarButton`/`ToolbarToggleItem` | 1 |
| `components/shortcut-key.tsx` | `ui/editor/shortcut.rs` (tooltip rendering) | 6 |
| `components/measured-container.tsx` | deferred, low priority | 5/6 |
| `components/section/one.tsx` | `ui/editor/section_heading.rs` | 1 |
| `components/section/two.tsx` | `ui/editor/section_formatting.rs` | 1 |
| `components/section/three.tsx` | `ui/editor/section_color.rs` (uses `toggle_group.rs` + `popover.rs`) | 5 |
| `components/section/four.tsx` | `ui/editor/section_lists.rs` | 2 |
| `components/section/five.tsx` | `ui/editor/section_insert.rs` | 5 |
| `components/link/link-edit-block.tsx` + `link-edit-popover.tsx` + `link-popover-block.tsx` | `ui/editor/link_popover.rs` (uses `popover.rs`) | 3 |
| `components/bubble-menu/link-bubble-menu.tsx` | `ui/editor/link_bubble_menu.rs` (stretch, §6) | 7 |
| `components/image/image-edit-dialog.tsx` + `image-edit-block.tsx` | `ui/editor/image_dialog.rs` (uses `dialog.rs`) | 4 |
| `extensions/image/` (node view, drag-resize, actions) | not ported in v1 (§1.4/§6) | 7 |
| `extensions/file-handler/` | not ported in v1 (drag/drop/paste-to-image) | 7 |
| `extensions/code-block-lowlight/` | `syntect`-based read-only highlighting in `EditorContent` | 5 |
| `extensions/horizontal-rule/`, `unset-all-marks/`, `reset-marks-on-enter/` | folded into `execCommand`/manual DOM ops, no dedicated files needed | 1/3 |
| `extensions/markdown-paste/` | deferred until real HTML↔Markdown adapter chosen (§3.4) | 6+ |
| `hooks/use-minimal-tiptap.ts` | `EditorContent`'s DOM bridge + `Editor` prop wiring | 0/1 |
| `hooks/use-tiptap-editor.ts` | `provide_context`/`use_context` `EditorHandle` | 1 |
| `hooks/use-throttle.ts` | reuse existing hook if present, else small inline debounce | 1 |
| `hooks/use-theme.ts` | reuse existing theme hook (likely already exists elsewhere in `dioxus-ui`) | 6 |
| `tiptap-shadcn/components/tiptap/extensions/floating-toolbar.tsx` | `ui/editor/mobile_toolbar.rs` (stretch, §1.7) | 7 |
| `tiptap-shadcn/components/tiptap/toolbars/mobile-toolbar-group.tsx` | overflow via `Drawer` for touch (stretch) | 7 |
| `tiptap-shadcn/components/tiptap/extensions/floating-menu.tsx` | slash-command menu (stretch, cheaper than link bubble-menu, §1.7) | 7 |

---

`__TMP/shadcn-minimal-tiptap/` and `__TMP/tiptap-shadcn/` are gitignored
local clones kept for reference during this research; safe to leave in place
or delete once implementation starts (they're not part of the repo history
either way).
