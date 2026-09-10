use dioxus::prelude::*;

use super::footer_logos::{BrandFooter, LogoDiscord, LogoLinkedIn, LogoYouTube, SvgGridPattern};
use crate::ui::button::Button;
use crate::ui::card::{Card, CardContent, CardDescription, CardTitle};
use crate::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterTitle,
};

/*
 * title: Footer with CTA Card
 * iframe_height: 897px
 * container_class: w-full bg-background
*/

#[component]
pub fn Footer01() -> Element {
    rsx! {
        CardSection {}

        Footer { class: "pt-12 sm:pt-20 bg-accent",
            FooterContainer { class: "space-y-16",
                FooterGrid {
                    FooterBrand { class: "space-y-4 md:space-y-5",
                        FooterBrandLink { aria_label: "go home", href: "/",
                            BrandFooter {}
                        }
                        FooterDescription {
                            "Rust/UI is a registry of reusable components that you can copy/paste into your own app. Customize them as you want."
                        }
                    }

                    FooterSectionsGrid { class: "col-span-3 sm:grid-cols-3",
                        FooterLinksSection {
                            FooterTitle { "Components" }
                            FooterLinks {
                                FooterLink { href: "/docs/components/accordion", "Accordion" }
                                FooterLink { href: "/docs/components/button", "Button" }
                                FooterLink { href: "/docs/components/card", "Card" }
                                FooterLink { href: "/docs/components/chips", "Chips" }
                            }
                        }
                        FooterLinksSection {
                            FooterTitle { "Hooks" }
                            FooterLinks {
                                FooterLink { href: "/docs/hooks/use-copy-clipboard", "Use Copy Clipboard" }
                                FooterLink { href: "/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll" }
                                FooterLink { href: "/docs/hooks/use-random", "Use Random" }
                            }
                        }
                        FooterLinksSection {
                            FooterTitle { "Community" }
                            div { class: "flex flex-wrap gap-3 text-sm",
                                FooterExternalLink { href: "https://www.youtube.com/@rustify-rs", LogoYouTube {} }
                                FooterExternalLink { href: "#", LogoDiscord {} }
                                FooterExternalLink { href: "#", LogoLinkedIn {} }
                            }
                        }
                    }
                }

                FooterSection { class: "border-t",
                    FooterCopyright {
                        "Built by "
                        a {
                            class: "font-semibold",
                            href: "https://rustify.rs/",
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
                        Button { href: "/docs/installation", "Start now" }
                    }

                    SvgGridPattern {}
                }
            }
        }
    }
}
