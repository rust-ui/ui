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




## Components

The Accordion component is composed of several subcomponents:

- **Accordion**: Main wrapper component managing expand/collapse state
- **AccordionItem**: Individual collapsible section container
- **AccordionHeader**: Clickable header area that toggles content
- **AccordionTitle**: Title text displayed in the header
- **AccordionContent**: Collapsible content area for each item



## Usage

```rust
use crate::components::ui::accordion::{
    Accordion,
    AccordionContent,
    AccordionHeader,
    AccordionItem,
    AccordionTitle,
};
```

```rust
<Accordion class="max-w-[400px]">
    <AccordionItem>
        <AccordionHeader>
            <AccordionTitle>"Accordion Item"</AccordionTitle>
        </AccordionHeader>
        <AccordionContent>
            "Accordion content goes here"
        </AccordionContent>
    </AccordionItem>
</Accordion>
```




## Examples

### Accordion Bordered

Accordion component with bordered styling for clear visual separation between items. This example demonstrates how to implement bordered accordions in Leptos with proper spacing and border styling for improved readability.

<StaticAccordionBordered />

### Accordion with Icons

Accordion component enhanced with custom icons for expanded and collapsed states. This example shows how to integrate Lucide icons with accordion headers to provide better visual feedback and improve user experience in Rust applications.

<StaticAccordionIcons />

### Accordion in Card (FAQ)

Accordion nested inside a Card for FAQ and settings sections. This pattern is useful for billing pages, help centers, or any grouped Q&A content where a card provides visual containment around the collapsible items.

<StaticAccordionCard />



### RTL

FAQ-style accordion with Arabic content. Trigger chevrons and content indentation adapt automatically to the RTL reading direction.

<StaticAccordionRtl />

## See Also

- [Tabs](/docs/components/tabs)
- [Card](/docs/components/card)
