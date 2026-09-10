use dioxus::prelude::*;
use icons::ChevronsUpDown;

use super::footer_logos::{
    BrandFooter, LogoFacebook, LogoInstagram, LogoLinkedIn, LogoThreads, LogoTikTok, LogoTwitter,
};
use crate::ui::button::{Button, ButtonSize};
use crate::ui::footer::{
    Footer, FooterBrandLink, FooterContainer, FooterCopyright, FooterExternalLink, FooterGrid, FooterLink,
    FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer, FooterTitle,
};
use crate::ui::input::{Input, InputType};

/*
 * title: Footer with Newsletter and Language Selector
 * iframe_height: 606px
 * container_class: w-full bg-background
*/

#[component]
pub fn Footer04() -> Element {
    rsx! {
        Footer { class: "pt-20",
            FooterSection { class: "border-b",
                FooterBrandLink { aria_label: "go home", href: "/",
                    BrandFooter {}
                }
                FooterSocialContainer { class: "justify-center",
                    FooterExternalLink { href: "#", aria_label: "Twitter", LogoTwitter {} }
                    FooterExternalLink { href: "#", aria_label: "LinkedIn", LogoLinkedIn {} }
                    FooterExternalLink { href: "#", aria_label: "Facebook", LogoFacebook {} }
                    FooterExternalLink { href: "#", aria_label: "Threads", LogoThreads {} }
                    FooterExternalLink { href: "#", aria_label: "Instagram", LogoInstagram {} }
                    FooterExternalLink { href: "#", aria_label: "TikTok", LogoTikTok {} }
                }
            }

            FooterContainer {
                FooterGrid { class: "grid gap-12 md:grid-cols-5 md:gap-0 lg:grid-cols-4",
                    FooterSectionsGrid { class: "grid-cols-2 sm:grid-cols-4 md:col-span-5 md:row-start-1 lg:col-span-3",
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

                    form { class: "row-start-1 pb-8 text-sm border-b md:col-span-2 md:border-none lg:col-span-1",
                        div { class: "space-y-4",
                            label {
                                "data-slot": "label",
                                class: "block text-sm font-medium leading-none select-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
                                r#for: "mail",
                                "Newsletter"
                            }
                            div { class: "flex gap-2",
                                Input {
                                    r#type: InputType::Email,
                                    id: "mail",
                                    placeholder: "Your email",
                                    name: "mail",
                                    class: "h-8 text-sm",
                                }
                                Button { size: ButtonSize::Sm, "Submit" }
                            }
                            span { class: "block text-sm text-muted-foreground", "Don't miss any update!" }
                        }
                    }
                }

                FooterSection { class: "border-t",
                    FooterCopyright { class: "block order-last text-center md:order-first",
                        "Rustify, All rights reserved"
                    }
                    form {
                        div { class: "relative",
                            select {
                                class: "flex py-1 px-3 w-full h-9 text-base bg-transparent rounded-md border appearance-none outline-none md:text-sm disabled:opacity-50 disabled:cursor-not-allowed disabled:pointer-events-none border-input file:text-foreground placeholder:text-muted-foreground selection:bg-primary selection:text-primary-foreground shadow-xs min-w-32 transition-[color,box-shadow] file:inline-flex file:h-7 file:border-0 file:bg-transparent file:text-sm file:font-medium aria-invalid:ring-destructive/20 aria-invalid:border-destructive dark:aria-invalid:ring-destructive/40 focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px]",
                                name: "language",
                                option { value: "1", "English" }
                                option { value: "2", "Espanol" }
                                option { value: "3", "Francais" }
                                option { value: "4", "Swahili" }
                                option { value: "5", "Lingala" }
                            }
                            ChevronsUpDown { class: "absolute inset-y-0 right-2 my-auto opacity-75 pointer-events-none size-4" }
                        }
                    }
                }
            }
        }
    }
}
