


```rust
use leptos::prelude::*;

use super::footer_logos::{BrandFooter, LogoLinkedIn, LogoTwitter};
use crate::components::ui::button::{Button, ButtonSize};
use crate::components::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer,
    FooterTitle,
};
use crate::components::ui::input::Input;

#[component]
pub fn Footer05() -> impl IntoView {
    view! {
        <Footer attr:role="contentinfo" class="pt-12">
            <FooterContainer class="space-y-16">
                <FooterSection class="border-b">
                    <FooterBrand class="space-y-6 max-w-xs">
                        <FooterBrandLink attr:aria-label="go home" attr:href="/">
                            <BrandFooter />
                        </FooterBrandLink>
                        <FooterDescription>Paris, 75001 - France</FooterDescription>
                    </FooterBrand>
                    <FooterSocialContainer class="gap-3">
                        <FooterExternalLink href="#" attr:aria-label="Twitter">
                            <LogoTwitter />
                        </FooterExternalLink>
                        <FooterExternalLink href="#" attr:aria-label="LinkedIn">
                            <LogoLinkedIn />
                        </FooterExternalLink>
                    </FooterSocialContainer>
                </FooterSection>
                <FooterGrid>
                    <FooterSectionsGrid class="sm:grid-cols-3 md:col-span-3">
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
                            <FooterTitle>Blocks</FooterTitle>
                            <FooterLinks>
                                <FooterLink attr:href="/blocks/login">Login</FooterLink>
                                <FooterLink attr:href="/blocks/sidenav">Sidenav</FooterLink>
                                <FooterLink attr:href="/blocks/faq">FAQ</FooterLink>
                            </FooterLinks>
                        </FooterLinksSection>
                    </FooterSectionsGrid>
                    <div class="md:col-span-2">
                        <form class="ml-auto space-y-4 w-full md:max-w-xs">
                            <label
                                data-slot="label"
                                class="block text-sm font-medium select-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50"
                                for="email"
                            >
                                Subscribe to our newsletter
                            </label>
                            <div class="flex gap-2">
                                <Input
                                    attr:r#type="email"
                                    attr:placeholder="Your email"
                                    attr:id="email"
                                    attr:name="email"
                                    attr:required
                                    class="h-8"
                                />
                                <Button attr:r#type="submit" size=ButtonSize::Sm>
                                    Subscribe
                                </Button>
                            </div>
                            <p class="text-xs text-muted-foreground">
                                Get the latest product news and behind the scenes updates. Unsubscribe at any time.
                            </p>
                        </form>
                    </div>
                </FooterGrid>

                <FooterSection class="items-start border-t">
                    <FooterCopyright>Rustify, All rights reserved</FooterCopyright>
                    <div class="flex gap-2 items-center py-1 pr-4 pl-2 rounded-full border border-transparent ring-1 shadow ring-foreground/5 bg-card">
                        <div class="flex relative size-3">
                            <span class="block absolute inset-0 bg-emerald-100 rounded-full animate-pulse duration-1500 size-full"></span>
                            <span class="block relative m-auto bg-emerald-500 rounded-full size-1"></span>
                        </div>
                        <span class="text-sm">All Systems Normal</span>
                    </div>
                </FooterSection>
            </FooterContainer>
        </Footer>
    }
}
```