use dioxus::prelude::*;
use icons::{AlignHorizontalSpaceAround, Blocks, Compass, LogIn, PanelLeft, Search};

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionLink, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordionIcons() -> Element {
    rsx! {
        Accordion { class: "overflow-hidden w-full rounded-lg border bg-background max-w-[500px]",
            AccordionItem {
                AccordionTrigger { open: true, class: "peer-checked:bg-accent hover:bg-accent",
                    AccordionTitle { "Registry" }
                }
                AccordionContent { class: "p-0",
                    ul { class: "text-sm",
                        li {
                            AccordionLink { href: "/docs/components",
                                Blocks {}
                                span { "Components" }
                            }
                        }
                        li {
                            AccordionLink { href: "/docs/hooks",
                                Compass {}
                                span { "Hooks" }
                            }
                        }
                        li {
                            AccordionLink { href: "/icons",
                                Search {}
                                span { "Icons" }
                            }
                        }
                    }
                }
            }
            AccordionItem {
                AccordionTrigger { class: "peer-checked:bg-accent hover:bg-accent",
                    AccordionTitle { "Blocks" }
                }
                AccordionContent { class: "p-0",
                    ul { class: "text-sm",
                        li {
                            AccordionLink { href: "/blocks/login",
                                LogIn {}
                                span { "Login" }
                            }
                        }
                        li {
                            AccordionLink { href: "/blocks/sidenav",
                                PanelLeft {}
                                span { "Sidenav" }
                            }
                        }
                        li {
                            AccordionLink { href: "/blocks/parallax",
                                AlignHorizontalSpaceAround {}
                                span { "Parallax" }
                            }
                        }
                    }
                }
            }
            AccordionLink { class: "p-3", href: "/charts",
                AccordionTitle { "Charts" }
            }
            AccordionLink { class: "p-3", href: "/icons",
                AccordionTitle { "Icons" }
            }
        }
    }
}
