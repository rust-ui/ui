use dioxus::prelude::*;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::ui::footer::{
    Footer, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterLink, FooterNavContainer,
};

/*
 * title: Footer Centered Simple
 * iframe_height: 448px
 * container_class: bg-muted
*/

#[component]
pub fn Footer02() -> Element {
    rsx! {
        Footer { class: "py-16 md:py-32",
            FooterContainer {
                FooterBrandLink { class: "mx-auto", aria_label: "go home", href: "/",
                    BrandFooter {}
                }
                FooterNavContainer {
                    FooterLink { href: "/docs/components/accordion", "Accordion" }
                    FooterLink { href: "/docs/components/button", "Button" }
                    FooterLink { href: "/docs/components/card", "Card" }
                    FooterLink { href: "/docs/components/chips", "Chips" }
                    FooterLink { href: "/blocks/login", "Login" }
                    FooterLink { href: "/blocks/sidenav", "Sidenav" }
                }
                FooterNavContainer {
                    FooterExternalLink { href: "#", aria_label: "Twitter", LogoTwitter {} }
                    FooterExternalLink { href: "#", aria_label: "LinkedIn", LogoLinkedIn {} }
                    FooterExternalLink { href: "#", aria_label: "Facebook", LogoFacebook {} }
                    FooterExternalLink { href: "#", aria_label: "Threads", LogoThreads {} }
                    FooterExternalLink { href: "#", aria_label: "Instagram", LogoInstagram {} }
                    FooterExternalLink { href: "#", aria_label: "TikTok", LogoTikTok {} }
                }
                FooterCopyright { class: "block text-center", "Rustify, All rights reserved" }
            }
        }
    }
}
