use dioxus::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordionBordered() -> Element {
    rsx! {
        Accordion { class: "overflow-hidden rounded-lg border bg-background max-w-[400px]",
            AccordionItem {
                AccordionTrigger { open: true, class: "peer-checked:bg-accent hover:bg-accent",
                    AccordionTitle { "Accordion item 01" }
                }
                AccordionContent { class: "pt-2",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
            AccordionItem {
                AccordionTrigger { class: "peer-checked:bg-accent hover:bg-accent",
                    AccordionTitle { "Accordion item 02" }
                }
                AccordionContent { class: "pt-2",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
            AccordionItem {
                AccordionTrigger { class: "peer-checked:bg-accent hover:bg-accent",
                    AccordionTitle { "Accordion item 03" }
                }
                AccordionContent { class: "pt-2",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
        }
    }
}
