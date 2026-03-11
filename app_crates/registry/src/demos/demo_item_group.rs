use icons::ChevronRight;
use leptos::prelude::*;

use crate::ui::avatar::{Avatar, AvatarFallback};
use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::item::{
    Item, ItemActions, ItemContent, ItemDescription, ItemGroup, ItemHeader, ItemMedia, ItemSeparator, ItemTitle,
};

#[component]
pub fn DemoItemGroup() -> impl IntoView {
    view! {
        <ItemGroup attr:role="list" class="w-full max-w-md rounded-md border">
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Message"</Badge>
                    <span class="text-xs text-muted-foreground">"2m ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"RS"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Ryan Smith sent a message"</ItemTitle>
                    <ItemDescription>"Hey, are you free for a quick call?"</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Alert"</Badge>
                    <span class="text-xs text-muted-foreground">"15m ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"SY"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Deploy completed"</ItemTitle>
                    <ItemDescription>"Production build succeeded with no errors."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
            <ItemSeparator />
            <Item class="py-3 px-4 rounded-none">
                <ItemHeader>
                    <Badge variant=BadgeVariant::Secondary>"Update"</Badge>
                    <span class="text-xs text-muted-foreground">"1h ago"</span>
                </ItemHeader>
                <ItemMedia>
                    <Avatar class="size-8">
                        <AvatarFallback>"MK"</AvatarFallback>
                    </Avatar>
                </ItemMedia>
                <ItemContent>
                    <ItemTitle>"Morgan updated the docs"</ItemTitle>
                    <ItemDescription>"Installation guide revised for Leptos 0.7."</ItemDescription>
                </ItemContent>
                <ItemActions>
                    <ChevronRight class="size-4 text-muted-foreground" />
                </ItemActions>
            </Item>
        </ItemGroup>
    }
}
