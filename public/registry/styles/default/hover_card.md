---
title: "Hover Card"
name: "hover_card"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/hover_card.rs"
---

# Hover Card

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add hover_card
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id;

/* ========================================================== */
/*                     ✨ TYPES ✨                            */
/* ========================================================== */

#[derive(Clone, Copy, PartialEq, Eq, Default)]
pub enum HoverCardSide {
    Top,
    #[default]
    Bottom,
    Left,
    Right,
}

#[derive(Clone)]
struct HoverCardContext {
    anchor_name: String,
    trigger_id: String,
    content_id: String,
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn HoverCard(children: Children, #[prop(default = HoverCardSide::default())] side: HoverCardSide) -> impl IntoView {
    let id = use_random_id();
    let anchor_name = format!("--hc_anchor_{}", id);
    let trigger_id = format!("hc_trigger_{}", id);
    let content_id = format!("hc_content_{}", id);

    let (position_styles, transform_origin) = match side {
        HoverCardSide::Bottom => ("position-area: block-end; margin-top: 8px;".to_string(), "center top".to_string()),
        HoverCardSide::Top => {
            ("position-area: block-start; margin-bottom: 8px;".to_string(), "center bottom".to_string())
        }
        HoverCardSide::Left => {
            ("position-area: inline-start; margin-right: 8px;".to_string(), "right center".to_string())
        }
        HoverCardSide::Right => ("position-area: inline-end; margin-left: 8px;".to_string(), "left center".to_string()),
    };

    let ctx = HoverCardContext {
        anchor_name: anchor_name.clone(),
        trigger_id: trigger_id.clone(),
        content_id: content_id.clone(),
    };

    view! {
        <leptos::context::Provider value=ctx>
            <style>
                {format!(
                    "
                    #{content_id} {{
                        position-anchor: {anchor_name};
                        inset: auto;
                        {position_styles}
                        position-try-fallbacks: flip-block;
                        position-try-order: most-height;
                        position-visibility: anchors-visible;

                        &:popover-open {{
                            opacity: 1;
                            transform: scale(1) translateY(0px);

                            @starting-style {{
                                opacity: 0;
                                transform: scale(0.95) translateY(-4px);
                            }}
                        }}

                        & {{
                            transition:
                                display 0.2s allow-discrete,
                                overlay 0.2s allow-discrete,
                                transform 0.2s cubic-bezier(0.16, 1, 0.3, 1),
                                opacity 0.15s ease-out;
                            opacity: 0;
                            transform: scale(0.95) translateY(-4px);
                            transform-origin: {transform_origin};
                        }}
                    }}
                    ",
                )}
            </style>
            {children()}
            <script>
                {format!(
                    r#"
                    (function() {{
                        const setup = () => {{
                            const trigger = document.getElementById('{trigger_id}');
                            const content = document.getElementById('{content_id}');
                            if (!trigger || !content) {{ setTimeout(setup, 50); return; }}
                            if (trigger.dataset.hcInit) return;
                            trigger.dataset.hcInit = '1';
                            let t;
                            const show = () => {{ clearTimeout(t); t = setTimeout(() => {{ try {{ content.showPopover(); }} catch(e) {{}} }}, 150); }};
                            const hide = () => {{ clearTimeout(t); t = setTimeout(() => {{ try {{ content.hidePopover(); }} catch(e) {{}} }}, 150); }};
                            trigger.addEventListener('mouseenter', show);
                            trigger.addEventListener('mouseleave', hide);
                            trigger.addEventListener('focus', show);
                            trigger.addEventListener('blur', hide);
                            content.addEventListener('mouseenter', () => clearTimeout(t));
                            content.addEventListener('mouseleave', hide);
                        }};
                        if (document.readyState === 'loading') {{
                            document.addEventListener('DOMContentLoaded', setup);
                        }} else {{
                            setup();
                        }}
                    }})();
                    "#,
                )}
            </script>
        </leptos::context::Provider>
    }
}

#[component]
pub fn HoverCardTrigger(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<HoverCardContext>();

    view! {
        <span
            id=ctx.trigger_id
            class=tw_merge!("inline-block", class)
            style=format!("anchor-name: {}", ctx.anchor_name)
        >
            {children()}
        </span>
    }
}

#[component]
pub fn HoverCardContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let ctx = expect_context::<HoverCardContext>();
    let class = tw_merge!("overflow-visible relative z-50 p-4 rounded-lg border bg-card shadow-md w-64", class);

    view! {
        <div class=class id=ctx.content_id popover="manual">
            {children()}
        </div>
    }
}
```
