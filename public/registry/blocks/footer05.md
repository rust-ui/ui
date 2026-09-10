


```rust
use dioxus::prelude::*;

use super::footer_logos::{BrandFooter, LogoLinkedIn, LogoTwitter};
use crate::components::ui::button::{Button, ButtonSize};
use crate::components::ui::footer::{
    Footer, FooterBrand, FooterBrandLink, FooterContainer, FooterCopyright, FooterDescription, FooterExternalLink,
    FooterGrid, FooterLink, FooterLinks, FooterLinksSection, FooterSection, FooterSectionsGrid, FooterSocialContainer,
    FooterTitle,
};
use crate::components::ui::input::{Input, InputType};

#[component]
pub fn Footer05() -> Element {
    rsx! {
        Footer { class: "pt-12",
            FooterContainer { class: "space-y-16",
                FooterSection { class: "border-b",
                    FooterBrand { class: "space-y-6 max-w-xs",
                        FooterBrandLink { aria_label: "go home", href: "/",
                            BrandFooter {}
                        }
                        FooterDescription { "Paris, 75001 - France" }
                    }
                    FooterSocialContainer { class: "gap-3",
                        FooterExternalLink { href: "#", aria_label: "Twitter",
                            LogoTwitter {}
                        }
                        FooterExternalLink { href: "#", aria_label: "LinkedIn",
                            LogoLinkedIn {}
                        }
                    }
                }

                FooterGrid {
                    FooterSectionsGrid { class: "sm:grid-cols-3 md:col-span-3",
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
                            FooterTitle { "Blocks" }
                            FooterLinks {
                                FooterLink { href: "/blocks/login", "Login" }
                                FooterLink { href: "/blocks/sidenav", "Sidenav" }
                                FooterLink { href: "/blocks/faq", "FAQ" }
                            }
                        }
                    }

                    div { class: "md:col-span-2",
                        form { class: "ml-auto space-y-4 w-full md:max-w-xs",
                            label {
                                "data-slot": "label",
                                class: "block text-sm font-medium select-none peer-disabled:cursor-not-allowed peer-disabled:opacity-50 group-data-[disabled=true]:pointer-events-none group-data-[disabled=true]:opacity-50",
                                r#for: "email",
                                "Subscribe to our newsletter"
                            }
                            div { class: "flex gap-2",
                                Input {
                                    r#type: InputType::Email,
                                    placeholder: "Your email",
                                    id: "email",
                                    name: "email",
                                    required: true,
                                    class: "h-8",
                                }
                                Button { button_type: "submit", size: ButtonSize::Sm, "Subscribe" }
                            }
                            p { class: "text-xs text-muted-foreground",
                                "Get the latest product news and behind the scenes updates. Unsubscribe at any time."
                            }
                        }
                    }
                }

                FooterSection { class: "items-start border-t",
                    FooterCopyright { "Rustify, All rights reserved" }
                    div { class: "flex gap-2 items-center py-1 pr-4 pl-2 rounded-full border border-transparent ring-1 shadow ring-foreground/5 bg-card",
                        div { class: "flex relative size-3",
                            span { class: "block absolute inset-0 bg-emerald-100 rounded-full animate-pulse duration-1500 size-full" }
                            span { class: "block relative m-auto bg-emerald-500 rounded-full size-1" }
                        }
                        span { class: "text-sm", "All Systems Normal" }
                    }
                }
            }
        }
    }
}
```