use dioxus::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::drawer::{
    Drawer, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHeader, DrawerPosition, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            Drawer {
                DrawerTrigger { "فتح اللوحة الجانبية" }

                DrawerContent { position: DrawerPosition::Right,
                    DrawerHeader {
                        DrawerTitle { "تعديل الملف الشخصي" }
                        DrawerDescription { "قم بإجراء تغييرات على ملفك الشخصي هنا." }
                    }

                    div { class: "p-4 text-sm text-muted-foreground",
                        "قم بتحديث اسمك المعروض وصورتك الرمزية وتفاصيل ملفك الشخصي الأخرى."
                    }

                    DrawerFooter {
                        DrawerClose { variant: ButtonVariant::Outline, "إلغاء" }
                        Button { "حفظ التغييرات" }
                    }
                }
            }
        }
    }
}
