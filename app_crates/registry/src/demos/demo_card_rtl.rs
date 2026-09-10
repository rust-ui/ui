use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::card::{Card, CardContent, CardDescription, CardFooter, CardHeader, CardTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoCardRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-lg",
            Card { class: "max-w-lg",
                CardHeader {
                    CardTitle { "عنوان البطاقة" }
                }
                CardContent {
                    CardDescription {
                        "مرحباً، هذا وصف تفصيلي لمحتوى البطاقة. يمكنك إضافة المزيد من النصوص هنا لتوفير معلومات إضافية حول غرض البطاقة وميزاتها وأي تفاصيل أخرى ذات صلة."
                    }
                }
                CardFooter { class: "justify-end",
                    Button { variant: ButtonVariant::Outline, "إلغاء" }
                    Button { "تأكيد" }
                }
            }
        }
    }
}
