use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            Drawer {
                DrawerTrigger { "فتح الدرج" }

                DrawerContent {
                    DrawerHandle {}
                    DrawerBody {
                        DrawerHeader {
                            DrawerTitle { "عنوان الدرج" }
                            DrawerDescription { "اسحب للأسفل للإغلاق أو انقر خارج الدرج." }
                        }
                        DrawerClose { "إغلاق" }
                    }
                }
            }
        }
    }
}
