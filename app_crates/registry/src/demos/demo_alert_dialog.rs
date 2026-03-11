use leptos::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialog() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>Open Alert</AlertDialogTrigger>

            <AlertDialogContent class="w-[425px]">
                <AlertDialogBody>
                    <AlertDialogHeader>
                        <AlertDialogTitle>"Are you absolutely sure?"</AlertDialogTitle>

                        <AlertDialogDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </AlertDialogDescription>
                    </AlertDialogHeader>

                    <AlertDialogFooter>
                        <AlertDialogClose class="w-full sm:w-fit">"Cancel"</AlertDialogClose>
                        <Button attr:r#type="submit" class="w-full sm:w-fit">
                            "Continue"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
