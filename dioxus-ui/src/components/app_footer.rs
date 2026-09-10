use dioxus::prelude::*;
use registry::blocks::footer_logos::{BrandFooter, LogoDiscord, LogoLinkedIn, LogoYouTube, SvgGridPattern};
use registry::ui::button::Button;
use registry::ui::card::{Card, CardContent, CardDescription, CardTitle};
use registry::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterTitle,
};

const ROUTE_ACCORDION: &str = "/docs/components/accordion";
const ROUTE_BUTTON: &str = "/docs/components/button";
const ROUTE_CARD: &str = "/docs/components/card";
const ROUTE_INSTALLATION: &str = "/docs/components/installation";

const ROUTE_USE_COPY_CLIPBOARD: &str = "/docs/hooks/use-copy-clipboard";
const ROUTE_USE_LOCK_BODY_SCROLL: &str = "/docs/hooks/use-lock-body-scroll";
const ROUTE_USE_HORIZONTAL_SCROLL: &str = "/docs/hooks/use-horizontal-scroll";

const ROUTE_HOME: &str = "/";
const URL_RUSTIFY: &str = "https://rustify.rs/";
const URL_YOUTUBE: &str = "https://www.youtube.com/@rustify-rs";
const URL_LINKEDIN_SHARE: &str = "https://www.linkedin.com/sharing/share-offsite/?url=https%3A%2F%2Fdioxus-ui.com";

#[component]
pub fn AppFooter() -> Element {
    rsx! {
        CardSection {}

        Footer { class: "pt-12 sm:pt-20 bg-accent",
            FooterContainer { class: "space-y-16",
                FooterGrid {
                    FooterBrand { class: "space-y-4 md:space-y-5",
                        FooterBrandLink { href: ROUTE_HOME,
                            BrandFooter {}
                        }
                        FooterDescription {
                            "Rust/UI is a registry of reusable components that you can copy/paste into your own app. Customize them as you want."
                        }
                        p { class: "mt-2 text-sm text-muted-foreground",
                            "Last updated: "
                            {env!("BUILD_DATE")}
                        }
                    }
                    FooterSectionsGrid { class: "col-span-3 sm:grid-cols-3",
                        FooterLinksSection {
                            FooterTitle { "Components" }
                            FooterLinks {
                                FooterLink { href: ROUTE_ACCORDION, "Accordion" }
                                FooterLink { href: ROUTE_BUTTON, "Button" }
                                FooterLink { href: ROUTE_CARD, "Card" }
                            }
                        }
                        FooterLinksSection {
                            FooterTitle { "Hooks" }
                            FooterLinks {
                                FooterLink { href: ROUTE_USE_COPY_CLIPBOARD, "Use Copy Clipboard" }
                                FooterLink { href: ROUTE_USE_LOCK_BODY_SCROLL, "Use Lock Body Scroll" }
                                FooterLink { href: ROUTE_USE_HORIZONTAL_SCROLL, "Use Horizontal Scroll" }
                            }
                        }
                        FooterLinksSection {
                            FooterTitle { "Community" }
                            div { class: "flex flex-wrap gap-3 text-sm",
                                FooterExternalLink { href: URL_YOUTUBE,
                                    LogoYouTube {}
                                }
                                FooterExternalLink { href: "#",
                                    LogoDiscord {}
                                }
                                FooterExternalLink { href: URL_LINKEDIN_SHARE,
                                    LogoLinkedIn {}
                                }
                            }
                        }
                    }
                }

                FooterSection { class: "border-t",
                    FooterCopyright {
                        "Built by "
                        a {
                            class: "font-semibold",
                            href: URL_RUSTIFY,
                            target: "_blank",
                            rel: "noopener",
                            "Rustify"
                        }
                        "."
                    }
                    FooterCopyright { "Rustify, All rights reserved" }
                }
            }
        }
    }
}

#[component]
pub fn CardSection() -> Element {
    rsx! {
        section { class: "relative pt-12 md:pt-24 bg-linear-to-b from-background to-accent from-50% to-50%",
            div { class: "px-6 mx-auto max-w-5xl",
                Card { class: "overflow-hidden relative p-12 md:py-20 md:px-32",
                    CardContent { class: "relative text-center",
                        CardTitle { class: "text-3xl md:text-4xl text-balance", "Build your App" }
                        CardDescription { class: "mt-4 mb-6 text-balance",
                            "A registry of reusable Dioxus components built with Tailwind CSS. Copy, paste, and customize for your Rust applications."
                        }
                        Button { href: ROUTE_INSTALLATION, "Start now" }
                    }
                    SvgGridPattern {}
                }
            }
        }
    }
}
