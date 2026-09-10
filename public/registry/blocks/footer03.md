


```rust
use dioxus::prelude::*;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::components::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterGrid, FooterLink,
    FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer, FooterTitle,
};

#[component]
pub fn Footer03() -> Element {
    rsx! {
        Footer { class: "pt-20",
            FooterContainer {
                FooterGrid {
                    FooterBrand {
                        FooterBrandLink { aria_label: "go home", href: "/",
                            BrandFooter {}
                        }
                    }
                    FooterSectionsGrid { class: "grid-cols-2 sm:grid-cols-4 md:col-span-3",
                        FooterLinksSection {
                            FooterTitle { "Components" }
                            FooterLink { href: "/docs/components/accordion", "Accordion" }
                            FooterLink { href: "/docs/components/button", "Button" }
                            FooterLink { href: "/docs/components/card", "Card" }
                            FooterLink { href: "/docs/components/chips", "Chips" }
                            FooterLink { href: "/docs/components/dialog", "Dialog" }
                            FooterLink { href: "/docs/components/dropdown-menu", "Dropdown" }
                            FooterLink { href: "/docs/components/input", "Input" }
                        }
                        FooterLinksSection {
                            FooterTitle { "Hooks" }
                            FooterLink { href: "/docs/hooks/use-copy-clipboard", "Use Copy Clipboard" }
                            FooterLink { href: "/docs/hooks/use-lock-body-scroll", "Use Lock Body Scroll" }
                            FooterLink { href: "/docs/hooks/use-random", "Use Random" }
                        }
                        FooterLinksSection {
                            FooterTitle { "Blocks" }
                            FooterLink { href: "/blocks/login", "Login" }
                            FooterLink { href: "/blocks/sidenav", "Sidenav" }
                            FooterLink { href: "/blocks/headers", "Headers" }
                            FooterLink { href: "/blocks/footers", "Footers" }
                            FooterLink { href: "/blocks/faq", "FAQ" }
                        }
                        FooterLinksSection {
                            FooterTitle { "Legal" }
                            FooterLink { href: "#", "Licence" }
                            FooterLink { href: "#", "Privacy" }
                            FooterLink { href: "#", "Cookies" }
                            FooterLink { href: "#", "Security" }
                        }
                    }
                }
                FooterSection { class: "border-t",
                    FooterCopyright { class: "block order-last text-center md:order-first",
                        "Rustify, All rights reserved"
                    }
                    FooterSocialContainer { class: "order-first justify-center md:order-last",
                        FooterExternalLink { href: "#", aria_label: "Twitter", LogoTwitter {} }
                        FooterExternalLink { href: "#", aria_label: "LinkedIn", LogoLinkedIn {} }
                        FooterExternalLink { href: "#", aria_label: "Facebook", LogoFacebook {} }
                        FooterExternalLink { href: "#", aria_label: "Threads", LogoThreads {} }
                        FooterExternalLink { href: "#", aria_label: "Instagram", LogoInstagram {} }
                        FooterExternalLink { href: "#", aria_label: "TikTok", LogoTikTok {} }
                    }
                }
            }
        }
    }
}
```