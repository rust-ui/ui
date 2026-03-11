use leptos::prelude::*;

use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoBreadcrumbRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-lg">
            <Breadcrumb>
                <BreadcrumbList>
                    <BreadcrumbItem>
                        <BreadcrumbLink attr:href="/">"الرئيسية"</BreadcrumbLink>
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbEllipsis />
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbLink attr:href="/demo-components/button">"المكونات"</BreadcrumbLink>
                    </BreadcrumbItem>

                    <BreadcrumbSeparator />

                    <BreadcrumbItem>
                        <BreadcrumbPage>"التوثيق"</BreadcrumbPage>
                    </BreadcrumbItem>
                </BreadcrumbList>
            </Breadcrumb>
        </DirectionProvider>
    }
}
