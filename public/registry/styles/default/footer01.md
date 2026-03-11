---
title: "Footer01"
name: "footer01"
cargo_dependencies: []
registry_dependencies: ["button", "card", "footer"]
type: "components:blocks"
path: "blocks/footer01.rs"
---

# Footer01

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add footer01
```

## Component Code

```rust
use leptos::prelude::*;

use super::footer_logos::{BrandFooter, LogoDiscord, LogoLinkedIn, LogoYouTube, SvgGridPattern};
use crate::components::ui::button::Button;
use crate::components::ui::card::{Card, CardContent, CardDescription, CardTitle};
use crate::components::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterTitle,
};

#[component]
pub fn Footer01() -> impl IntoView {
    view! {
        // * See below
        <CardSection />

        <Footer attr:role="contentinfo" class="pt-12 sm:pt-20 bg-accent">
            <FooterContainer class="space-y-16">
                <FooterGrid>
                    <FooterBrand class="space-y-4 md:space-y-5">
                        <FooterBrandLink attr:aria-label="go home" attr:href="/">
                            <BrandFooter />
                        </FooterBrandLink>
                        <FooterDescription>
                            Rust/UI is a registry of reusable components that you can copy/paste into your own app. Customize them as you want.
                        </FooterDescription>
                    </FooterBrand>
                    <FooterSectionsGrid class="col-span-3 sm:grid-cols-3">
                        <FooterLinksSection>
                            <FooterTitle>Components</FooterTitle>
                            <FooterLinks>
                                <FooterLink attr:href="/docs/components/accordion">Accordion</FooterLink>
                                <FooterLink attr:href="/docs/components/button">Button</FooterLink>
                                <FooterLink attr:href="/docs/components/card">Card</FooterLink>
                                <FooterLink attr:href="/docs/components/chips">Chips</FooterLink>
                            </FooterLinks>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Hooks</FooterTitle>
                            <FooterLinks>
                                <FooterLink attr:href="/docs/hooks/use-copy-clipboard">Use Copy Clipboard</FooterLink>
                                <FooterLink attr:href="/docs/hooks/use-lock-body-scroll">
                                    Use Lock Body Scroll
                                </FooterLink>
                                <FooterLink attr:href="/docs/hooks/use-random">Use Random</FooterLink>
                            </FooterLinks>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Community</FooterTitle>
                            <div class="flex flex-wrap gap-3 text-sm">
                                <FooterExternalLink href="https://www.youtube.com/@rustify-rs">
                                    <LogoYouTube />
                                </FooterExternalLink>
                                <FooterExternalLink href="#">
                                    <LogoDiscord />
                                </FooterExternalLink>
                                <FooterExternalLink href="#">
                                    <LogoLinkedIn />
                                </FooterExternalLink>
                            </div>
                        </FooterLinksSection>
                    </FooterSectionsGrid>
                </FooterGrid>

                <FooterSection class="border-t">
                    <FooterCopyright>
                        "Built by " <a class="font-semibold" href="https://rustify.rs/" target="_blank" rel="noopener">
                            Rustify
                        </a>"."
                    </FooterCopyright>
                    <FooterCopyright>Rustify, All rights reserved</FooterCopyright>
                </FooterSection>
            </FooterContainer>
        </Footer>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn CardSection() -> impl IntoView {
    view! {
        <section class="relative pt-12 md:pt-24 bg-linear-to-b from-background to-accent from-50% to-50%">
            <div class="px-6 mx-auto max-w-5xl">
                <Card class="overflow-hidden relative p-12 md:py-20 md:px-32">
                    <CardContent class="relative text-center">
                        <CardTitle class="text-3xl md:text-4xl text-balance">Build your App</CardTitle>
                        <CardDescription class="mt-4 mb-6 text-balance">
                            A registry of reusable Leptos components built with Tailwind CSS. Copy, paste, and customize for your Rust applications.
                        </CardDescription>
                        <Button href="/docs/components/installation">Start now</Button>
                    </CardContent>

                    <SvgGridPattern />
                </Card>
            </div>
        </section>
    }
}
```
