---
title: "Footer04"
name: "footer04"
cargo_dependencies: []
registry_dependencies: ["button", "footer", "input"]
type: "components:blocks"
path: "blocks/footer04.rs"
---

# Footer04

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add footer04
```

## Component Code

```rust
use icons::ChevronsUpDown;
use leptos::prelude::*;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::components::ui::button::{Button, ButtonSize};
use crate::components::ui::footer::{
    Footer, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterGrid, FooterLink,
    FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer, FooterTitle,
};
use crate::components::ui::input::Input;

#[component]
pub fn Footer04() -> impl IntoView {
    view! {
        <Footer class="pt-20">
            <FooterSection class="border-b">
                <FooterBrandLink attr:aria-label="go home" attr:href="/">
                    <BrandFooter />
                </FooterBrandLink>
                <FooterSocialContainer class="justify-center">
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
                </FooterSocialContainer>
            </FooterSection>

            <FooterContainer>
                <FooterGrid class="grid gap-12 md:grid-cols-5 md:gap-0 lg:grid-cols-4">
                    <FooterSectionsGrid class="grid-cols-2 sm:grid-cols-4 md:col-span-5 md:row-start-1 lg:col-span-3">
                        <FooterLinksSection>
                            <FooterTitle>Components</FooterTitle>
                            <FooterLink attr:href="/docs/components/accordion">Accordion</FooterLink>
                            <FooterLink attr:href="/docs/components/button">Button</FooterLink>
                            <FooterLink attr:href="/docs/components/card">Card</FooterLink>
                            <FooterLink attr:href="/docs/components/chips">Chips</FooterLink>
                            <FooterLink attr:href="/docs/components/dialog">Dialog</FooterLink>
                            <FooterLink attr:href="/docs/components/dropdown-menu">Dropdown</FooterLink>
                            <FooterLink attr:href="/docs/components/input">Input</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Hooks</FooterTitle>
                            <FooterLink attr:href="/docs/hooks/use-copy-clipboard">Use Copy Clipboard</FooterLink>
                            <FooterLink attr:href="/docs/hooks/use-lock-body-scroll">Use Lock Body Scroll</FooterLink>
                            <FooterLink attr:href="/docs/hooks/use-random">Use Random</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Blocks</FooterTitle>
                            <FooterLink attr:href="/blocks/login">Login</FooterLink>
                            <FooterLink attr:href="/blocks/sidenav">Sidenav</FooterLink>
                            <FooterLink attr:href="/blocks/headers">Headers</FooterLink>
                            <FooterLink attr:href="/blocks/footers">Footers</FooterLink>
                            <FooterLink attr:href="/blocks/faq">FAQ</FooterLink>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Legal</FooterTitle>
                            <FooterLink attr:href="#">Licence</FooterLink>
                            <FooterLink attr:href="#">Privacy</FooterLink>
                            <FooterLink attr:href="#">Cookies</FooterLink>
                            <FooterLink attr:href="#">Security</FooterLink>
                        </FooterLinksSection>
                    </FooterSectionsGrid>
                    <form class="row-start-1 pb-8 text-sm border-b md:col-span-2 md:border-none lg:col-span-1">
                        <div class="space-y-4">
                            <label
                                data-slot="label"
                                class="block text-sm font-medium leading-none select-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50"
                                for="mail"
                            >
                                Newsletter
                            </label>
                            <div class="flex gap-2">
                                <Input
                                    attr:r#type="email"
                                    attr:id="mail"
                                    attr:placeholder="Your email"
                                    attr:name="mail"
                                    class="h-8 text-sm"
                                />
                                <Button size=ButtonSize::Sm>Submit</Button>
                            </div>
                            <span class="block text-sm text-muted-foreground">"Don't miss any update!"</span>
                        </div>
                    </form>
                </FooterGrid>

                <FooterSection class="border-t">
                    <FooterCopyright class="block order-last text-center md:order-first">
                        Rustify, All rights reserved
                    </FooterCopyright>
                    <form action="">
                        <div class="relative">
                            <select
                                class="flex py-1 px-3 w-full h-9 text-base bg-transparent rounded-md border appearance-none outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none border-input file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground shadow-xs min-w-32 transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]"
                                name="language"
                            >
                                <option value="1">English</option>
                                <option value="2">Espanol</option>
                                <option value="3">Français</option>
                                <option value="4">Swahili</option>
                                <option value="5">Lingala</option>
                            </select>

                            <ChevronsUpDown class="absolute inset-y-0 right-2 my-auto opacity-75 pointer-events-none size-4" />
                        </div>
                    </form>
                </FooterSection>
            </FooterContainer>
        </Footer>
    }
}
```
