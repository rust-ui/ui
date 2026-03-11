---
title: "Mask"
name: "mask"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/mask.rs"
---

# Mask

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add mask
```

## Component Code

```rust
use leptos::prelude::*;
use tw_merge::*;

// TODO UI 🐛 : Fix MaskColor (-> demo)

#[component]
pub fn MaskWrapper(#[prop(into, optional)] class: String, children: Children) -> impl IntoView {
    let merged_class = tw_merge!(
        "flex justify-center items-center",
        "relative w-full h-full",
        "rounded-lg border",
        "overflow-hidden",
        "min-h-[300px]",
        class
    );

    view! { <div class=merged_class>{children()}</div> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[derive(TwClass, Clone, Copy)]
#[tw(class = "absolute inset-y-0 pointer-events-none from-white dark:from-background")]
pub struct MaskClass {
    pub side: MaskSide,
    // TODO. Fix MaskColor
}

#[derive(TwVariant)]
pub enum MaskSide {
    #[tw(default, class = "")]
    Default,
    #[tw(class = "left-0 w-1/3 bg-gradient-to-r")]
    Left,
    #[tw(class = "right-0 w-1/3 bg-gradient-to-l")]
    Right,
    #[tw(class = "top-0 w-full bg-gradient-to-b")]
    Top,
    #[tw(class = "bottom-0 w-full bg-gradient-to-t")]
    Bottom,
}

#[derive(TwVariant)]
pub enum MaskColor {
    #[tw(default, class = "from-pink-500 to-violet-500 dark:from-purple-700 dark:to-indigo-700")]
    Pink,
}

#[component]
pub fn Mask(
    #[prop(into, optional)] side: Signal<MaskSide>,
    // #[prop(into, optional)] color: Signal<MaskColor>,
    // TODO. └──> Not working properly, fix this later.
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let merged_class = Memo::new(move |_| {
        let side = side.get();

        let mask = MaskClass { side };
        mask.with_class(class.clone())
    });

    view! { <div class=merged_class /> }
}
```
