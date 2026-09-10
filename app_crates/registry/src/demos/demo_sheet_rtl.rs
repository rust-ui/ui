use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Sheet {
                SheetTrigger { "فتح اللوحة الجانبية" }

                SheetContent { direction: SheetDirection::Right,
                    SheetHeader {
                        SheetTitle { "تعديل الملف الشخصي" }
                        SheetDescription { "قم بإجراء تغييرات على ملفك الشخصي هنا." }
                    }

                    div { class: "p-4 text-sm text-muted-foreground",
                        "قم بتحديث اسمك المعروض وصورتك الرمزية وتفاصيل ملفك الشخصي الأخرى."
                    }

                    SheetFooter {
                        SheetClose { variant: ButtonVariant::Outline, "إلغاء" }
                        Button { "حفظ التغييرات" }
                    }
                }
            }
        }
    }
}
