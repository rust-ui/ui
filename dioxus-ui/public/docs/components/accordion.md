+++
title = "Accordion"
description = "Rust/UI component that displays an Accordion."
tags = ["accordion"]
is_new = false
image = "/images/thumbnails/accordion.webp"
image_dark = "/images/thumbnails/accordion-dark.webp"
+++

<StaticAccordion />

## Installation

<StaticInstallAccordion />

## Usage

```rust
use crate::ui::accordion::{Accordion, AccordionItem, AccordionTrigger, AccordionContent};
```

```rust
let mut open = use_signal(|| false);

rsx! {
    Accordion {
        AccordionItem {
            AccordionTrigger {
                open: open,
                onclick: move |_| open.toggle(),
                "Is it accessible?"
            }
            AccordionContent {
                open: open,
                "Yes. It adheres to the WAI-ARIA design pattern."
            }
        }
    }
}
```

## Bordered

<StaticAccordionBordered />

## See Also

- [Collapsible](/components/collapsible)
- [Tabs](/components/tabs)
