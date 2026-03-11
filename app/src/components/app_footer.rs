use leptos::prelude::*;
use registry::blocks::footer_logos::{BrandFooter, LogoDiscord, LogoLinkedIn, LogoYouTube, SvgGridPattern};
use registry::ui::button::Button;
use registry::ui::card::{Card, CardContent, CardDescription, CardTitle};
use registry::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterTitle,
};

/* ========================================================== */
/*                     ✨ ROUTES ✨                          */
/*========================================================== */

// Components
const ROUTE_ACCORDION: &str = "/docs/components/accordion";
const ROUTE_BUTTON: &str = "/docs/components/button";
const ROUTE_CARD: &str = "/docs/components/card";
const ROUTE_FORM: &str = "/docs/components/form";
const ROUTE_INSTALLATION: &str = "/docs/components/installation";

// Hooks
const ROUTE_USE_COPY_CLIPBOARD: &str = "/docs/hooks/use-copy-clipboard";
const ROUTE_USE_LOCK_BODY_SCROLL: &str = "/docs/hooks/use-lock-body-scroll";
const ROUTE_USE_HORIZONTAL_SCROLL: &str = "/docs/hooks/use-horizontal-scroll";

// External
const ROUTE_HOME: &str = "/";
const URL_RUSTIFY: &str = "https://rustify.rs/";
const URL_YOUTUBE: &str = "https://www.youtube.com/@rustify-rs";
const URL_LINKEDIN_SHARE: &str = "https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Frust-ui.com";

#[component]
pub fn AppFooter() -> impl IntoView {
    view! {
        <CardSection />

        <Footer attr:role="contentinfo" class="pt-12 sm:pt-20 bg-accent">
            <FooterContainer class="space-y-16">
                <FooterGrid>
                    <FooterBrand class="space-y-4 md:space-y-5">
                        <FooterBrandLink attr:aria-label="go home" attr:href=ROUTE_HOME>
                            <BrandFooter />
                        </FooterBrandLink>
                        <FooterDescription>
                            Rust/UI is a registry of reusable components that you can copy/paste into your own app. Customize them as you want.
                        </FooterDescription>
                        <p class="mt-2 text-sm text-muted-foreground">"Last updated: " {env!("BUILD_DATE")}</p>
                    </FooterBrand>
                    <FooterSectionsGrid class="col-span-3 sm:grid-cols-3">
                        <FooterLinksSection>
                            <FooterTitle>Components</FooterTitle>
                            <FooterLinks>
                                <FooterLink attr:href=ROUTE_ACCORDION>Accordion</FooterLink>
                                <FooterLink attr:href=ROUTE_BUTTON>Button</FooterLink>
                                <FooterLink attr:href=ROUTE_CARD>Card</FooterLink>
                                <FooterLink attr:href=ROUTE_FORM>Form</FooterLink>
                            </FooterLinks>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Hooks</FooterTitle>
                            <FooterLinks>
                                <FooterLink attr:href=ROUTE_USE_COPY_CLIPBOARD>Use Copy Clipboard</FooterLink>
                                <FooterLink attr:href=ROUTE_USE_LOCK_BODY_SCROLL>Use Lock Body Scroll</FooterLink>
                                <FooterLink attr:href=ROUTE_USE_HORIZONTAL_SCROLL>Use Horizontal Scroll</FooterLink>
                            </FooterLinks>
                        </FooterLinksSection>
                        <FooterLinksSection>
                            <FooterTitle>Community</FooterTitle>
                            <div class="flex flex-wrap gap-3 text-sm">
                                <FooterExternalLink href=URL_YOUTUBE>
                                    <LogoYouTube />
                                </FooterExternalLink>
                                <FooterExternalLink href="#">
                                    <LogoDiscord />
                                </FooterExternalLink>
                                <FooterExternalLink href=URL_LINKEDIN_SHARE>
                                    <LogoLinkedIn />
                                </FooterExternalLink>
                            </div>
                        </FooterLinksSection>
                    </FooterSectionsGrid>
                </FooterGrid>

                <FooterSection class="border-t">
                    <FooterCopyright>
                        "Built by " <a class="font-semibold" href=URL_RUSTIFY target="_blank" rel="noopener">
                            Rustify
                        </a>"."
                    </FooterCopyright>
                    <FooterCopyright>Rustify, All rights reserved</FooterCopyright>
                </FooterSection>
            </FooterContainer>
        </Footer>
    }
}

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
                        <Button href=ROUTE_INSTALLATION>Start now</Button>
                    </CardContent>

                    <SvgGridPattern />
                </Card>
            </div>
        </section>
    }
}
