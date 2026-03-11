---
title: "Footer02"
name: "footer02"
cargo_dependencies: []
registry_dependencies: ["footer"]
type: "components:blocks"
path: "blocks/footer02.rs"
---

# Footer02

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add footer02
```

## Component Code

```rust
use leptos::prelude::*;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::components::ui::footer::{
    Footer, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterLink, FooterNavContainer,
};

#[component]
pub fn Footer02() -> impl IntoView {
    view! {
        <Footer class="py-16 md:py-32">
            <FooterContainer>
                <FooterBrandLink class="mx-auto" attr:aria-label="go home" attr:href="/">
                    <BrandFooter />
                </FooterBrandLink>
                <FooterNavContainer>
                    <FooterLink attr:href="/docs/components/accordion">Accordion</FooterLink>
                    <FooterLink attr:href="/docs/components/button">Button</FooterLink>
                    <FooterLink attr:href="/docs/components/card">Card</FooterLink>
                    <FooterLink attr:href="/docs/components/chips">Chips</FooterLink>
                    <FooterLink attr:href="/blocks/login">Login</FooterLink>
                    <FooterLink attr:href="/blocks/sidenav">Sidenav</FooterLink>
                </FooterNavContainer>
                <FooterNavContainer>
                    <FooterExternalLink href="#" attr:aria-label="Twitter">
                        <LogoTwitter />
                    </FooterExternalLink>
                    <FooterExternalLink href="#" attr:aria-label="LinkedIn">
                        <LogoLinkedIn />
                    </FooterExternalLink>
                    <FooterExternalLink href="#" attr:aria-label="Facebook">
                        <LogoFacebook />
                    </FooterExternalLink>
                    <FooterExternalLink href="#" attr:aria-label="Threads">
                        <LogoThreads />
                    </FooterExternalLink>
                    <FooterExternalLink href="#" attr:aria-label="Instagram">
                        <LogoInstagram />
                    </FooterExternalLink>
                    <FooterExternalLink href="#" attr:aria-label="TikTok">
                        <LogoTikTok />
                    </FooterExternalLink>
                </FooterNavContainer>
                <FooterCopyright class="block text-center">Rustify, All rights reserved</FooterCopyright>
            </FooterContainer>
        </Footer>
    }
}
```
