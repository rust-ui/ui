# Minimal editor — feature matrix

## P0 — Core, security, correctness

| Feature | Status | Reference / implementation |
|---|---:|---|
| Bold, italic, underline, strikethrough | ✅ | `FormatAction`, `use_editor.rs` |
| Bullet and ordered lists | ✅ | Native `execCommand` |
| H1/H2 | ✅ | `FormatAction::Heading` |
| Clear formatting | ✅ | Native `removeFormat` |
| Initial Markdown → HTML content | ✅ | `markdown_to_html`, SSR HTML |
| Live HTML callback | ✅ | `Editor::on_change`; no demo output panel |
| Placeholder | ✅ | `data-placeholder` |
| Disabled state | ✅ | `contenteditable=false` |
| Active toolbar state | ✅ | `query_command_state`, `selectionchange` |
| Typing/caret/selection verification | ⚠️ | Playwright MCP |
| Undo/redo verification | ⚠️ | Native browser behavior |
| IME verification | ⚠️ | Browser composition events |
| Paste HTML sanitization | ⚠️ | Intercept paste before insertion |
| HTML sanitizer tests | ⚠️ | `use_editor.rs` sanitizer |
| URL sanitizer tests | ⚠️ | Strict `javascript:`, `vbscript:`, `file:`, `data:` rules |
| SSR/WASM/native verification | ⚠️ | `#[cfg(target_arch = "wasm32")]` |

## P1 — Essential rich text

| Feature | Status | Reference / implementation |
|---|---:|---|
| Inline code `<code>` | ⚠️ | Current Code incorrectly creates `<pre>`; use Range/DOM operation |
| Code block `<pre><code>` | ❌ | Separate `CodeBlock` action |
| Normal paragraph action | ❌ | Primary heading selector pattern |
| H3–H6 | ❌ | `FormatAction::Heading(3..=6)` |
| Heading dropdown | ❌ | `__TMP/shadcn-minimal-tiptap/.../components/section/one.tsx` |
| Link insert/edit/remove | ❌ | `.../components/link/` + sanitized URL |
| Blockquote | ❌ | `section/five.tsx` |
| Horizontal rule | ❌ | `section/five.tsx` |
| Hard line break | ❌ | `<br>` insertion |
| List nesting/exit behavior | ⚠️ | Enter, double-Enter, backspace verification |
| Keyboard shortcut labels | ❌ | `utils.ts` + `shortcut-key.tsx` |
| Accessibility labels/focus | ⚠️ | `aria-pressed`, keyboard navigation, tooltips |

## P2 — Common productivity features

| Feature | Status | Reference / implementation |
|---|---:|---|
| Image insertion by URL | ❌ | `components/image/`; sanitize URL |
| Image alt/title editing | ❌ | Image dialog attributes |
| Image removal | ❌ | DOM selection operation |
| Text color | ❌ | `section/three.tsx`; `foreColor` |
| Text highlight | ❌ | `tiptap-shadcn/.../color-and-highlight.tsx` |
| Text alignment | ❌ | `tiptap-shadcn/.../alignment.tsx` |
| Toolbar undo/redo | ❌ | `tiptap-shadcn/.../undo.tsx`, `redo.tsx` |
| Search and replace | ❌ | `tiptap-shadcn/.../search-and-replace-toolbar.tsx` |
| Throttled `on_change` | ❌ | Primary `hooks/use-throttle.ts` |
| Word/character count | ❌ | Derived `textContent` |
| Max length/validation callback | ❌ | Consumer API |
| Read-only mode | ❌ | Separate from disabled |
| Markdown paste | ❌ | Primary `extensions/markdown-paste/` |
| Mobile toolbar overflow | ❌ | Drawer/overflow pattern |

## P3 — Advanced media and interaction

| Feature | Status | Reference / implementation |
|---|---:|---|
| Image file picker | ❌ | Primary `extensions/file-handler/` |
| Image drag/drop | ❌ | File handler + upload callback |
| Image paste | ❌ | Clipboard file handling |
| Upload progress/placeholder | ❌ | `tiptap-shadcn/.../image-placeholder.tsx` |
| Image resize handles | 🚫 | Primary `extensions/image/`; needs node view |
| Image captions | ❌ | Stable image wrapper required |
| Link bubble menu | 🚫 | Primary `components/bubble-menu/`; selection positioning gap |
| Floating toolbar | ❌ | `tiptap-shadcn/.../floating-toolbar.tsx` |
| Slash commands | ❌ | `tiptap-shadcn/.../floating-menu.tsx` |
| Responsive bottom-sheet toolbar | ❌ | Existing Drawer/media-query primitives |

## P4 — Document-model / v2

| Feature | Status | Reason |
|---|---:|---|
| Task lists | 🚫 | No native `execCommand` equivalent |
| GFM tables | 🚫 | Needs schema, navigation, paste rules |
| Markdown output mode | 🚫 | No general HTML → Markdown converter |
| JSON output | 🚫 | No document schema/model |
| Collaboration | 🚫 | Needs transactions/conflict resolution |
| Live syntax highlighting in code | 🚫 | DOM rerender breaks caret/undo |
| Full image node view | 🚫 | Needs persistent nodes/overlays |

## Implementation order

| Order | Scope |
|---:|---|
| 1 | Playwright validation: typing, selection, caret, lists, undo/redo, reload |
| 2 | Harden HTML/URL sanitizer + intercept paste |
| 3 | Split inline Code and CodeBlock |
| 4 | Paragraph + H1–H6 dropdown |
| 5 | Links |
| 6 | Blockquote + HR + hard break |
| 7 | Image URL + alt text |
| 8 | Color/highlight + alignment |
| 9 | Mobile overflow + search/replace |
| 10 | Re-evaluate architecture before P3/P4 |
