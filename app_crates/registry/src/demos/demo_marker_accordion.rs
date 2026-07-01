use icons::{FileSearch, FolderOpen};
use leptos::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle,
    AccordionTrigger,
};
use crate::ui::marker::{Marker, MarkerContent, MarkerIcon, MarkerVariant};
use crate::ui::spinner::Spinner;

#[component]
pub fn DemoMarkerAccordion() -> impl IntoView {
    view! {
        <div class="flex w-full flex-col gap-2">
            <Marker variant=MarkerVariant::Separator>
                <MarkerIcon><Spinner /></MarkerIcon>
                <MarkerContent>
                    <Accordion class="w-full">
                        <AccordionItem>
                            <AccordionTrigger open=true>
                                <AccordionTitle class="text-sm font-normal text-muted-foreground">
                                    "Exploring codebase"
                                </AccordionTitle>
                            </AccordionTrigger>
                            <AccordionContent class="pt-0">
                                <div class="flex flex-col gap-1 text-xs text-muted-foreground">
                                    <div class="flex items-center gap-1.5">
                                        <FileSearch class="size-3" />
                                        "Searched for " <code>"use_message_scroller"</code>
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        <FolderOpen class="size-3" />
                                        "Opened " <code>"src/ui/message_scroller.rs"</code>
                                    </div>
                                    <div class="flex items-center gap-1.5">
                                        <FileSearch class="size-3" />
                                        "Searched for " <code>"ScrollMode"</code>
                                    </div>
                                </div>
                            </AccordionContent>
                        </AccordionItem>
                    </Accordion>
                </MarkerContent>
            </Marker>
        </div>
    }
}
