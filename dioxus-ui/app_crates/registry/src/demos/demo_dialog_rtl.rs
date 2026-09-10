use dioxus::prelude::*;

use crate::ui::button::Button;
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::input::Input;
use crate::ui::label::Label;

#[component]
pub fn DemoDialogRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl,
            Dialog {
                DialogTrigger { "فتح الحوار" }
                DialogContent { class: "sm:max-w-[425px]",
                    DialogBody {
                        DialogHeader {
                            DialogTitle { "تعديل الملف الشخصي" }
                            DialogDescription {
                                "قم بإجراء تغييرات على ملفك الشخصي هنا. انقر على حفظ عند الانتهاء."
                            }
                        }
                        div { class: "flex flex-col gap-4 justify-center",
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "rtl-name-1", "الاسم" }
                                Input { id: "rtl-name-1" }
                            }
                            div { class: "flex flex-col gap-2",
                                Label { html_for: "rtl-username-1", "اسم المستخدم" }
                                Input { id: "rtl-username-1" }
                            }
                        }
                        DialogFooter {
                            DialogClose { class: "w-full sm:w-fit", "إلغاء" }
                            Button { button_type: "submit", class: "w-full sm:w-fit", "حفظ التغييرات" }
                        }
                    }
                }
            }
        }
    }
}
