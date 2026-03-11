use leptos::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger};

#[component]
pub fn DemoTabs() -> impl IntoView {
    view! {
        <Tabs default_value="account" class="w-full max-w-sm">
            <TabsList>
                <TabsTrigger value="account">Account</TabsTrigger>
                <TabsTrigger value="password">Password</TabsTrigger>
                <TabsTrigger value="settings">Settings</TabsTrigger>
            </TabsList>
            <TabsContent value="account">
                <p class="text-muted-foreground">Manage your account settings and profile information.</p>
            </TabsContent>
            <TabsContent value="password">
                <p class="text-muted-foreground">Change your password and security preferences.</p>
            </TabsContent>
            <TabsContent value="settings">
                <p class="text-muted-foreground">Configure your notification and privacy settings.</p>
            </TabsContent>
        </Tabs>
    }
}
