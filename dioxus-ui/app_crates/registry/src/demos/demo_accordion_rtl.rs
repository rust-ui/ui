use dioxus::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAccordionRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-[400px]",
            Accordion { class: "max-w-[400px]",
                AccordionItem {
                    AccordionTrigger { open: true,
                        AccordionTitle { "البند الأول" }
                    }
                    AccordionContent { class: "pt-0",
                        AccordionDescription { "هذا هو وصف العنصر القابل للطي." }
                    }
                }
                AccordionItem {
                    AccordionTrigger {
                        AccordionTitle { "البند الثاني" }
                    }
                    AccordionContent { class: "pt-0",
                        AccordionDescription { "هذا هو وصف العنصر القابل للطي." }
                    }
                }
                AccordionItem {
                    AccordionTrigger {
                        AccordionTitle { "البند الثالث" }
                    }
                    AccordionContent { class: "pt-0",
                        AccordionDescription { "هذا هو وصف العنصر القابل للطي." }
                    }
                }
            }
        }
    }
}
