use dioxus::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};

#[component]
pub fn DemoAccordionCard() -> Element {
    rsx! {
        Card { class: "max-w-sm",
            CardHeader {
                CardTitle { "Subscription & Billing" }
                CardDescription { "Common questions about plans and payments." }
            }
            CardContent {
                Accordion {
                    AccordionItem {
                        AccordionTrigger { open: true,
                            AccordionTitle { "What plans do you offer?" }
                        }
                        AccordionContent { class: "pt-0",
                            AccordionDescription {
                                "We offer three tiers: Starter, Pro, and Enterprise. Each plan includes different usage limits and feature sets."
                            }
                        }
                    }
                    AccordionItem {
                        AccordionTrigger {
                            AccordionTitle { "Can I change my plan later?" }
                        }
                        AccordionContent { class: "pt-0",
                            AccordionDescription {
                                "Yes, you can upgrade or downgrade your plan at any time from your account settings. Changes take effect on your next billing cycle."
                            }
                        }
                    }
                    AccordionItem {
                        AccordionTrigger {
                            AccordionTitle { "How do I cancel my subscription?" }
                        }
                        AccordionContent { class: "pt-0",
                            AccordionDescription {
                                "You can cancel anytime from the billing section. Your access continues until the end of the current billing period."
                            }
                        }
                    }
                }
            }
        }
    }
}
