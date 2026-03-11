---
title: "Footer"
name: "footer"
cargo_dependencies: []
registry_dependencies: []
type: "components:ui"
path: "ui/footer.rs"
---

# Footer

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add footer
```

## Component Code

```rust
use leptos::prelude::*;
use leptos_ui::clx;

// TODO. max-w-5xl should be set only once by the Wrapper.

mod components {
    use super::*;
    clx! {Footer, footer, ""}
    clx! {FooterBrandLink, a, "block size-fit"}
    clx! {FooterLinksSection, div, "space-y-4 text-sm"}
    clx! {FooterTitle, span, "block font-medium"}
    clx! {FooterLink, a, "block duration-150 text-foreground/70 hover:text-primary"}
    clx! {FooterLinks, div, "flex flex-wrap gap-4 sm:flex-col"}
    clx! {FooterDescription, p, "text-sm text-foreground/70 text-balance"}
    clx! {FooterGrid, div, "grid gap-12 md:grid-cols-5"}
    clx! {FooterContainer, div, "px-6 mx-auto max-w-5xl"}
    clx! {FooterSection, section,
        "w-full max-w-5xl mx-auto py-6 flex flex-wrap gap-4 justify-between items-center",
        "[.border-b]:mb-14",
        "[.border-t]:mt-14"
    }
    clx! {FooterSocialContainer, div, "flex flex-wrap gap-6 text-sm"}
    clx! {FooterBrand, div, "md:col-span-2"}
    clx! {FooterSectionsGrid, div, "grid gap-6"}
    clx! {FooterCopyright, small, "text-sm text-foreground/70"}
    clx! {FooterNavContainer, div, "flex flex-wrap gap-6 justify-center my-8 text-sm"}

    // void! {FooterDivider, div, "h-px bg-repeat-x opacity-25 bg-[length:6px_1px] [background-image:linear-gradient(90deg,var(--color-foreground)_1px,transparent_1px)]"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn FooterExternalLink(
    children: Children,
    href: &'static str,
    #[prop(into, optional, default = "Social media link".to_string())] aria_label: String,
) -> impl IntoView {
    view! {
        <a
            target="_blank"
            rel="noopener noreferrer"
            class="block text-foreground/70 hover:text-primary"
            href=href
            aria-label=aria_label
        >
            {children()}
        </a>
    }
}
```
