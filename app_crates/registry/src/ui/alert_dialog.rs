use leptos::prelude::*;

use crate::ui::button::{ButtonSize, ButtonVariant};
use crate::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};

#[component]
pub fn AlertDialog(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! { <Dialog class=class>{children()}</Dialog> }
}

#[component]
pub fn AlertDialogTrigger(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    view! {
        <DialogTrigger class=class variant=variant size=size>
            {children()}
        </DialogTrigger>
    }
}

#[component]
pub fn AlertDialogContent(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogContent class=class close_on_backdrop_click=false data_name_prefix="AlertDialog">
            {children()}
        </DialogContent>
    }
}

#[component]
pub fn AlertDialogBody(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogBody class=class attr:data-name="AlertDialogBody">
            {children()}
        </DialogBody>
    }
}

#[component]
pub fn AlertDialogHeader(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogHeader class=class attr:data-name="AlertDialogHeader">
            {children()}
        </DialogHeader>
    }
}

#[component]
pub fn AlertDialogTitle(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogTitle class=class attr:data-name="AlertDialogTitle">
            {children()}
        </DialogTitle>
    }
}

#[component]
pub fn AlertDialogDescription(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogDescription class=class attr:data-name="AlertDialogDescription">
            {children()}
        </DialogDescription>
    }
}

#[component]
pub fn AlertDialogFooter(children: Children, #[prop(optional, into)] class: String) -> impl IntoView {
    view! {
        <DialogFooter class=class attr:data-name="AlertDialogFooter">
            {children()}
        </DialogFooter>
    }
}

#[component]
pub fn AlertDialogClose(
    children: Children,
    #[prop(optional, into)] class: String,
    #[prop(default = ButtonVariant::Outline)] variant: ButtonVariant,
    #[prop(default = ButtonSize::Default)] size: ButtonSize,
) -> impl IntoView {
    view! {
        <DialogClose class=class variant=variant size=size>
            {children()}
        </DialogClose>
    }
}
