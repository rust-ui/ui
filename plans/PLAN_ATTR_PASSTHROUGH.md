# Plan: Attribute Passthrough Parity With Leptos

Goal: every `app_crates/registry/src/ui/*.rs` component accepts arbitrary
forwarded attributes, the way the Leptos registry does, so call sites can pass
`aria-label`, `id`, `role`, `data-*`, `href`, `disabled`, `value`, ... straight
onto a component and have it land on the component's root element.

## Why this gap exists

Leptos 0.7 auto-forwards any `attr:foo=bar` written on a component onto that
component's root node. No component code is needed.

Dioxus 0.7 does not auto-forward. Each component must opt in:

```rust
#[component]
pub fn NavMenuList(
    #[props(into, optional)] class: Option<String>,
    #[props(extends = GlobalAttributes, extends = menu)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("...", class.as_deref().unwrap_or(""));
    rsx! {
        menu { "data-name": "NavMenuList", class: "{merged_class}", ..attributes, {children} }
    }
}
```

- `extends = GlobalAttributes` covers `id`, `role`, `style`, `title`,
  `tabindex`, `hidden`, and every `data-*` / `aria-*`.
- add `extends = <element>` (e.g. `extends = a`, `extends = input`,
  `extends = button`) for element-specific attributes (`href`, `target`, `rel`,
  `disabled`, `value`, `placeholder`, `name`, `min`, `max`, `step`, `rows`,
  `readonly`, `required`, `autocomplete`, `autofocus`, `r`, ...).
- `..attributes` goes on the component's real root element, after the fixed
  attributes so callers can override, before `{children}`.

Count in scope: 88 files, ~472 `#[component]` fns. Current coverage: 0.

## Call-site translation

Leptos and Dioxus spell forwarded attributes differently. When porting a Leptos
call site:

| Leptos | Dioxus |
| --- | --- |
| `attr:data-orientation="horizontal"` | `"data-orientation": "horizontal"` |
| `attr:aria-label="home"` | `"aria-label": "home"` |
| `attr:id=some_id` | `id: some_id` |
| `attr:disabled=is_disabled` | `disabled: is_disabled` |
| `attr:href=url` | `href: url` |

Leptos app currently passes `attr:` to registry components in ~800 spots. Most
frequent keys: `href` (168), `aria-label` (164), `id` (85), `placeholder` (57),
`role` (50), `r` (45), `src` (32), `alt` (32), `disabled` (27),
`data-sidenav` (25), `tabindex` (23), `data-name` (21), `value` (20),
`data-state` (20), `title` (18), `target` (16), `style` (15), `rel` (15),
`data-orientation` (14).

## Caveats (why this is not a blind sed)

1. Component-rooted components. If a component's root in `rsx!` is another
   component (not a lowercase HTML element), `..attributes` cannot spread there
   until that inner component also extends attributes, or the root is wrapped in
   a real element. Affected files below are marked `root: component` or
   `root: mixed`.
2. Prop name collisions. Adding `extends = <el>` brings in attribute names that
   may already be explicit props on the component (`class`, `value`,
   `disabled`, `checked`, `id`, `name`, `href`). The macro errors on the
   overlap. Options: keep the explicit prop and only `extends = GlobalAttributes`,
   or rename the internal prop, or drop the explicit prop and read it out of
   `attributes`. Decide per component.
3. `data-name` ordering. Every component sets a fixed `"data-name"`; keep it
   first so a caller cannot clobber it, put `..attributes` after the fixed set.
4. Multiple roots / fragments. A component returning a fragment (several
   siblings) has no single root; pick the semantically primary element or add a
   wrapper only if Leptos also wraps.
5. `extends` from multiple elements only works when the elements have no
   conflicting attributes (per Dioxus docs), so `extends = a, extends = button`
   style combos need checking.
6. Generated metadata. `ui/*.rs` are hand-authored; `src/__registry__/` and
   `public/registry/` are generated. Re-run `build_registry_dioxus` after the
   sweep only if public registry snippets embed component signatures (they
   should not, but verify).

## Suggested waves

Group by component family, `cargo check` on all 3 targets + commit + push per
wave.

1. Primitives, element-rooted, no collisions: `badge`, `label`, `image`,
   `input`, `textarea`, `slider`, `separator`, `skeleton`, `shimmer`,
   `progress`, `spinner`, `kbd`, `aspect_ratio`, `callout`, `status`,
   `pressable`, `button_action`, `theme_toggle`, `animate`, `mask`, `marker`.
2. Form family: `button`, `link`, `checkbox`, `switch`, `radio_button`,
   `radio_button_group`, `field`, `form`, `input_group`, `input_otp`,
   `input_phone`, `input_prompt`, `auto_form`, `toggle_group`.
3. Overlay family: `dialog`, `alert_dialog`, `drawer`, `sheet`, `popover`,
   `hover_card`, `tooltip`, `command`.
4. Menu / nav family: `header`, `navigation_menu`, `dropdown_menu`, `menubar`,
   `context_menu`, `select`, `multi_select`, `toolbar`, `bottom_nav`,
   `pagination`, `breadcrumb`, `tabs`, `stepper`, `sidenav`.
5. Layout / content family: `card`, `card_carousel`, `carousel`, `accordion`,
   `collapsible`, `alert`, `empty`, `item`, `message`, `bubble`, `marquee`,
   `scroll_area`, `table`, `avatar`, `chips`, `action_bar`, `attachment`,
   `direction_provider`.
6. Complex / data: `data_grid`, `data_table`, `date_picker`
   (+ `date_picker_state`, `date_picker_dual_state`), `charts`, `sonner`,
   `drag_and_drop`, `dropzone`, `workflow`, `toolbar`.

## Per-file inventory

`root` legend: `element` = every `#[component]` root is an HTML element (clean
spread); `mixed` = some roots are components; `component` = all roots are
components (needs unwrapping or inner extends first).

| file | #cmp | root | notes |
| --- | ---: | --- | --- |
| accordion.rs | 8 | element | roots: a article div h4 input p |
| action_bar.rs | 4 | element | roots: div svg; one `style` root |
| alert.rs | 3 | element | div h4 p |
| alert_dialog.rs | 10 | component | all roots are `Dialog*` wrappers; extend `dialog.rs` first, then forward |
| animate.rs | 3 | element | div |
| aspect_ratio.rs | 1 | element | div |
| attachment.rs | 9 | mixed | one `Button` root; rest a div span |
| auto_form.rs | 1 | element | form |
| avatar.rs | 6 | element | div img span |
| badge.rs | 1 | element | span; watch `variant` prop, GlobalAttributes only |
| bottom_nav.rs | 4 | element | button div nav span |
| breadcrumb.rs | 7 | element | a button li nav ol span; `href` collision on link item |
| bubble.rs | 4 | element | a div |
| button.rs | 1 | component | root is `Link`; extend `link.rs` first; `disabled`/`class` collisions |
| button_action.rs | 1 | element | span |
| button_group.rs | 3 | mixed | one `Separator` root; div span |
| callout.rs | 1 | element | div |
| card.rs | 9 | element | div footer h2 li p ul |
| card_carousel.rs | 9 | element | button div img span |
| carousel.rs | 6 | element | button div; one `style` root |
| charts.rs | 1 | element | div |
| checkbox.rs | 1 | element | button; `checked`/`disabled` collisions, GlobalAttributes only or extend `button` carefully |
| chips.rs | 2 | element | label; one `style` root |
| collapsible.rs | 3 | element | button div |
| command.rs | 15 | mixed | one `Button` root; a div footer h2 input |
| context_menu.rs | 12 | element | a div li span ul; one `style` root |
| data_grid.rs | 19 | mixed | one `DropdownMenu` root; div input span |
| data_table.rs | 0 | - | no `#[component]` here (re-exports) |
| date_picker.rs | 11 | element | button div header span table td |
| date_picker_dual_state.rs | 0 | - | state only |
| date_picker_state.rs | 0 | - | state only |
| dialog.rs | 10 | element | button div footer h3 p; `DialogAction`/`DialogClose` render `button`, `disabled` collision |
| direction_provider.rs | 1 | element | div |
| drag_and_drop.rs | 3 | element | div |
| drawer.rs | 10 | element | button div footer h3 p |
| dropdown_menu.rs | 14 | element | a div li span ul |
| dropzone.rs | 9 | element | div p |
| empty.rs | 6 | element | div h3 p |
| field.rs | 10 | element | div fieldset label legend p |
| footer.rs | 16 | mixed | `Link` roots; a div footer p section |
| form.rs | 14 | mixed | `FormFieldWrapper` / `Label` roots; div fieldset form input |
| header.rs | 18 | mixed | 4 component roots (`Link`); div li menu nav p. This is the file that started the sweep (`NavMenuList` needs `data-orientation` + `dir`) |
| hover_card.rs | 3 | element | div span; one `style` root |
| image.rs | 1 | element | img; extend `img` for `src`/`alt`/`loading` |
| input.rs | 1 | element | input; `value`/`type`/`placeholder`/`disabled` collisions, extend `input` and reconcile explicit props |
| input_group.rs | 6 | element | button div input span textarea |
| input_otp.rs | 4 | element | div |
| input_phone.rs | 3 | component | `CommandItem` / `InputPhoneWrapper` roots |
| input_prompt.rs | 5 | mixed | `Button` / `InputGroup*` roots; div textarea |
| item.rs | 10 | element | a div p |
| kbd.rs | 2 | element | kbd |
| label.rs | 1 | element | label; leptos scopes `peer-disabled/{id}:` (see PLAN_CSS_PARITY); `for`/`id` handling |
| link.rs | 1 | element | button; note leptos `link` renders router `<a>`, dioxus renders `button` (no leptos_router). `href` collision |
| marker.rs | 3 | element | a span |
| marquee.rs | 3 | element | div; one `style` root |
| mask.rs | 2 | element | div |
| menubar.rs | 15 | element | button div li span ul; one `style` root |
| message.rs | 6 | element | div |
| mod.rs | 0 | - | module file |
| multi_select.rs | 8 | element | button div li span ul |
| navigation_menu.rs | 6 | element | a button div li ul |
| pagination.rs | 7 | element | button li nav span ul; see PLAN_CSS_PARITY (renders button not router a) |
| popover.rs | 5 | element | button div h3 p; one `style` root |
| pressable.rs | 1 | element | div |
| progress.rs | 1 | element | div |
| radio_button.rs | 2 | element | button div |
| radio_button_group.rs | 3 | element | fieldset label span |
| scroll_area.rs | 7 | element | div |
| select.rs | 7 | element | button div li span ul |
| separator.rs | 1 | element | div; `role`/`aria-orientation` likely already set |
| sheet.rs | 9 | element | button div footer h2 p |
| shimmer.rs | 1 | element | div |
| sidenav.rs | 23 | element | a aside button div footer input; largest file, many `data-sidenav` in leptos |
| skeleton.rs | 1 | element | div |
| slider.rs | 1 | element | input; extend `input`, reconcile `min`/`max`/`step`/`value` |
| sonner.rs | 4 | mixed | `SonnerContainer` root; button div ol |
| spinner.rs | 2 | component | roots are `Loader` / `LoaderCircle` icon components; wrap or GlobalAttributes on a span |
| status.rs | 2 | element | div |
| stepper.rs | 7 | element | button div span |
| switch.rs | 2 | element | button span; `checked`/`disabled` collisions |
| table.rs | 9 | element | caption div table tbody td tfoot |
| tabs.rs | 4 | element | button div |
| textarea.rs | 1 | element | textarea; extend `textarea`, reconcile `value`/`rows`/`placeholder`/`disabled` |
| theme_toggle.rs | 1 | element | button |
| toggle_group.rs | 3 | element | a button div |
| toolbar.rs | 8 | mixed | one `Separator` root; a button div li ul |
| tooltip.rs | 3 | element | div (2 roots resolved); check fragment |
| workflow.rs | 11 | mixed | one `WfNode` root; div p span |

## Status

Not started. This file is documentation only; no code changes yet.

Maintenance rule (same as the other PLAN files): delete rows as each file
reaches full passthrough; do not keep "done" markers.
