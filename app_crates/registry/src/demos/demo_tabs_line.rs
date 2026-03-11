use leptos::prelude::*;

use crate::ui::tabs::{Tabs, TabsContent, TabsList, TabsTrigger, TabsVariant};

#[component]
pub fn DemoTabsLine() -> impl IntoView {
    view! {
        <Tabs default_value="preview" class="w-full max-w-sm">
            <TabsList variant=TabsVariant::Line>
                <TabsTrigger value="preview">Preview</TabsTrigger>
                <TabsTrigger value="code">Code</TabsTrigger>
                <TabsTrigger value="output">Output</TabsTrigger>
            </TabsList>
            <TabsContent value="preview">
                <p class="text-muted-foreground">Live preview of your component.</p>
            </TabsContent>
            <TabsContent value="code">
                <p class="text-muted-foreground">View and edit the source code.</p>
            </TabsContent>
            <TabsContent value="output">
                <p class="text-muted-foreground">Compiled output and build logs.</p>
            </TabsContent>
        </Tabs>
    }
}
