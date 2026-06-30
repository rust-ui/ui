# Plan — Chat Interface Components (shadcn → Leptos/Rust)

> Source: `rust_ui_internals/!!!_OTHERS/SHADCN-UI/apps/v4/`  
> Target: `app_crates/registry/src/`

**Do not modify `ui/chat.rs`** — separate older component, leave it alone.

---

## Files to Create

### UI components
```
app_crates/registry/src/ui/bubble.rs
app_crates/registry/src/ui/message.rs
app_crates/registry/src/ui/marker.rs
app_crates/registry/src/ui/attachment.rs
app_crates/registry/src/ui/message_scroller.rs
```

### Register in `ui/mod.rs`
```rust
pub mod bubble;
pub mod message;
pub mod marker;
pub mod attachment;
pub mod message_scroller;
```

### Docs
```
public/docs/components/bubble.md
public/docs/components/marker.md
public/docs/components/attachment.md
public/docs/components/message.md
public/docs/components/message_scroller.md
```

---

## Rust/Leptos Differences vs shadcn React

### 1. Variants = PascalCase enums

shadcn uses string literals (`"default"`, `"secondary"`).  
We use enums with `#[derive(Default, PartialEq, Clone)]` and map to `data-*` attrs.

```rust
// shadcn: variant="secondary"
// Rust:
#[derive(Default, PartialEq, Clone)]
pub enum BubbleVariant {
    #[default] Default,
    Secondary, Muted, Tinted, Outline, Ghost, Destructive,
}

#[derive(Default, PartialEq, Clone)]
pub enum BubbleAlign { #[default] Start, End }

#[derive(Default, PartialEq, Clone)]
pub enum MarkerVariant { #[default] Default, Separator, Border }

#[derive(Default, PartialEq, Clone)]
pub enum AttachmentState { Idle, Uploading, Processing, Error, #[default] Done }

#[derive(Default, PartialEq, Clone)]
pub enum AttachmentSize { #[default] Default, Sm, Xs }

#[derive(Default, PartialEq, Clone)]
pub enum AttachmentOrientation { #[default] Horizontal, Vertical }

#[derive(Default, PartialEq, Clone)]
pub enum AttachmentMediaVariant { #[default] Icon, Image }

#[derive(Default, PartialEq, Clone)]
pub enum MessageAlign { #[default] Start, End }

#[derive(Default, PartialEq, Clone)]
pub enum BubbleReactionsSide { Top, #[default] Bottom }
```

Each enum implements `as_str() -> &'static str` for the `data-*` attribute value (lowercase).

### 2. `variants!` macro for variant-heavy components

Components with multiple style axes (Bubble, Attachment) use the `variants!` macro like Badge.  
Components that are just layout wrappers use `clx!` macro.

### 3. No `asChild` prop

shadcn uses Radix's `asChild` pattern to polymorphically render a different element.  
Leptos doesn't have this. Alternatives per component:

- **`AttachmentTrigger`**: add `href: Option<String>` prop — renders `<a>` if `Some`, else `<button>`
- **`BubbleReactions`** children can just be spans or buttons directly — no render delegation needed
- **`Marker`** wrapping Drawer/Accordion: just put the trigger component *inside* `<Marker>` as children — works naturally in Leptos

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

### 5. `data-*` attributes in Leptos

```rust
// Set via attr:
<div
    data-slot="bubble"
    attr:data-variant=variant.as_str()
    attr:data-align=align.as_str()
    class=...
>
```

### 6. `MessageScroller` — no `@shadcn/react` primitive package

**This is the biggest difference.**  
shadcn wraps `@shadcn/react/message-scroller` — a React npm package with complex scroll state management. We have no equivalent.

We implement from scratch with Leptos reactive primitives:
- `NodeRef<leptos::html::Div>` for the viewport
- `RwSignal<bool>` for `show_scroll_button`
- `Effect` for auto-scroll on content change
- `on:scroll` for detecting manual scroll position
- Context for exposing `scroll_to_bottom` to child `MessageScrollerButton`

No `useChat` dependency either — demo uses static `RwSignal<Vec<ChatMessage>>`.

---

## Reference Patterns (from codebase)

### Multi-sub-part component pattern (`avatar.rs`)
```rust
// 1. Enums at top
#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum AvatarSize { Sm, #[default] Default, Lg }

// 2. clx! for layout-only sub-parts (mod block)
mod components {
    use super::*;
    clx! { AvatarFallback, div, "absolute inset-0 flex size-full ..." }
}
pub use components::*;

// 3. #[component] for parts needing props/logic
#[component]
pub fn Avatar(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(optional)] size: AvatarSize,
) -> impl IntoView {
    let size_str = match size { AvatarSize::Sm => "sm", AvatarSize::Default => "default", AvatarSize::Lg => "lg" };
    let merged_class = tw_merge!("group/avatar ...", class);
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
use crate::ui::badge::{Badge, BadgeVariant};

#[component]
pub fn DemoBadgeVariants() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-4 items-center @md:flex-row">
            <Badge>Default</Badge>
            <Badge variant=BadgeVariant::Secondary>Secondary</Badge>
        </div>
    }
}
```

### Timer / interval — no leptos_use, use web_sys directly
`leptos_use` is NOT in Cargo.toml. Use raw web_sys (same pattern as `use_press_hold.rs`):
```rust
use wasm_bindgen::prelude::*;
use web_sys::window;

let closure = Closure::wrap(Box::new(move || { /* append token */ }) as Box<dyn FnMut()>);
let id = window().unwrap()
    .set_interval_with_callback_and_timeout_and_arguments_0(
        closure.as_ref().unchecked_ref(), 10
    ).unwrap();
closure.forget();

// Cancel: window().unwrap().clear_interval_with_handle(id);
```
Store `id` in `RwSignal<Option<i32>>` to cancel from stop button.

### Available icons (confirmed in icons crate)
`GitBranch`, `FileSearch`, `FileText`, `Download`, `CircleUserRound`,
`Search`, `Check`, `X`, `ArrowDown`, `Paperclip`, `ArrowUp` — all present.

---

## CSS Prerequisites — Must Add Before Components Work

These utilities appear in the new components but **don't exist in our project yet**.

> **First step**: find the project's Tailwind input CSS file (check `app/style/`, `public/`, `app/src/`) and add `@utility` blocks there.

### `wrap-break-word`
Used in: `BubbleContent`, `Message`, `MessageScrollerContent`
```css
@utility wrap-break-word {
  overflow-wrap: break-word;
  word-break: break-word;
}
```
Fallback: replace class with `[overflow-wrap:break-word]` arbitrary value.

### `scroll-fade-b` / `scroll-fade-x`
Used in: `MessageScrollerViewport` (fade-b), `AttachmentGroup` (fade-x) — cosmetic mask fade at edges.
```css
@utility scroll-fade-b {
  mask-image: linear-gradient(to bottom, black calc(100% - 2rem), transparent 100%);
}
@utility scroll-fade-x {
  mask-image: linear-gradient(to right, black calc(100% - 2rem), transparent 100%);
}
```
Fallback: omit entirely — scroll still works, just no fade effect.

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
Note: project already has `no-scrollbar` (Shimmer component) which is equivalent to `scrollbar-none`.

### `shimmer` CSS text animation utility
Used in: Marker separator demo (`<MarkerContent class="shimmer">Thinking…</MarkerContent>`), Attachment title during upload/processing state.

**This is NOT our `<Shimmer>` skeleton component** — completely different thing.  
shadcn `shimmer` = pure CSS text sweep animation (`background-clip: text` + moving gradient).

Must add to our CSS file. Full definition from `packages/shadcn/src/tailwind.css`:

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

Usage in demos: `class="shimmer text-muted-foreground"` on any text element.

---

## Component 1 — Bubble

### Variants on Bubble (7)
`Default | Secondary | Muted | Tinted | Outline | Ghost | Destructive`

### Special: Tinted variant uses raw oklch CSS
Cannot simplify — must keep the full class string:
```
bg-[oklch(from_var(--primary)_0.93_calc(c*0.4)_h)]
dark:bg-[oklch(from_var(--primary)_0.3_calc(c*0.4)_h)]
```

### BubbleReactions props
- `side: BubbleReactionsSide` (Top | Bottom, default Bottom)
- `align: BubbleAlign` (Start | End, default End)

Styled as absolute emoji chip strip: `absolute z-10 flex rounded-full bg-muted px-1.5 py-0.5`

---

## Component 2: Message (`ui/message.rs`)

### Composition
```
MessageGroup   — clx!  "flex min-w-0 flex-col gap-2"
Message        — component, align: MessageAlign prop (Start|End)
  MessageAvatar  — clx!  "flex w-fit min-w-8 shrink-0 ... rounded-full bg-muted"
  MessageContent — clx!  "flex w-full min-w-0 flex-col gap-2.5 wrap-break-word"
  MessageHeader  — clx!  "flex ... px-3 text-xs font-medium text-muted-foreground"
  MessageFooter  — clx!  "flex ... px-3 text-xs font-medium text-muted-foreground"
```

`Message` sets `data-align` attr → CSS `data-[align=end]:flex-row-reverse` flips to right-aligned.  
All sub-parts that read parent state do so via CSS group selectors (no Leptos context needed).

---

## Component 3: Marker (`ui/marker.rs`)

### Composition
```
Marker        — component, variant: MarkerVariant prop
  MarkerIcon    — clx! span, "size-4 shrink-0"
  MarkerContent — clx! span, "min-w-0 wrap-break-word"
```

### Variants (3)
- `Default` — plain row
- `Separator` — adds `before`/`after` pseudo `h-px bg-border` lines (Tailwind `before:` / `after:`)
- `Border` — adds `border-b border-border pb-2`

---

## Component 4: Attachment (`ui/attachment.rs`)

Most complex. Two variant axes + a state machine + media variants.

### Composition
```
AttachmentGroup       — clx! div, horizontal scroll container w/ snap
Attachment            — component, size + orientation + state props → data-* attrs
  AttachmentMedia     — component, variant: Icon|Image
  AttachmentContent   — clx! div, text content wrapper
    AttachmentTitle       — clx! span
    AttachmentDescription — clx! span, truncated metadata line
  AttachmentActions   — clx! div, action button strip
    AttachmentAction  — wraps Button with ghost/icon-xs defaults
  AttachmentTrigger   — button/anchor over the whole card (absolute inset-0 z-10)
```

### Props on `Attachment`
```rust
state: AttachmentState          // data-state → CSS for error/idle/done styling
size: AttachmentSize            // data-size → padding/icon sizes via CSS
orientation: AttachmentOrientation  // data-orientation → horizontal/vertical layout
```

### `AttachmentTrigger` Rust adaptation
shadcn uses `asChild` to render as `<a>` for links. We use:
```rust
#[prop(optional, into)] href: Option<String>
// if href.is_some() → render <a href=href>, else → render <button type="button">
```

### CSS complexity
Many classes on `AttachmentMedia` use chained group selectors:
```
group-data-[orientation=vertical]/attachment:w-full
group-data-[size=sm]/attachment:w-8
group-data-[state=error]/attachment:bg-destructive/10
```
These can be copied as-is — Tailwind parses them correctly.

---

## Component 5: MessageScroller (`ui/message_scroller.rs`)

### Composition (exact same API as shadcn)
```
MessageScrollerProvider  — provides Context, owns signals
MessageScroller          — outer styled frame div
  MessageScrollerViewport  — scrollable div (NodeRef, on:scroll)
    MessageScrollerContent — inner flex-col container (gap-8)
      MessageScrollerItem  — row wrapper, optional scrollAnchor
  MessageScrollerButton    — "scroll to bottom" floating button
```

### Leptos implementation plan

```rust
// Context struct
#[derive(Clone)]
struct MessageScrollerCtx {
    show_button: RwSignal<bool>,
    scroll_to_bottom: Callback<()>,
}
```

**`MessageScrollerProvider`**:
- Creates `RwSignal<bool> show_button = false`
- Creates `NodeRef<Div> viewport_ref`
- Creates `scroll_to_bottom` callback: reads `viewport_ref`, calls `.set_scroll_top(el.scroll_height())`
- Provides context

**`MessageScrollerViewport`**:
- Reads `viewport_ref` from context
- `on:scroll` handler: check `el.scroll_top() + el.client_height() >= el.scroll_height() - 50` → set `show_button`

**`MessageScrollerItem`**:
- Props: `#[prop(optional)] scroll_anchor: bool`, `#[prop(optional, into)] message_id: String`
- If `scroll_anchor = true`: when this item mounts, trigger a "scroll to keep in view" behavior via Effect

**`MessageScrollerButton`**:
- Reads context, shows/hides based on `show_button`
- `on:click` → calls `scroll_to_bottom`
- Animated: `opacity-0 scale-75 → opacity-100 scale-100` based on signal

### scrollAnchor behavior (basic implementation)

shadcn: when `MessageScrollerItem` has `scrollAnchor=true` (set on user turns), the primitive positions the viewport so that item appears near the top with `scrollPreviousItemPeek` px of the previous turn still visible above it.

We implement a basic version:
- `MessageScrollerItem` with `scroll_anchor=true` fires an `Effect` on mount
- Effect reads `NodeRef` of the item, calls `.scroll_into_view_with_bool(true)` (aligns to top)
- No `scrollPreviousItemPeek` offset in v1 — TODO below

```
TODO: implement scrollPreviousItemPeek
Currently scroll_anchor snaps item to viewport top with no offset.
shadcn keeps N px of previous turn visible above the anchored row.
Fix: instead of scroll_into_view, manually compute scroll_top =
  item.offset_top() - peek_px, then set on viewport NodeRef.
Needs peek_px prop on MessageScrollerProvider.
```

### What shadcn's primitive does that we skip entirely v1:
```
TODO: scrollToMessage(id) — jump to specific MessageScrollerItem by message_id
Currently not implemented. To add: store HashMap<String, NodeRef> in context,
expose scroll_to_message(id: String) from MessageScrollerCtx.

TODO: scrollToStart / scrollToEnd imperative commands from outside the component
Currently only the floating button scrolls to end.
To add: expose from context so external controls (toolbar, search) can call them.

TODO: useMessageScrollerVisibility — track which items are in the viewport
Not implemented. Would need IntersectionObserver (web_sys::IntersectionObserver).

TODO: Position preservation when prepending older messages (infinite scroll up)
When new items are prepended, browser shifts scroll position.
Fix: snapshot scroll_height before prepend, restore scroll_top after.

TODO: content-visibility: auto virtualization for long threads
MessageScrollerItem has [content-visibility:auto] class from shadcn.
This works for rendering perf but full virtualization (unmounting) not implemented.
```

---

## Demos Plan

### Bubble demos (8)
| File | Shows |
|------|-------|
| `demo_bubble.rs` | Default bubble, basic usage |
| `demo_bubble_variants.rs` | All 7 variants with description text |
| `demo_bubble_alignment.rs` | Start (assistant) vs End (user) |
| `demo_bubble_grouped.rs` | Thread: MessageGroup → Message → BubbleGroup → multiple Bubbles |
| `demo_bubble_reactions.rs` | BubbleReactions with emoji spans |
| `demo_bubble_reactions_buttons.rs` | BubbleReactions with clickable button emojis + toast |
| `demo_bubble_collapsible.rs` | Bubble wrapping Collapsible (tool-call expand) |
| `demo_bubble_button_links.rs` | Bubble containing buttons and anchor links |

### Marker demos (5)
| File | Shows |
|------|-------|
| `demo_marker.rs` | Default: user joined/left events |
| `demo_marker_border.rs` | Border: git branch switch, file review, notes opened |
| `demo_marker_separator.rs` | Separator: with Spinner, shimmer text, button, check icon |
| `demo_marker_accordion.rs` | Separator wrapping Accordion (explored N files) |
| `demo_marker_drawer.rs` | Separator with Drawer trigger (file activity) |

### Attachment demos (8)
| File | Shows |
|------|-------|
| `demo_attachment.rs` | File attachments: PDF, code file, zip |
| `demo_attachment_content_only.rs` | No media, content-only layout |
| `demo_attachment_states.rs` | Idle / Uploading (spinner) / Processing / Error / Done |
| `demo_attachment_images.rs` | Image variant, vertical orientation |
| `demo_attachment_image_states.rs` | Image upload states |
| `demo_attachment_sizes.rs` | Default / Sm / Xs |
| `demo_attachment_group.rs` | Horizontal scrollable AttachmentGroup (snap scroll) |
| `demo_attachment_triggers.rs` | With remove action + Dialog trigger |

### Message demos (8)
| File | Shows |
|------|-------|
| `demo_message.rs` | Basic message with Bubble |
| `demo_message_avatar.rs` | With Avatar component |
| `demo_message_group.rs` | MessageGroup: multiple sequential bubbles, avatar hidden |
| `demo_message_group_chat.rs` | Full thread: sent + received sides |
| `demo_message_header_footer.rs` | With timestamp header + action footer |
| `demo_message_actions.rs` | Footer with copy/retry/react buttons |
| `demo_message_attachment.rs` | With Attachment inside MessageContent |
| `demo_message_attachment_group.rs` | With AttachmentGroup (multiple files) |

### MessageScroller demos (1 for v1)
| File | Shows |
|------|-------|
| `demo_message_scroller.rs` | Static chat thread, auto-scroll, scroll-to-bottom button |

shadcn demo uses `useChat` from `@ai-sdk/react` with a mock scripted transport that streams token-by-token with delays.  
We match this as closely as possible:
- `RwSignal<Vec<ChatMessage>>` for message list
- `RwSignal<String>` for current streaming chunk (appended token by token)
- `RwSignal<ChatStatus>` enum: `Idle | Streaming | Done`
- `set_interval` (from `leptos_use`) to simulate chunked token arrival at ~10ms/chunk
- User sends message → status = Streaming → tokens append to last assistant message → status = Done
- Same scripted conversation as shadcn (pre-defined Q&A pairs, sequential)
- Stop button cancels the interval and sets status = Done

---

## Build Order

```
1. marker.rs          — no new deps
2. message.rs         — no new deps
3. bubble.rs          — no new deps
4. attachment.rs      — depends on Button
5. message_scroller.rs — depends on Button; demos compose all above
```

Demos for full thread (`demo_message_group_chat`, `demo_message_scroller`) built last.

---

## Register Steps (repeat per component)

1. Create `ui/component.rs`
2. `pub mod component;` → `ui/mod.rs`
3. Create demo files `demos/demo_component_*.rs`
4. Add demo mods → `demos/mod.rs`
5. Create `public/docs/components/component.md`
6. `cargo run` from `build_registry/` to regenerate registry JSON + `__registry__`
7. Update `public/docs/changelog.md`
