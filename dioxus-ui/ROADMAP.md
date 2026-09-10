# Dioxus UI — Roadmap

Porting components from [rust-ui.com](https://www.rust-ui.com) (Leptos) to Dioxus.

**Source of truth: RUST-UI.** Demos and `public/docs/` md files must match RUST-UI exactly.

Each ported component requires:
- `src/ui/<component>.rs`
- `src/registry/<component>.rs`
- `public/docs/<component>.md`
- `src/demos/demo_<component>*.rs` — matching RUST-UI exactly

After adding or removing demos, run `build_registry` to regenerate `src/demos/mod.rs`, `src/registry/mod.rs`, and validate that every `<DemoXxx />` tag in `public/docs/` has a matching demo file:

```
cargo run --manifest-path dioxus_ui_internals/build_registry/Cargo.toml
```

---

## Inputs & Forms

- [x] Button (`button`, `button_group`)
- [x] Input
- [x] Textarea
- [x] Select Native
- [x] Checkbox
- [x] Switch
- [x] Slider
- [x] Radio Button (`radio_group`)
- [ ] Select
- [ ] Combobox
- [ ] Multi Select
- [ ] Date Picker
- [ ] Input Group
- [ ] Input OTP
- [ ] Input Phone
- [ ] Field
- [ ] Form
- [ ] Auto Form

## Display

- [x] Badge
- [x] Card
- [x] Separator
- [x] Avatar
- [x] Table
- [x] Alert
- [x] Kbd
- [x] Label
- [x] Chips
- [x] Status
- [x] Empty
- [ ] Callout

## Feedback / Loading

- [x] Skeleton
- [x] Spinner
- [x] Progress
- [ ] Shimmer
- [ ] Animate
- [ ] Animate Group
- [ ] Toast
- [ ] Sonner

## Navigation

- [x] Breadcrumb
- [x] Tabs
- [x] Pagination
- [ ] Bottom Nav
- [ ] Navigation Menu
- [ ] Menubar

## Overlay

- [x] Dialog
- [x] Alert Dialog
- [x] Tooltip
- [ ] Drawer
- [ ] Sheet
- [ ] Popover
- [ ] Hover Card
- [ ] Dropdown Menu
- [ ] Context Menu
- [ ] Command

## Layout

- [x] Accordion
- [x] Collapsible
- [ ] Carousel
- [ ] Card Carousel
- [ ] Scroll Area
- [ ] Aspect Ratio
- [ ] Item

## Utilities

- [x] Theme Toggle
- [x] Toggle Group
- [ ] Toggle
- [ ] Pressable
- [ ] Marquee
- [ ] Drag and Drop
- [ ] Dropzone
- [ ] Direction Provider
