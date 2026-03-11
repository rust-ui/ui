---
title: "Demo Drawer Scrollable"
name: "demo_drawer_scrollable"
cargo_dependencies: []
registry_dependencies: ["drawer"]
type: "components:demos"
path: "demos/demo_drawer_scrollable.rs"
---

# Demo Drawer Scrollable

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_drawer_scrollable
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerScrollable() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />

                <DrawerBody class="overflow-y-auto max-h-[300px]">
                    <DrawerHeader>
                        <DrawerTitle>"Ira Glass on Taste"</DrawerTitle>
                    </DrawerHeader>
                    <DrawerDescription>
                        "Nobody tells this to people who are beginners, I wish someone told me. All of us who do creative work, we get into it because we have good taste."
                    </DrawerDescription>
                    <DrawerDescription>
                        "But there is this gap. For the first couple years you make stuff, it's just not that good. It's trying to be good, it has potential, but it's not. But your taste, the thing that got you into the game, is still killer. And your taste is why your work disappoints you. A lot of people never get past this phase, they quit."
                    </DrawerDescription>
                    <DrawerDescription>
                        "Most people I know who do interesting, creative work went through years of this. We know our work doesn't have this special thing that we want it to have. We all go through this. And if you are just starting out or you are still in this phase, you gotta know its normal and the most important thing you can do is do a lot of work."
                    </DrawerDescription>
                    <DrawerDescription>
                        "Put yourself on a deadline so that every week you will finish one story. It is only by going through a volume of work that you will close that gap, and your work will be as good as your ambitions. And I took longer to figure out how to do this than anyone I've ever met. It's gonna take awhile. It's normal to take awhile. You've just gotta fight your way through."
                    </DrawerDescription>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}
```
