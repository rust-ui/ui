# Missing UI Components (registry vs codebase)

Registry: 83 components | We have: ~61 | Missing: 22

## Missing (22)

- action_bar
- auto_form
- button_action
- charts
- command
- context_menu
- data_grid
- data_table
- date_picker
- date_picker_dual_state
- date_picker_state
- drag_and_drop
- dropdown_menu
- form
- header
- input_phone
- link
- menubar
- multi_select
- navigation_menu
- select
- sonner

## We Have (61 matched to registry)

accordion, alert, alert_dialog, animate, aspect_ratio, avatar, badge, bento_grid, bottom_nav,
breadcrumb, button, button_group, callout, card, card_carousel, carousel, chat, checkbox, chips,
collapsible, dialog, direction_provider, drawer, empty, expandable, faq_transition, field, footer,
hover_card, image, input, input_group, input_otp, item, kbd, label, marquee, mask, pagination,
popover, pressable, progress, radio_button, radio_button_group, select_native, separator, sheet,
shimmer, sidenav, skeleton, slider, spinner, status, switch, table, tabs, textarea, theme_toggle,
toggle_group, tooltip, scroll_area

## Custom (not in registry)

command_bar, demo_wrapper, doc_header, navbar, seo_meta, toc, toggle

## Skipped (blocked)

- footer — naming conflict with existing site layout component
- link — requires dioxus-router (not in Cargo.toml)
- docker — registry source unavailable
