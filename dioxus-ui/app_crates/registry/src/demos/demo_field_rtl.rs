use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::field::{
    Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSeparator, FieldSet, FieldVariant,
};
use crate::ui::input::Input;

#[component]
pub fn DemoFieldRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-md",
            div { class: "w-full max-w-md",
                FieldGroup {
                    FieldSet {
                        FieldLegend { "طريقة الدفع" }
                        FieldDescription { "جميع المعاملات آمنة ومشفرة." }
                        FieldGroup {
                            Field {
                                FieldLabel { html_for: "rtl-card-name", "الاسم على البطاقة" }
                                Input { id: "rtl-card-name", placeholder: "محمد علي" }
                            }
                            Field {
                                FieldLabel { html_for: "rtl-card-number", "رقم البطاقة" }
                                Input {
                                    id: "rtl-card-number",
                                    placeholder: "١٢٣٤ ٥٦٧٨ ٩٠١٢ ٣٤٥٦",
                                }
                                FieldDescription { "أدخل رقم بطاقتك المكون من ١٦ رقماً." }
                            }
                            div { class: "grid grid-cols-3 gap-4",
                                Field {
                                    FieldLabel { html_for: "rtl-exp-month", "الشهر" }
                                    Input { id: "rtl-exp-month", placeholder: "MM" }
                                }
                                Field {
                                    FieldLabel { html_for: "rtl-exp-year", "السنة" }
                                    Input { id: "rtl-exp-year", placeholder: "YYYY" }
                                }
                                Field {
                                    FieldLabel { html_for: "rtl-cvv", "CVV" }
                                    Input { id: "rtl-cvv", placeholder: "١٢٣" }
                                }
                            }
                        }
                    }
                    FieldSeparator {}
                    FieldSet {
                        FieldLegend { "عنوان الفوترة" }
                        FieldDescription { "عنوان الفوترة المرتبط بطريقة الدفع الخاصة بك." }
                        FieldGroup {
                            Field { variant: FieldVariant::Horizontal,
                                Checkbox { checked: true }
                                FieldLabel { class: "font-normal", "نفس عنوان الشحن" }
                            }
                        }
                    }
                    Field { variant: FieldVariant::Horizontal,
                        Button { "إرسال" }
                        Button { variant: ButtonVariant::Outline, "إلغاء" }
                    }
                }
            }
        }
    }
}
