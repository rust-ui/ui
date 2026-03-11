use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabsRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Tabs default_value="account" class="w-full max-w-sm">
                <TabsList>
                    <TabsTrigger value="account">"الحساب"</TabsTrigger>
                    <TabsTrigger value="password">"كلمة المرور"</TabsTrigger>
                    <TabsTrigger value="settings">"الإعدادات"</TabsTrigger>
                </TabsList>
                <TabsContent value="account">
                    <p class="text-muted-foreground">
                        "إدارة إعدادات حسابك ومعلومات ملفك الشخصي."
                    </p>
                </TabsContent>
                <TabsContent value="password">
                    <p class="text-muted-foreground">
                        "تغيير كلمة المرور وتفضيلات الأمان."
                    </p>
                </TabsContent>
                <TabsContent value="settings">
                    <p class="text-muted-foreground">"ضبط إعدادات الإشعارات والخصوصية."</p>
                </TabsContent>
            </Tabs>
        </DirectionProvider>
    }
}
