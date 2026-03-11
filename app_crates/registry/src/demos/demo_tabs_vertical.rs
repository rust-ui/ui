use leptos::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsOrientation, TabsTrigger};

#[component]
pub fn DemoTabsVertical() -> impl IntoView {
    view! {
        <Tabs default_value="general" orientation=TabsOrientation::Vertical class="w-full max-w-md">
            <TabsList>
                <TabsTrigger value="general">General</TabsTrigger>
                <TabsTrigger value="security">Security</TabsTrigger>
                <TabsTrigger value="notifications">Notifications</TabsTrigger>
            </TabsList>
            <TabsContent value="general">
                <p class="text-muted-foreground">Configure general application settings.</p>
            </TabsContent>
            <TabsContent value="security">
                <p class="text-muted-foreground">Manage your security preferences and two-factor authentication.</p>
            </TabsContent>
            <TabsContent value="notifications">
                <p class="text-muted-foreground">Control how and when you receive notifications.</p>
            </TabsContent>
        </Tabs>
    }
}
