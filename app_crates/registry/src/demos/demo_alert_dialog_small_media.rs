use icons::Bluetooth;
use leptos::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;

#[component]
pub fn DemoAlertDialogSmallMedia() -> impl IntoView {
    view! {
        <AlertDialog>
            <AlertDialogTrigger>"Show Dialog"</AlertDialogTrigger>
            <AlertDialogContent class="w-[300px]">
                <AlertDialogBody>
                    <AlertDialogHeader class="items-center sm:items-center sm:text-center">
                        <div class="flex justify-center items-center mb-2 rounded-md size-10 bg-muted">
                            <Bluetooth class="size-5" />
                        </div>
                        <AlertDialogTitle>"Allow accessory to connect?"</AlertDialogTitle>
                        <AlertDialogDescription>
                            "Do you want to allow the Bluetooth accessory to connect to this device?"
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
