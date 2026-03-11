use icons::File;
use leptos::prelude::*;

use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemGroup, ItemMedia, ItemMediaVariant, ItemSeparator, ItemSize, ItemTitle,
};
use crate::ui::progress::Progress;

struct UploadFile {
    name: &'static str,
    progress: f64,
    time_remaining: &'static str,
}

#[component]
pub fn DemoItemFileUpload() -> impl IntoView {
    let files = vec![
        UploadFile { name: "document.pdf", progress: 45.0, time_remaining: "2m 30s" },
        UploadFile { name: "presentation.pptx", progress: 78.0, time_remaining: "45s" },
        UploadFile { name: "spreadsheet.xlsx", progress: 12.0, time_remaining: "5m 12s" },
        UploadFile { name: "image.jpg", progress: 100.0, time_remaining: "Complete" },
    ];

    view! {
        <ItemGroup attr:role="list" class="w-full max-w-md rounded-md border">
            {files
                .into_iter()
                .enumerate()
                .map(|(i, file)| {
                    let is_last = i == 3;
                    view! {
                        <Item size=ItemSize::Xs class="px-4">
                            <ItemMedia variant=ItemMediaVariant::Icon>
                                <File class="size-5" />
                            </ItemMedia>
                            <ItemContent class="min-w-0 truncate">
                                <ItemTitle class="inline truncate">{file.name}</ItemTitle>
                            </ItemContent>
                            <ItemContent class="flex-none">
                                <Progress value=Signal::derive(move || file.progress) class="w-24" />
                            </ItemContent>
                            <ItemActions class="justify-end w-16">
                                <span class="text-xs text-muted-foreground">{file.time_remaining}</span>
                            </ItemActions>
                        </Item>
                        {(!is_last).then(|| view! { <ItemSeparator /> })}
                    }
                })
                .collect_view()}
        </ItemGroup>
    }
}
