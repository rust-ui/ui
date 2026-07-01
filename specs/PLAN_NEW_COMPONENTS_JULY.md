# Plan — Chat Interface Components (shadcn → Leptos/Rust)

> Source: `rust_ui_internals/!!!_OTHERS/SHADCN-UI/apps/v4/`  
> Target: `app_crates/registry/src/`

**Do not modify `ui/chat.rs`** — separate older component, untouched.

## Status

| Component | UI file | Demos | Doc | Registry |
|-----------|---------|-------|-----|----------|
| Marker | ✅ Done | ✅ Done (5) | ✅ Done | ✅ Done |
| Message | ✅ Done | ✅ Done (6) | ✅ Done | ✅ Done |
| Bubble | ✅ Done | ✅ Done (9) | ✅ Done | ✅ Done |
| Attachment | ✅ Done | ✅ Done (6) | ✅ Done | ✅ Done |
| MessageScroller | ⬜ Todo | ⬜ Todo | ⬜ Todo | ⬜ Todo |

---

> ### 🚨 PORTING RULES — NON-NEGOTIABLE
>
> **Components:**
> - Port EXACTLY from shadcn source. Same CSS classes, same composition, same props (adapted for Leptos).
> - Convert `data-slot` → `data-name`. Convert enum string values to PascalCase (strum::Display).
> - If shadcn uses `asChild`/`render` prop (Radix Slot): split into `href: Option<String>` (→ `<a>`) and `on_click: Option<Callback>` (→ `<button>`), fallback to `<div>`.
> - If a CSS selector or behavior can't be ported 1:1, add a `// TODO PORT: <explanation>` comment in the component file.
>
> **Demos:**
> - Port EXACT shadcn demos only. Same content, same text, same icons, same wrapper (`flex w-full max-w-sm flex-col gap-8 py-12`).
> - Do NOT invent demos that don't exist in shadcn. Check MDX docs for which demos are shown.
> - If a demo uses `toast()`, `asChild`, Dialog, or any pattern that can't be ported 1:1, add a `// TODO PORT: <explanation>` comment in the demo file.

---

## Files to Create

### UI components
```
app_crates/registry/src/ui/marker.rs
app_crates/registry/src/ui/message.rs
app_crates/registry/src/ui/bubble.rs
app_crates/registry/src/ui/attachment.rs
app_crates/registry/src/ui/message_scroller.rs
```

### Register in `ui/mod.rs` (alphabetical order)
```rust
pub mod attachment;
pub mod bubble;
pub mod marker;
pub mod message;
pub mod message_scroller;
```

### Docs
```
public/docs/components/marker.md
public/docs/components/message.md
public/docs/components/bubble.md
public/docs/components/attachment.md
public/docs/components/message_scroller.md
```

---

## Step 0 — CSS Prerequisites (do this first)

These utilities are used by the new components but **don't exist in our project yet**.  
Find the Tailwind input CSS file (check `app/style/`, `public/`, `app/src/`) and add these blocks.

### `wrap-break-word`
Used in: `BubbleContent`, `Message`, `MessageScrollerContent`
```css
@utility wrap-break-word {
  overflow-wrap: break-word;
  word-break: break-word;
}
```

### `scroll-fade-b` / `scroll-fade-x`
Used in: `MessageScrollerViewport` (bottom fade), `AttachmentGroup` (right fade) — cosmetic only.
```css
@utility scroll-fade-b {
  mask-image: linear-gradient(to bottom, black calc(100% - 2rem), transparent 100%);
}
@utility scroll-fade-x {
  mask-image: linear-gradient(to right, black calc(100% - 2rem), transparent 100%);
}
```

### `scrollbar-thin` / `scrollbar-gutter-stable` / `scrollbar-none`
Used in: `MessageScrollerViewport`. shadcn uses `tailwind-scrollbar` plugin — we don't have it.
```css
@utility scrollbar-thin {
  scrollbar-width: thin;
}
@utility scrollbar-gutter-stable {
  scrollbar-gutter: stable;
}
@utility scrollbar-none {
  scrollbar-width: none;
  &::-webkit-scrollbar { display: none; }
}
```

### `shimmer` — CSS text animation (NOT our `<Shimmer>` skeleton component)
shadcn `shimmer` = pure CSS text sweep animation (`background-clip: text` + moving gradient).  
Used in: Marker separator demo text, `AttachmentTitle` during uploading/processing states.  
Usage: `class="shimmer text-muted-foreground"` on any text element.

Full definition from `packages/shadcn/src/tailwind.css`:
```css
@property --shimmer-angle {
  syntax: "<angle>";
  inherits: true;
  initial-value: 20deg;
}

@theme inline {
  @keyframes tw-shimmer {
    from { background-position: 100% 0; }
    to   { background-position: 0 0; }
  }
}

@utility shimmer {
  --_spread: var(--shimmer-spread, calc(3ch + 40px));
  --_base: currentColor;
  --_highlight: var(--shimmer-color, oklch(from currentColor l c h / calc(alpha * 0.2)));
  background-image: linear-gradient(
    calc(90deg + var(--shimmer-angle)),
    var(--_base) calc(50% - var(--_spread)),
    color-mix(in oklch, var(--_highlight), var(--_base) 50%) calc(50% - var(--_spread) * 0.5),
    var(--_highlight) 50%,
    color-mix(in oklch, var(--_highlight), var(--_base) 50%) calc(50% + var(--_spread) * 0.5),
    var(--_base) calc(50% + var(--_spread))
  );
  background-repeat: no-repeat;
  background-size: calc(200% + var(--_spread) * 2) 100%;
  background-position: 0 0;
  background-clip: text;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  animation: tw-shimmer var(--shimmer-duration, 2s) linear infinite;
  @variant dark {
    --_highlight: var(--shimmer-color, oklch(from currentColor max(0.8, calc(l + 0.4)) c h / calc(alpha + 0.4)));
  }
  &:where([dir="rtl"], [dir="rtl"] *) { animation-direction: reverse; }
}

@utility shimmer-once { animation-iteration-count: 1; }
@utility shimmer-reverse { animation-direction: reverse; }
@utility shimmer-none { -webkit-text-fill-color: currentColor; }
```

---

## Rust/Leptos Differences vs shadcn React

### 1. Variants = PascalCase enums with `strum::Display`

shadcn uses string literals (`"default"`, `"secondary"`). We use enums + `strum::Display`.  
`strum::Display` outputs PascalCase (e.g. `Separator.to_string()` → `"Separator"`).  
**CSS selectors must match PascalCase** (e.g. `data-[variant=Separator]`, `data-[state=Done]`).

```rust
#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleVariant {
    #[default] Default,
    Secondary, Muted, Tinted, Outline, Ghost, Destructive,
}

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleAlign { #[default] Start, End }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum BubbleReactionsSide { Top, #[default] Bottom }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum MarkerVariant { #[default] Default, Separator, Border }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum MessageAlign { #[default] Start, End }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentState { Idle, Uploading, Processing, Error, #[default] Done }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentSize { #[default] Default, Sm, Xs }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentOrientation { #[default] Horizontal, Vertical }

#[derive(Clone, Copy, PartialEq, Eq, Default, strum::Display)]
pub enum AttachmentMediaVariant { #[default] Icon, Image }
```

Use `variant.to_string()` in `data-variant=` attrs. No manual `as_str()` needed.

### 2. Macros

- **`clx!`** — layout-only sub-parts (no props other than class/children)
- **`variants!`** — components with multiple style variants (like `badge.rs`)
- **`#[component]`** — anything needing custom props or logic

### 3. No `asChild`

shadcn uses Radix `asChild` to polymorphically swap element type.  
Our adaptations:
- `AttachmentTrigger` → `href: Option<String>` prop → renders `<a>` if Some, `<button>` if None
- `BubbleContent asChild` → ignore, always renders `<div>`
- `Marker asChild` → ignore, children compose naturally inside

### 4. Props pattern

```rust
#[component]
pub fn Bubble(
    #[prop(optional)] variant: BubbleVariant,
    #[prop(optional)] align: BubbleAlign,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView
```

### 5. `data-*` attributes — `data-name`, NOT `data-slot`

Project convention: use `data-name` (matches `clx!` macro auto-output). **Never `data-slot`.**

- `clx!` macro → auto-sets `data-name=stringify!($name)` → PascalCase (e.g. `"BubbleContent"`)
- `#[component]` → set manually: `data-name="Bubble"`

CSS selectors must use `data-name` with PascalCase values:
```
shadcn:  *:data-[slot=bubble-content]:bg-primary
ours:    *:data-[name=BubbleContent]:bg-primary

shadcn:  has-data-[slot=attachment-content]:px-2.5
ours:    has-data-[name=AttachmentContent]:px-2.5

shadcn:  group-has-data-[slot=message-footer]/message:-translate-y-8
ours:    group-has-data-[name=MessageFooter]/message:-translate-y-8

shadcn:  *:data-slot:self-end        (presence selector)
ours:    *:data-name:self-end
```

```rust
<div
    data-name="Bubble"
    attr:data-variant=variant.as_str()
    attr:data-align=align.as_str()
    class=merged_class
>
```

### 6. MessageScroller — no `@shadcn/react` primitive

shadcn wraps `@shadcn/react/message-scroller` npm package. We implement from scratch with:
- `NodeRef<leptos::html::Div>` for viewport
- `RwSignal<bool>` for show_scroll_button
- `Effect` for auto-scroll
- `on:scroll` for position tracking
- Leptos `provide_context` / `use_context` for child access

### 7. Timer/interval — web_sys, NOT leptos_use

`leptos_use` is NOT in Cargo.toml. Use raw web_sys (pattern from `use_press_hold.rs`):
```rust
use wasm_bindgen::prelude::*;
use web_sys::window;

let closure = Closure::wrap(Box::new(move || { /* append token */ }) as Box<dyn FnMut()>);
let id = window().unwrap()
    .set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(),
        10  // ms
    ).unwrap();
closure.forget();

// Cancel: window().unwrap().clear_interval_with_handle(id);
```
Store `id` in `RwSignal<Option<i32>>` to cancel from stop button.

---

## Reference Patterns (confirmed from codebase)

### Multi-sub-part component (`avatar.rs` pattern)
```rust
// Enums at top
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize { Sm, #[default] Default, Lg }

// clx! sub-parts in mod block
mod components {
    use super::*;
    clx! { AvatarFallback, div, "absolute inset-0 flex size-full items-center ..." }
    clx! { AvatarGroup, div, "group/avatar-group flex -space-x-2 ..." }
}
pub use components::*;

// #[component] for parts with props
#[component]
pub fn Avatar(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: AvatarSize,
) -> impl IntoView {
    let size_str = match size {
        AvatarSize::Sm => "sm",
        AvatarSize::Default => "default",
        AvatarSize::Lg => "lg",
    };
    let merged_class = tw_merge!("group/avatar relative flex ...", class);
    view! {
        <div class=merged_class data-slot="avatar" data-size=size_str>
            {children()}
        </div>
    }
}
```

### Demo file structure
```rust
use leptos::prelude::*;
use crate::ui::bubble::{Bubble, BubbleContent, BubbleVariant};

#[component]
pub fn DemoBubbleVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4">
            <Bubble><BubbleContent>"Default"</BubbleContent></Bubble>
            <Bubble variant=BubbleVariant::Secondary>
                <BubbleContent>"Secondary"</BubbleContent>
            </Bubble>
        </div>
    }
}
```

### Available icons (confirmed in `icons` crate)
`GitBranch`, `FileSearch`, `FileText`, `Download`, `CircleUserRound`,
`Search`, `Check`, `X`, `ArrowDown`, `Paperclip`, `ArrowUp`,
`UserRound`, `UserRoundMinus`, `NotepadText`, `Link`, `Activity`,
`FolderOpen`, `GitBranchPlus`, `Pencil` — all present.

---

## Component 1 — Marker (`ui/marker.rs`) ✅ DONE

### Composition
```
Marker        — #[component], variant: MarkerVariant (strum::Display)
  MarkerIcon    — #[component], aria-hidden="true", children: Children
  MarkerContent — clx! span (data-name="MarkerContent" auto-set)
```

### CSS classes (PascalCase selectors — strum::Display)
```rust
// Marker base
"group/marker relative flex min-h-4 w-full items-center gap-2 text-left text-sm text-muted-foreground [&_svg:not([class*='size-'])]:size-4 [a]:underline [a]:underline-offset-3 [a]:hover:text-foreground"

// variant Default   → ""
// variant Separator → "before:mr-1 before:h-px before:min-w-0 before:flex-1 before:bg-border after:ml-1 after:h-px after:min-w-0 after:flex-1 after:bg-border"
// variant Border    → "border-b border-border pb-2"

// MarkerIcon (aria-hidden="true", #[component] not clx! — needs aria attr)
"size-4 shrink-0 [&_svg:not([class*='size-'])]:size-4"

// MarkerContent (group selectors use PascalCase to match strum output)
"min-w-0 wrap-break-word group-data-[variant=Separator]/marker:flex-none group-data-[variant=Separator]/marker:text-center *:[a]:underline *:[a]:underline-offset-3 *:[a]:hover:text-foreground"
```

### Demos ✅ DONE — exact shadcn demos
**Rule: always port the EXACT shadcn demos, same content, same wrapper (`max-w-sm py-12 gap-8`).**

| File | shadcn source | Shows |
|------|---------------|-------|
| `demo_marker.rs` | `marker-demo` | GitBranch, Spinner+shimmer, Separator, Search |
| `demo_marker_variants.rs` | `marker-variants` | Default / Separator / Border text-only |
| `demo_marker_status.rs` | `marker-status` | Spinner in default + separator with role="status" |
| `demo_marker_shimmer.rs` | `marker-shimmer` | shimmer text only, no icon |
| `demo_marker_separator.rs` | `marker-separator` | Today / Worked for 42s / Conversation compacted |
| `demo_marker_border.rs` | `marker-border` | GitBranch / Search / FileText border rows |
| `demo_marker_icon.rs` | `marker-icon` | Default, Separator with icon, flex-col stacked |
| `demo_marker_link_button.rs` | `marker-link-button` | href=<a>, on_click=<button> |
| `demo_shimmer_marker.rs` | `shimmer-marker` | Spinner+shimmer default + separator |

### Marker also supports `href` and `on_click`
```rust
// href → renders <a>
<Marker href="#pr-link">...</Marker>
// on_click → renders <button>
<Marker on_click=Callback::new(move |_| { ... })>...</Marker>
```

### TODO PORT notes (Marker)
- **`render` prop → `href`/`on_click`**: shadcn uses `render={<a href="..." />}` (asChild). We use two explicit props. Comment in `marker.rs` and `demo_marker_link_button.rs`.
- **`toast()` → `web_sys::window().alert_with_message()`**: in `demo_marker_link_button.rs`, shadcn calls `toast(...)`. We use a browser alert since wiring sonner in a standalone demo is disproportionate.

---

## Component 2 — Message (`ui/message.rs`)

Layout-only, no state. All alignment done via CSS group selectors from `data-align`.

### Composition
```
MessageGroup   — clx! div
Message        — #[component], align: MessageAlign
  MessageAvatar  — clx! div
  MessageContent — clx! div
  MessageHeader  — clx! div
  MessageFooter  — clx! div
```

### CSS classes
```rust
// MessageGroup
"flex min-w-0 flex-col gap-2"

// Message (align End → data-[align=End]:flex-row-reverse)
"group/message relative flex w-full min-w-0 gap-2 text-sm data-[align=End]:flex-row-reverse"

// MessageAvatar
"flex w-fit min-w-8 shrink-0 items-center justify-center self-end overflow-hidden rounded-full bg-muted group-has-data-[name=MessageFooter]/message:-translate-y-8"

// MessageContent
"flex w-full min-w-0 flex-col gap-2.5 wrap-break-word group-data-[align=End]/message:*:data-name:self-end"

// MessageHeader
"flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0"

// MessageFooter
"flex max-w-full min-w-0 items-center px-3 text-xs font-medium text-muted-foreground group-has-data-[variant=Ghost]/message:px-0 group-data-[align=End]/message:justify-end"
```

### TODO PORT notes (Message)
- **`*:data-name:self-end` presence selector**: shadcn uses `*:data-[slot]:self-end` (any child with data-slot attr gets `self-end`). Our equivalent is `*:data-name:self-end`. Verify Tailwind v4 supports this presence selector; if not, may need explicit selectors per child type.
- **`group-has-data-[name=MessageFooter]/message:-translate-y-8`**: complex nested group selector. If avatar sliding doesn't work, check Tailwind v4 support for `group-has-data-[name=X]/scope` syntax.

### Demos (6) — exact shadcn MDX demos
| File | shadcn source | Shows |
|------|---------------|-------|
| `demo_message.rs` | `message-demo` | Avatar + bubble, Marker below (assistant + user thread) |
| `demo_message_avatar.rs` | `message-avatar` | MessageAvatar with Avatar, start + end alignment |
| `demo_message_group.rs` | `message-group` | MessageGroup: sequential bubbles, empty avatar on follow-ups |
| `demo_message_header_footer.rs` | `message-header-footer` | Sender name in header, timestamp in footer |
| `demo_message_actions.rs` | `message-actions` | Footer with Copy/Retry/Thumbs Up icon buttons |
| `demo_message_attachment.rs` | `message-attachment` | Attachment inside MessageContent |

---

## Component 3 — Bubble (`ui/bubble.rs`)

### Composition
```
BubbleGroup     — clx! div
Bubble          — #[component], variant + align props
  BubbleContent   — #[component], href/on_click props (asChild replacement), renders <a>|<button>|<div>
  BubbleReactions — #[component], side + align props
```

### CSS classes
```rust
// BubbleGroup
"flex min-w-0 flex-col gap-2"

// Bubble base (variant classes applied via data-variant CSS selectors on BubbleContent)
"group/bubble relative flex w-fit max-w-[80%] min-w-0 flex-col gap-1 group-data-[align=End]/message:self-end data-[align=End]:self-end data-[variant=Ghost]:max-w-full"

// Per variant → applied as class on Bubble, targeting *:data-[name=BubbleContent]
// (data-name is project convention; shadcn uses data-slot — our clx! sets data-name="BubbleContent")
// Includes hover states for interactive bubble content (button/a children)
// Default:
//   "*:data-[name=BubbleContent]:bg-primary *:data-[name=BubbleContent]:text-primary-foreground
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-primary/80"
// Secondary:
//   "*:data-[name=BubbleContent]:bg-secondary *:data-[name=BubbleContent]:text-secondary-foreground
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--secondary),var(--foreground)_5%)]"
// Muted:
//   "*:data-[name=BubbleContent]:bg-muted
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[color-mix(in_oklch,var(--muted),var(--foreground)_5%)]"
// Tinted:
//   "*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)]
//    *:data-[name=BubbleContent]:text-foreground
//    dark:*:data-[name=BubbleContent]:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)]
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)]
//    dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]"
// Outline:
//   "*:data-[name=BubbleContent]:border-border *:data-[name=BubbleContent]:bg-background
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted
//    [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground
//    dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-input/30"
// Ghost:
//   "border-none *:data-[name=BubbleContent]:rounded-none *:data-[name=BubbleContent]:bg-transparent
//    *:data-[name=BubbleContent]:p-0
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted
//    [&>[data-name=BubbleContent]:is(button,a):hover]:text-foreground
//    dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-muted/50"
// Destructive:
//   "*:data-[name=BubbleContent]:bg-destructive/10 *:data-[name=BubbleContent]:text-destructive
//    dark:*:data-[name=BubbleContent]:bg-destructive/20
//    [&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/20
//    dark:[&>[data-name=BubbleContent]:is(button,a):hover]:bg-destructive/30"

// BubbleContent
"w-fit max-w-full min-w-0 overflow-hidden rounded-xl border border-transparent px-3 py-2 text-sm leading-relaxed wrap-break-word group-data-[align=End]/bubble:self-end [button]:text-left [button,a]:transition-colors [button,a]:outline-none [button,a]:focus-visible:border-ring [button,a]:focus-visible:ring-3 [button,a]:focus-visible:ring-ring/50"

// BubbleReactions (absolute chip strip, positioned by side/align data attrs)
"absolute z-10 flex w-fit shrink-0 items-center justify-center gap-1 rounded-full bg-muted px-1.5 py-0.5 text-sm ring-3 ring-card has-[button]:p-0"
// side=top    → "top-0 -translate-y-3/4"
// side=bottom → "bottom-0 translate-y-3/4"
// align=start → "left-3"
// align=end   → "right-3"
```

### Special: Tinted variant
Raw oklch — copy exactly, Tailwind v4 handles it:
```
bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)]
text-foreground
dark:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)]
hover: bg-[oklch(from_var(--primary)_0.88_calc(c*0.5)_h)]
dark:hover: bg-[oklch(from_var(--primary)_0.35_calc(c*0.5)_h)]
```
Note: `text-foreground` is required on tinted (NOT `text-primary-foreground`).

### BubbleReactions defaults
`side` default = `Bottom`, `align` default = `End` (right-aligned, below bubble).

### BubbleContent implementation — needs `on_click`
shadcn `BubbleContent` uses `asChild` to render as `<button>` (see `bubble-link-button` demo).
We use same `href`/`on_click` split as Marker:

```rust
// TODO PORT: shadcn uses `BubbleContent asChild` (Radix Slot) to swap element type.
// We split into href (→ <a>) and on_click (→ <button>), fallback to <div>.
#[component]
pub fn BubbleContent(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional)] on_click: Option<Callback<ev::MouseEvent>>,
    #[prop(optional, into)] class: String,
    children: Children,
) -> impl IntoView { ... }
```

### TODO PORT notes (Bubble)
- **`BubbleContent asChild`**: shadcn uses `asChild` + Radix Slot. We split into `href`/`on_click` props (same as Marker). `bubble-link-button` demo uses `on_click` for all 3 tinted buttons.
- **`toast()` in demos**: reactions demo and link-button demo call `toast()`. Use `web_sys::window().alert_with_message()`. Add `// TODO PORT:` comment in those demos.
- **Variant classes on Bubble target BubbleContent via `*:data-[name=BubbleContent]`**: parent→child CSS targeting. Verify it renders correctly — if not, move variant classes directly to `BubbleContent`.

### Demos (9) — exact shadcn MDX demos
| File | shadcn source | Shows |
|------|---------------|-------|
| `demo_bubble.rs` | `bubble-demo` | Default bubble with reactions |
| `demo_bubble_variants.rs` | `bubble-variants` | All 7 variants with text |
| `demo_bubble_alignment.rs` | `bubble-alignment` | Start vs End alignment |
| `demo_bubble_grouped.rs` | `bubble-group-demo` | BubbleGroup with consecutive bubbles |
| `demo_bubble_link_button.rs` | `bubble-link-button` | Tinted on_click buttons (forgot password, etc.) |
| `demo_bubble_reactions.rs` | `bubble-reactions` | Emoji reactions + button reaction (Yes, run it) |
| `demo_bubble_collapsible.rs` | `bubble-collapsible` | Bubble + Collapsible (show more/less) |
| `demo_bubble_tooltip.rs` | `bubble-tooltip` | Bubble wrapped in Tooltip |
| `demo_bubble_popover.rs` | `bubble-popover` | Bubble paired with Popover |

---

## Component 4 — Attachment (`ui/attachment.rs`)

Most complex. Two style axes (size + orientation) + state machine + media variant.

### Composition
```
AttachmentGroup       — clx! div
Attachment            — #[component], size + orientation + state props
  AttachmentMedia     — #[component], variant: Icon|Image
  AttachmentContent   — clx! div
    AttachmentTitle       — clx! span  ← shimmer class auto-applied via CSS group selector
    AttachmentDescription — clx! span
  AttachmentActions   — clx! div
    AttachmentAction  — #[component], wraps Button (ghost + icon-xs defaults)
  AttachmentTrigger   — #[component], renders <a> or <button>
```

### CSS classes
```rust
// AttachmentGroup
"flex min-w-0 scroll-fade-x snap-x snap-mandatory scroll-px-1 scrollbar-none gap-3 overflow-x-auto overscroll-x-contain py-1 *:data-[name=Attachment]:flex-none *:data-[name=Attachment]:snap-start"

// Attachment base
"group/attachment relative flex w-fit max-w-full min-w-0 shrink-0 flex-wrap rounded-xl border bg-card text-card-foreground transition-colors focus-within:ring-1 focus-within:ring-ring/50 has-[>a,>button]:hover:bg-muted/50 data-[state=Error]:border-destructive/30 data-[state=Idle]:border-dashed"
// size=Default → "gap-2 text-sm has-data-[name=AttachmentContent]:px-2.5 has-data-[name=AttachmentContent]:py-2 has-data-[name=AttachmentMedia]:p-2"
// size=Sm      → "gap-2.5 text-xs has-data-[name=AttachmentContent]:px-2 has-data-[name=AttachmentContent]:py-1.5 has-data-[name=AttachmentMedia]:p-1.5"
// size=Xs      → "gap-1.5 rounded-lg text-xs has-data-[name=AttachmentContent]:px-1.5 has-data-[name=AttachmentContent]:py-1 has-data-[name=AttachmentMedia]:p-1"
// orientation=Horizontal → "min-w-40 items-center"
// orientation=Vertical   → "w-24 flex-col has-data-[name=AttachmentContent]:w-30"

// AttachmentMedia base
// Note: Spinner has no data-name in our project — target via SVG selector instead
"relative flex aspect-square w-10 shrink-0 items-center justify-center overflow-hidden rounded-lg bg-muted text-foreground group-data-[orientation=Vertical]/attachment:w-full group-data-[size=Sm]/attachment:w-8 group-data-[size=Xs]/attachment:w-7 group-data-[size=Xs]/attachment:rounded-md group-data-[state=Error]/attachment:bg-destructive/10 group-data-[state=Error]/attachment:text-destructive [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 group-data-[orientation=Vertical]/attachment:[&_svg:not([class*='size-'])]:size-6 group-data-[size=Xs]/attachment:[&_svg:not([class*='size-'])]:size-3.5"
// variant=image → "opacity-60 group-data-[state=Done]/attachment:opacity-100 group-data-[state=Idle]/attachment:opacity-100 *:[img]:aspect-square *:[img]:w-full *:[img]:object-cover"

// AttachmentContent (NOT flex-col — just flex-1 with leading-tight)
"max-w-full min-w-0 flex-1 leading-tight group-data-[orientation=Vertical]/attachment:px-1"

// AttachmentTitle
"block max-w-full min-w-0 truncate font-medium group-data-[state=Processing]/attachment:shimmer group-data-[state=Uploading]/attachment:shimmer"

// AttachmentDescription
"mt-0.5 block min-w-0 truncate text-xs text-muted-foreground group-data-[state=Error]/attachment:text-destructive/80 max-w-full"

// AttachmentActions
"relative z-20 flex shrink-0 items-center group-data-[orientation=Vertical]/attachment:absolute group-data-[orientation=Vertical]/attachment:top-3 group-data-[orientation=Vertical]/attachment:right-3 group-data-[orientation=Vertical]/attachment:gap-1"

// AttachmentTrigger
"absolute inset-0 z-10 outline-none"
```

### AttachmentTrigger implementation
```rust
#[component]
pub fn AttachmentTrigger(
    #[prop(optional, into)] href: Option<String>,
    #[prop(optional, into)] aria_label: String,
    #[prop(optional, into)] class: String,
) -> impl IntoView {
    let merged = tw_merge!("absolute inset-0 z-10 outline-none", class);
    if let Some(href) = href {
        view! { <a href=href aria-label=aria_label class=merged /> }.into_any()
    } else {
        view! { <button type="button" aria-label=aria_label class=merged /> }.into_any()
    }
}
```

### TODO PORT notes (Attachment)
- **`AttachmentTrigger` is plain `<button>` by default**: shadcn demo wraps it with `<DialogTrigger asChild><AttachmentTrigger /></DialogTrigger>`. In Leptos, our Dialog trigger wraps content differently — `AttachmentTrigger` stays as a plain `<button>` with `on_click` for the dialog case. For the demo, use our project's Dialog component. Add `// TODO PORT:` in `demo_attachment_trigger.rs`.
- **`AttachmentAction` wraps `Button`**: shadcn passes props to Button (ghost, icon-xs defaults). In Leptos, render our `<Button variant=Ghost size=IconXs>` directly inside `AttachmentAction`. Add `// TODO PORT:` in `attachment.rs`.
- **Spinner in `AttachmentMedia`**: shadcn targets it via `*:data-[slot=spinner]:size-6!`. Our Spinner has no `data-name` attr (it's a `#[component]` rendering `<Loader>` SVG). Use SVG selector `[&_svg]:size-6` in the group-data-[orientation=Vertical] case.

### Demos (6) — exact shadcn MDX demos
| File | shadcn source | Shows |
|------|---------------|-------|
| `demo_attachment.rs` | `attachment-demo` | PDF + code file + zip with actions (X button) |
| `demo_attachment_image.rs` | `attachment-image` | Image variant, vertical orientation |
| `demo_attachment_states.rs` | `attachment-states` | Idle / Uploading (Spinner) / Processing / Error / Done |
| `demo_attachment_sizes.rs` | `attachment-sizes` | Default / Sm / Xs |
| `demo_attachment_group.rs` | `attachment-group` | Horizontal scrollable AttachmentGroup |
| `demo_attachment_trigger.rs` | `attachment-trigger` | Full-card trigger opening Dialog |

---

## Component 5 — MessageScroller (`ui/message_scroller.rs`)

No npm equivalent exists in Leptos — we implement from scratch, closely matching shadcn's scroll engine.

### shadcn implementation deep-dive

Source: `packages/react/src/message-scroller/` — NOT a thin wrapper, a full scroll engine.

**Scroll mode state machine (internal):**
```
following-bottom    — autoScroll=true and at bottom; scrolls with new content
free-scrolling      — reader scrolled away; position frozen
anchored-to-message — holding a streaming turn at reading line while it streams
settling-jump       — programmatic jump animating; user intent suppressed
```

**What triggers `userScrollIntent()` → releases follow-bottom:**
- `on:wheel`, `on:touchmove`
- `on:keydown` for `ArrowUp/Down`, `PageUp/Down`, `Home`, `End`, `Space`

**DOM observers:**
- `MutationObserver` on Content watching `childList: true` → `handleContentChange()`
- `ResizeObserver` on Viewport → `handleResize()`
- `ResizeObserver` on Content → `handleResize()`
- `IntersectionObserver` on items with `messageId` → visibility tracking
- All state commits batched in `requestAnimationFrame`

**`data-*` attributes shadcn writes to Root + Viewport:**
```
data-scrollable="start end"   — both edges have overflow
data-scrollable="end"         — only bottom has overflow
(absent)                      — no overflow
data-autoscrolling            — presence attr, set during programmatic scrolls, cleared after 180ms
```
CSS uses `[data-autoscrolling]:scrollbar-none` to hide scrollbar during programmatic scrolls.

**Spacer:** Hidden `<div data-message-scroller-spacer hidden />` at end of Content.  
Used for anchor math when `scrollPreviousItemPeek` (default 64px) adds top offset.

**`scrollAnchor` item handling:**
- `handleContentChange()` detects new anchored items in the diff (new items with `data-scroll-anchor="true"`)
- Calls `scrollToElement(anchor, { align: "start" }, { keepPreviousPeek: true })`
- keepPreviousPeek = keep 64px of the previous message visible above the new anchor
- If multiple new anchors arrive at once (batch) while `following-bottom` → just `scrollToEnd`

**`MessageScrollerButton`:**
- `active` = `stateStore.end` (overflow exists below)
- Renders with `inert` attr (not `pointer-events-none`) when inactive
- `data-active="true|false"`

---

### Composition
```
MessageScrollerProvider  — provide_context, owns all refs + signals
MessageScroller          — outer frame div; data-slot="message-scroller"; data-scrollable + data-autoscrolling
  MessageScrollerViewport  — data-slot="message-scroller-viewport"; NodeRef; ResizeObserver; on:scroll/wheel/keydown/touchmove
    MessageScrollerContent — data-slot="message-scroller-content"; MutationObserver; ResizeObserver; role="log" aria-relevant="additions"
      MessageScrollerItem  — data-slot="message-scroller-item"; data-scroll-anchor="true|false"; data-message-id
  MessageScrollerButton    — data-slot="message-scroller-button"; data-direction; data-variant; data-size
                             renders <Button variant=Secondary size=IconSm> as inner element
                             data-active="true|false"; inert attr when inactive
                             children: <ArrowDownIcon /> + <span class="sr-only">Scroll to end</span>
```

---

### Context struct
```rust
#[derive(Clone)]
struct MessageScrollerCtx {
    scrollable_start: RwSignal<bool>,  // overflow above (drives start button)
    scrollable_end: RwSignal<bool>,    // overflow below (drives end button)
    auto_scrolling: RwSignal<bool>,    // true during programmatic scrolls

    viewport_ref: NodeRef<leptos::html::Div>,
    root_ref: NodeRef<leptos::html::Div>,

    sync_after_scroll: Callback<()>,   // called from Viewport on:scroll
    user_scroll_intent: Callback<()>,  // called from wheel/touch/keys
    handle_content_change: Callback<()>, // called from MutationObserver in Content
}
```

---

### CSS classes
```rust
// MessageScroller (outer frame)
"group/message-scroller relative flex size-full min-h-0 flex-col overflow-hidden"

// MessageScrollerViewport
// Note: data-autoscrolling:scrollbar-none is a Tailwind v4 data variant — goes IN the class string
"size-full min-h-0 min-w-0 scroll-fade-b scrollbar-thin scrollbar-gutter-stable overflow-y-auto overscroll-contain contain-content data-autoscrolling:scrollbar-none"

// MessageScrollerContent
"flex h-max min-h-full flex-col gap-8"

// MessageScrollerItem
"min-w-0 shrink-0 [contain-intrinsic-size:auto_10rem] [content-visibility:auto]"

// MessageScrollerButton — renders as <Button variant=Secondary size=IconSm> via render prop
// direction prop: "end" (default) | "start"
// data-direction, data-variant, data-size attrs written
"absolute inset-s-1/2 -translate-x-1/2 border-border bg-background text-foreground
 transition-[translate,scale,opacity] duration-200
 hover:bg-muted hover:text-foreground
 data-[active=false]:pointer-events-none
 data-[active=false]:scale-95
 data-[active=false]:opacity-0
 data-[active=false]:duration-400
 data-[active=false]:ease-[cubic-bezier(0.7,0,0.84,0)]
 data-[active=true]:translate-y-0
 data-[active=true]:scale-100
 data-[active=true]:opacity-100
 data-[active=true]:ease-[cubic-bezier(0.23,1,0.32,1)]
 data-[direction=end]:bottom-4
 data-[direction=end]:data-[active=false]:translate-y-full
 data-[direction=start]:top-4
 data-[direction=start]:data-[active=false]:-translate-y-full
 rtl:translate-x-1/2
 data-[direction=start]:[&_svg]:rotate-180"
```

---

### v1 Logic (Leptos — close to shadcn)

**Mode tracking** (use `StoredValue<ScrollMode>`, not reactive):
```rust
enum ScrollMode { FollowingBottom, FreeScrolling, AnchoredToMessage }
```

**`handle_content_change()`** (called by MutationObserver):
```rust
let viewport = viewport_ref.get_untracked().unwrap();
let at_bottom = viewport.scroll_top() as f64
    + viewport.client_height() as f64
    >= viewport.scroll_height() as f64 - 8.0;  // DEFAULT_SCROLL_EDGE_THRESHOLD = 8

// following-bottom: scroll to end
if auto_scroll && mode == FollowingBottom {
    viewport.set_scroll_top(viewport.scroll_height());
    set_auto_scrolling(true, clear after 180ms);
    return;
}

// Check new scroll_anchor items → scroll_to_element(anchor, peek=64)
// Otherwise just update scrollable signals
```

**`sync_after_scroll()`** (called on:scroll):
```rust
// Recompute scrollable_start, scrollable_end, reconcile mode
let at_top = viewport.scroll_top() <= 8;
let at_end = scroll_top + client_height >= scroll_height - 8;
scrollable_start.set(!at_top);
scrollable_end.set(!at_end);

if auto_scroll && at_end && mode == FreeScrolling {
    mode = FollowingBottom;
}
if mode == FollowingBottom && !at_end && !auto_scrolling {
    mode = FreeScrolling;
}
```

**`user_scroll_intent()`**:
```rust
if mode != FreeScrolling { mode = FreeScrolling; }
```

**`scroll_to_element(el, peek_px=64)`**:
```rust
let target = el.offset_top() as f64 - peek_px as f64;
viewport.set_scroll_top(target as i32);
```

**MutationObserver setup** (in `MessageScrollerContent` Effect):
```rust
use wasm_bindgen::prelude::*;
use web_sys::{MutationObserver, MutationObserverInit};

let cb = Closure::wrap(Box::new(move |_: js_sys::Array, _: MutationObserver| {
    ctx.handle_content_change.call(());
}) as Box<dyn FnMut(_, _)>);

let observer = MutationObserver::new(cb.as_ref().unchecked_ref()).unwrap();
let mut init = MutationObserverInit::new();
init.child_list(true);
observer.observe_with_options(&content_el, &init).unwrap();
cb.forget();
// store observer in StoredValue for disconnect on cleanup
```

**ResizeObserver setup** (same pattern, in Viewport and Content Effects).

---

### TODOs (v2+)
```
TODO: scrollToMessage(id: String)
MessageScrollerItem registers itself in context HashMap<String, HtmlDivElement>.
Scroll viewport: target.offset_top() - scroll_margin.

TODO: scrollToStart / scrollToEnd public API
Expose Callback<()> from MessageScrollerCtx for external callers.

TODO: settling-jump mode
Suppress user_scroll_intent during programmatic smooth scroll (~200ms timer).

TODO: useMessageScrollerVisibility via IntersectionObserver
web_sys::IntersectionObserver on items with messageId.
Expose RwSignal<Vec<String>> of visible ids via context.

TODO: defaultScrollPosition = "last-anchor"
On first render with items, find last data-scroll-anchor="true" item and open there.

TODO: preserveScrollOnPrepend
Snapshot offset_top of first visible item before prepend, restore after.
Needed for infinite scroll upward (loading history).
```

### Demo — `demo_message_scroller.rs`

Simulates streaming like shadcn's `useChat` mock transport:
- `RwSignal<Vec<ChatMessage>>` for full message list
- `RwSignal<Option<String>>` for streaming chunk being assembled
- `RwSignal<ChatStatus>` enum: `Idle | Streaming | Done`
- `RwSignal<Option<i32>>` for interval handle
- On send: pick next scripted message, start interval, append tokens at ~10ms/token
- Stop button: `clear_interval_with_handle(id)`, set status = Done
- Same scripted Q&A conversation as shadcn demo (multi-turn, markdown in responses)
- `MessageScrollerItem` with `scroll_anchor=true` on user messages
- Card layout: header + MessageScroller + InputGroup footer

---

## Build Order

```
1. marker.rs           — no new component deps
2. message.rs          — no new component deps
3. bubble.rs           — no new component deps
4. attachment.rs       — depends on Button
5. message_scroller.rs — depends on Button; demos compose all 4 above
```

Add CSS utilities (Step 0) before starting component 1.  
Full-thread demos (`demo_message_group_chat`, `demo_message_scroller`) built last — they compose all 5 components.

---

## Register Steps (repeat per component)

1. Create `ui/component.rs`
2. Add `pub mod component;` to `ui/mod.rs` (alphabetical)
3. Create demo files `demos/demo_component_*.rs`
4. Add demo mods to `demos/mod.rs`
5. Create `public/docs/components/component.md`
6. `cargo run` from `build_registry/` — regenerates `public/registry/*` + `__registry__`
7. Update `public/docs/changelog.md` — group entries by month (`## Month YYYY`), never one section per component. If a month section exists, add new bullet under it. Use `_placeholder.webp` / `_placeholder-dark.webp` for component doc images when no specific thumbnail exists.
