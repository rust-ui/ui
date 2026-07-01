---
title: "Demo Bubble Link Button"
name: "demo_bubble_link_button"
cargo_dependencies: []
registry_dependencies: ["bubble"]
type: "components:demos"
path: "demos/demo_bubble_link_button.rs"
---

# Demo Bubble Link Button

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_bubble_link_button
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::bubble::{Bubble, BubbleAlign, BubbleContent, BubbleGroup, BubbleVariant};

#[component]
pub fn DemoBubbleLinkButton() -> impl IntoView {
    view! {
        <div class="flex flex-col gap-8 py-12 w-full max-w-sm">
            <Bubble variant=BubbleVariant::Muted>
                <BubbleContent>"How can I help you today?"</BubbleContent>
            </Bubble>
            <BubbleGroup>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    // TODO PORT: shadcn uses BubbleContent render={<button onClick={() => toast(...)} />}
                    // (asChild/render pattern). Ported as on_click with window.alert fallback.
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked forgot password").ok());
                    })>"I forgot my password"</BubbleContent>
                </Bubble>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked help with subscription").ok());
                    })>"I need help with my subscription"</BubbleContent>
                </Bubble>
                <Bubble variant=BubbleVariant::Tinted align=BubbleAlign::End>
                    <BubbleContent on_click=Callback::new(|_| {
                        let _ = web_sys::window()
                            .and_then(|w| w.alert_with_message("You clicked something else. Talk to a human.").ok());
                    })>"Something else. Talk to a human."</BubbleContent>
                </Bubble>
            </BubbleGroup>
        </div>
    }
}
```
