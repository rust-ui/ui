use dioxus::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordion() -> Element {
    rsx! {
        Accordion { class: "max-w-[400px]",
            AccordionItem {
                AccordionTrigger { open: true,
                    AccordionTitle { "Accordion item 01" }
                }
                AccordionContent { class: "pt-0",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
            AccordionItem {
                AccordionTrigger {
                    AccordionTitle { "Accordion item 02" }
                }
                AccordionContent { class: "pt-0",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
            AccordionItem {
                AccordionTrigger {
                    AccordionTitle { "Accordion item 03" }
                }
                AccordionContent { class: "pt-0",
                    AccordionDescription { "This is the Accordion description" }
                }
            }
        }
    }
}
