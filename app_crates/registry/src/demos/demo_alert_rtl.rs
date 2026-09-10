use dioxus::prelude::*;
use icons::Terminal;

use crate::ui::alert::{Alert, AlertDescription, AlertTitle};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-lg",
            Alert {
                Terminal {}
                AlertTitle { "تنبيه!" }
                AlertDescription {
                    "يمكنك إضافة المكونات إلى تطبيقك باستخدام واجهة سطر الأوامر."
                }
            }
        }
    }
}
