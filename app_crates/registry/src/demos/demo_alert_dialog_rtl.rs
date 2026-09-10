use dioxus::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertDialogRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "max-w-fit",
            AlertDialog {
                AlertDialogTrigger { "فتح التنبيه" }
                AlertDialogContent { class: "w-[425px]",
                    AlertDialogBody {
                        AlertDialogHeader {
                            AlertDialogTitle { "هل أنت متأكد تماماً؟" }
                            AlertDialogDescription {
                                "لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائياً وإزالة بياناتك من خوادمنا."
                            }
                        }
                        AlertDialogFooter {
                            AlertDialogClose { class: "w-full sm:w-fit", "إلغاء" }
                            Button { button_type: "submit", class: "w-full sm:w-fit", "متابعة" }
                        }
                    }
                }
            }
        }
    }
}
