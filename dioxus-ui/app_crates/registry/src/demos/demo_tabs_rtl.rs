use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabsRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-sm",
            Tabs { default_value: "account", class: "w-full max-w-sm",
                TabsList {
                    TabsTrigger { value: "account", "الحساب" }
                    TabsTrigger { value: "password", "كلمة المرور" }
                    TabsTrigger { value: "settings", "الإعدادات" }
                }
                TabsContent { value: "account",
                    p { class: "text-muted-foreground", "إدارة إعدادات حسابك ومعلومات ملفك الشخصي." }
                }
                TabsContent { value: "password",
                    p { class: "text-muted-foreground", "تغيير كلمة المرور وتفضيلات الأمان." }
                }
                TabsContent { value: "settings",
                    p { class: "text-muted-foreground", "ضبط إعدادات الإشعارات والخصوصية." }
                }
            }
        }
    }
}
