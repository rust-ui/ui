use leptos::prelude::*;

use crate::ui::breadcrumb::{
    Breadcrumb, BreadcrumbEllipsis, BreadcrumbItem, BreadcrumbLink, BreadcrumbList, BreadcrumbPage, BreadcrumbSeparator,
};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLink, DropdownMenuTrigger,
};

#[component]
pub fn DemoBreadcrumbDropdown() -> impl IntoView {
    view! {
        <Breadcrumb>
            <BreadcrumbList>
                <BreadcrumbItem>
                    <BreadcrumbLink attr:href="/">Home</BreadcrumbLink>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <DropdownMenu>
                        <DropdownMenuTrigger>
                            <BreadcrumbEllipsis />
                        </DropdownMenuTrigger>
                        <DropdownMenuContent>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs">"Documentation"</DropdownMenuLink>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs/components">"Components"</DropdownMenuLink>
                            </DropdownMenuItem>
                            <DropdownMenuItem>
                                <DropdownMenuLink attr:href="/docs/components/button">"Button"</DropdownMenuLink>
                            </DropdownMenuItem>
                        </DropdownMenuContent>
                    </DropdownMenu>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <BreadcrumbLink attr:href="/docs/components/breadcrumb">Breadcrumb</BreadcrumbLink>
                </BreadcrumbItem>

                <BreadcrumbSeparator />

                <BreadcrumbItem>
                    <BreadcrumbPage>"Dropdown"</BreadcrumbPage>
                </BreadcrumbItem>
            </BreadcrumbList>
        </Breadcrumb>
    }
}
