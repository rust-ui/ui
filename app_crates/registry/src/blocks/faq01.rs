use dioxus::prelude::*;

use crate::ui::badge::{Badge, BadgeVariant};

/*
 * title: FAQ with Numbered Grid
*/

#[component]
fn FaqNumber(children: Element) -> Element {
    rsx! {
        span { class: "flex justify-center items-center font-mono text-xs rounded-sm size-6 shrink-0 bg-secondary text-primary",
            {children}
        }
    }
}

#[component]
fn FaqItem(children: Element) -> Element {
    rsx! {
        div { class: "flex gap-4", {children} }
    }
}

#[component]
fn FaqContent(children: Element) -> Element {
    rsx! {
        div { {children} }
    }
}

#[component]
fn FaqQuestion(children: Element) -> Element {
    rsx! {
        h3 { class: "mb-2 text-lg font-medium", {children} }
    }
}

#[component]
fn FaqAnswer(children: Element) -> Element {
    rsx! {
        p { class: "text-sm text-muted-foreground", {children} }
    }
}

#[component]
pub fn Faq01() -> Element {
    rsx! {
        section { class: "py-20",
            div { class: "container",
                div { class: "text-center",
                    Badge { variant: BadgeVariant::Outline, "FAQ" }
                    h1 { class: "mt-4 text-4xl font-semibold", "Common Questions & Answers" }
                    p { class: "mt-6 font-medium text-muted-foreground",
                        "Find out all the essential details about our platform and how it can serve your needs."
                    }
                }
                div { class: "grid gap-8 mx-auto mt-14 md:grid-cols-2 md:gap-12",
                    FaqItem {
                        FaqNumber { "1" }
                        FaqContent {
                            FaqQuestion { "What is a FAQ and why is it important?" }
                            FaqAnswer {
                                "FAQ stands for Frequently Asked Questions. It is a list that provides answers to common questions people may have about a specific product, service, or topic."
                            }
                        }
                    }
                    FaqItem {
                        FaqNumber { "2" }
                        FaqContent {
                            FaqQuestion { "Why should I use a FAQ on my website or app?" }
                            FaqAnswer {
                                "Utilizing a FAQ section on your website or app is a practical way to offer instant assistance to your users or customers. Instead of waiting for customer support responses, they can find quick answers to commonly asked questions."
                            }
                        }
                    }
                    FaqItem {
                        FaqNumber { "3" }
                        FaqContent {
                            FaqQuestion { "How do I effectively create a FAQ section?" }
                            FaqAnswer {
                                "Creating a FAQ section starts with gathering the most frequent questions you receive from your users or customers. Once you have a list, you need to write clear, detailed, and helpful answers to each question."
                            }
                        }
                    }
                    FaqItem {
                        FaqNumber { "4" }
                        FaqContent {
                            FaqQuestion { "What are the benefits of having a well-maintained FAQ section?" }
                            FaqAnswer {
                                "There are numerous advantages to maintaining a robust FAQ section. Firstly, it provides immediate answers to common queries, which improves the user experience."
                            }
                        }
                    }
                    FaqItem {
                        FaqNumber { "5" }
                        FaqContent {
                            FaqQuestion { "How should I organize my FAQ for optimal usability?" }
                            FaqAnswer {
                                "An organized FAQ is critical for user-friendliness. Start by grouping similar questions into categories, such as Billing, Account Setup, or Technical Support. This way, users can quickly find the section that addresses their specific concerns."
                            }
                        }
                    }
                    FaqItem {
                        FaqNumber { "6" }
                        FaqContent {
                            FaqQuestion { "How often should I update my FAQ, and why is it necessary?" }
                            FaqAnswer {
                                "Regular updates to your FAQ are essential to keeping the information accurate and relevant. As your product or service evolves, so will the types of questions your users ask."
                            }
                        }
                    }
                }
            }
        }
    }
}
