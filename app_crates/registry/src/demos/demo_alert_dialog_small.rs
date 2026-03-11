use leptos::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmall() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Show Dialog"</AlertDialogTrigger>
            <AlertDialogContent class="w-[300px]">
                <AlertDialogBody>
                    <AlertDialogHeader class="items-center sm:items-center sm:text-center">
                        <AlertDialogTitle>"Allow accessory to connect?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Do you want to allow the USB accessory to connect to this device?"
                        </AlertDialogDescription>
                    </AlertDialogHeader>
                    <AlertDialogFooter class="flex-row">
                        <AlertDialogClose class="flex-1">"Don't allow"</AlertDialogClose>
                        <Button attr:r#type="submit" class="flex-1">
                            "Allow"
                        </Button>
                    </AlertDialogFooter>
                </AlertDialogBody>
            </AlertDialogContent>
        </AlertDialog>
    }
}
