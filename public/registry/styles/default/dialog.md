---
title: "Dialog"
name: "dialog"
cargo_dependencies: []
registry_dependencies: ["button"]
type: "components:ui"
path: "ui/dialog.rs"
description: "Rust/UI component that displays a modal dialog that the user can interact with."
tags: ["dialog"]
---

# Dialog

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add dialog
```

## Component Code

```rust
use icons::X;
use leptos::context::Provider;
use leptos::prelude::*;
use leptos_ui::clx;
use tw_merge::*;

use crate::components::hooks::use_random::use_random_id_for;
use crate::components::ui::button::{Button, ButtonSize, ButtonVariant};

mod components {
    use super::*;
    clx! {DialogBody, div, "flex flex-col gap-4"}
    clx! {DialogHeader, div, "flex flex-col gap-2 text-center sm:text-left"}
    clx! {DialogTitle, h3, "text-lg leading-none font-semibold"}
    clx! {DialogDescription, p, "text-muted-foreground text-sm"}
    clx! {DialogFooter, footer, "flex flex-col-reverse gap-2 sm:flex-row sm:justify-end"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(Clone)]
struct DialogContext {
    target_id: String,
}

#[component]
pub fn Dialog(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    let dialog_target_id = use_random_id_for("dialog");

    let ctx = DialogContext { target_id: dialog_target_id };

    let merged_class = tw_merge!("w-fit", class);

    view! {
        <Provider value=ctx>
            <div class=merged_class data-name="__Dialog">
                {children()}
            </div>
        </Provider>
    }
}

#[component]
pub fn DialogTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let trigger_id = format!("trigger_{}", ctx.target_id);

    view! {
        <Button
            class=class
            attr:id=trigger_id
            attr:tabindex="0"
            attr:data-dialog-trigger=ctx.target_id
            variant=variant
            size=size
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn DialogContent(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = true)] show_close_button: bool,
    #[prop(default = true)] close_on_backdrop_click: bool,
    #[prop(default = "Dialog")] data_name_prefix: &'static str,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();
    let merged_class = tw_merge!(
        // "flex flex-col gap-4", // TODO 🐛 Bug when I try to have this.. Using DialogBody instead.
        "relative bg-background border rounded-2xl shadow-lg p-6 w-full max-w-[calc(100%-2rem)] max-h-[85vh] fixed top-[50%] left-[50%] translate-x-[-50%] translate-y-[-50%] z-100 transition-all duration-200 data-[state=closed]:opacity-0 data-[state=closed]:scale-95 data-[state=open]:opacity-100 data-[state=open]:scale-100",
        class
    );

    let backdrop_data_name = format!("{}Backdrop", data_name_prefix);
    let content_data_name = format!("{}Content", data_name_prefix);

    let target_id_clone = ctx.target_id.clone();
    let backdrop_id = format!("{}_backdrop", ctx.target_id);
    let target_id_for_script = ctx.target_id.clone();
    let backdrop_id_for_script = backdrop_id.clone();
    let backdrop_behavior = if close_on_backdrop_click { "auto" } else { "manual" };

    view! {
        <script src="/hooks/lock_scroll.js"></script>

        <div
            data-name=backdrop_data_name
            id=backdrop_id
            class="fixed inset-0 transition-opacity duration-200 pointer-events-none z-60 bg-black/50 data-[state=closed]:opacity-0 data-[state=open]:opacity-100"
            data-state="closed"
        />

        <div
            data-name=content_data_name
            class=merged_class
            id=ctx.target_id
            data-target="target__dialog"
            data-state="closed"
            data-backdrop=backdrop_behavior
            style="pointer-events: none;"
        >
            <button
                type="button"
                class=format!(
                    "absolute top-4 right-4 p-1 rounded-sm focus:ring-2 focus:ring-offset-2 focus:outline-none [&_svg:not([class*='size-'])]:size-4 focus:ring-ring{}",
                    if show_close_button { "" } else { " hidden" },
                )
                data-dialog-close=target_id_clone
                aria-label="Close dialog"
            >
                <span class="hidden">"Close Dialog"</span>
                <X />
            </button>

            {children()}
        </div>

        <script>
            {format!(
                r#"
                (function() {{
                    const setupDialog = () => {{
                        const dialog = document.querySelector('#{}');
                        const backdrop = document.querySelector('#{}');
                        const trigger = document.querySelector('[data-dialog-trigger="{}"]');

                        if (!dialog || !backdrop || !trigger) {{
                            setTimeout(setupDialog, 50);
                            return;
                        }}

                        if (dialog.hasAttribute('data-initialized')) {{
                            return;
                        }}
                        dialog.setAttribute('data-initialized', 'true');

                        const openDialog = () => {{
                            // Lock scrolling
                            window.ScrollLock.lock();

                            dialog.setAttribute('data-state', 'open');
                            backdrop.setAttribute('data-state', 'open');
                            dialog.style.pointerEvents = 'auto';
                            backdrop.style.pointerEvents = 'auto';
                        }};

                        const closeDialog = () => {{
                            dialog.setAttribute('data-state', 'closed');
                            backdrop.setAttribute('data-state', 'closed');
                            dialog.style.pointerEvents = 'none';
                            backdrop.style.pointerEvents = 'none';

                            // Unlock scrolling after animation
                            window.ScrollLock.unlock(200);
                        }};

                        // Open dialog when trigger is clicked
                        trigger.addEventListener('click', openDialog);

                        // Close buttons
                        const closeButtons = dialog.querySelectorAll('[data-dialog-close]');
                        closeButtons.forEach(btn => {{
                            btn.addEventListener('click', closeDialog);
                        }});

                        // Close on backdrop click (if data-backdrop="auto")
                        backdrop.addEventListener('click', () => {{
                            if (dialog.getAttribute('data-backdrop') === 'auto') {{
                                closeDialog();
                            }}
                        }});

                        // Handle ESC key to close
                        document.addEventListener('keydown', (e) => {{
                            if (e.key === 'Escape' && dialog.getAttribute('data-state') === 'open') {{
                                e.preventDefault();
                                closeDialog();
                            }}
                        }});
                    }};

                    if (document.readyState === 'loading') {{
                        document.addEventListener('DOMContentLoaded', setupDialog);
                    }} else {{
                        setupDialog();
                    }}
                }})();
                "#,
                target_id_for_script,
                backdrop_id_for_script,
                target_id_for_script,
            )}
        </script>
    }
}

#[component]
pub fn DialogClose(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    view! {
        <Button
            class=class
            attr:data-dialog-close=ctx.target_id
            attr:aria-label="Close dialog"
            variant=variant
            size=size
        >
            {children()}
        </Button>
    }
}

#[component]
pub fn DialogAction(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Default)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    let ctx = expect_context::<DialogContext>();

    view! {
        <Button
            class=class
            attr:data-dialog-close=ctx.target_id
            attr:aria-label="Close dialog"
            variant=variant
            size=size
        >
            {children()}
        </Button>
    }
}
```
