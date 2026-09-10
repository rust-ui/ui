use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};
use crate::ui::input::Input;

#[component]
pub fn DemoField() -> Element {
    rsx! {
        div { class: "w-full max-w-md",
            FieldGroup {
                FieldSet {
                    FieldLegend { "Payment Method" }
                    FieldDescription { "All transactions are secure and encrypted." }
                    FieldGroup {
                        Field {
                            FieldLabel { html_for: "card-name", "Name on Card" }
                            Input { id: "card-name", placeholder: "Evil Rabbit" }
                        }
                        Field {
                            FieldLabel { html_for: "card-number", "Card Number" }
                            Input { id: "card-number", placeholder: "1234 5678 9012 3456" }
                            FieldDescription { "Enter your 16-digit card number." }
                        }
                        div { class: "grid grid-cols-3 gap-4",
                            Field {
                                FieldLabel { html_for: "exp-month", "Month" }
                                Input { id: "exp-month", placeholder: "MM" }
                            }
                            Field {
                                FieldLabel { html_for: "exp-year", "Year" }
                                Input { id: "exp-year", placeholder: "YYYY" }
                            }
                            Field {
                                FieldLabel { html_for: "cvv", "CVV" }
                                Input { id: "cvv", placeholder: "123" }
                            }
                        }
                    }
                }
                FieldSeparator {}
                FieldSet {
                    FieldLegend { "Billing Address" }
                    FieldDescription {
                        "The billing address associated with your payment method."
                    }
                    FieldGroup {
                        Field { variant: FieldVariant::Horizontal,
                            Checkbox { checked: true }
                            FieldLabel { class: "font-normal", "Same as shipping address" }
                        }
                    }
                }
                Field { variant: FieldVariant::Horizontal,
                    Button { "Submit" }
                    Button { variant: ButtonVariant::Outline, "Cancel" }
                }
            }
        }
    }
}
