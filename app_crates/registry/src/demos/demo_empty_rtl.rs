use icons::{ArrowUpRight, FolderCode};
use leptos::prelude::*;

use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::empty::{Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle};

#[component]
pub fn DemoEmptyRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-md">
            <Empty>
                <EmptyHeader>
                    <EmptyMedia variant=EmptyMediaVariant::Icon>
                        <FolderCode />
                    </EmptyMedia>
                    <EmptyTitle>"لا توجد مشاريع بعد"</EmptyTitle>
                    <EmptyDescription>
                        "لم تقم بإنشاء أي مشاريع بعد. ابدأ بإنشاء مشروعك الأول."
                    </EmptyDescription>
                </EmptyHeader>
                <EmptyContent>
                    <div class="flex gap-2">
                        <Button>"إنشاء مشروع"</Button>
                        <Button variant=ButtonVariant::Outline>"استيراد مشروع"</Button>
                    </div>
                </EmptyContent>
                <Button variant=ButtonVariant::Link size=ButtonSize::Sm class="text-muted-foreground">
                    <a href="#" class="flex gap-1 items-center">
                        <span>"معرفة المزيد"</span>
                        <ArrowUpRight class="rtl:rotate-180" />
                    </a>
                </Button>
            </Empty>
        </DirectionProvider>
    }
}
