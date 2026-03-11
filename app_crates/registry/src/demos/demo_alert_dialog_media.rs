use icons::CirclePlus;
use leptos::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogMedia() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Share Project"</AlertDialogTrigger>
            <AlertDialogContent class="w-[425px]">
                <AlertDialogBody>
                    <AlertDialogHeader>
                        <div class="flex justify-center items-center mb-2 rounded-md size-10 bg-muted">
                            <CirclePlus class="size-5" />
                        </div>
                        <AlertDialogTitle>"Share this project?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Anyone with the link will be able to view and edit this project."
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter>
                        <AlertDialogClose class="w-full sm:w-fit">"Cancel"</AlertDialogClose>
                        <Button attr:r#type="submit" class="w-full sm:w-fit">
                            "Share"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
