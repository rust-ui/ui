use leptos::prelude::*;

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::button::Button;
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAlertDialogRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <AlertDialog>
                <AlertDialogTrigger>"فتح التنبيه"</AlertDialogTrigger>

                <AlertDialogContent class="w-[425px]">
                    <AlertDialogBody>
                        <AlertDialogHeader>
                            <AlertDialogTitle>"هل أنت متأكد تماماً؟"</AlertDialogTitle>

                            <AlertDialogDescription>
                                "لا يمكن التراجع عن هذا الإجراء. سيؤدي هذا إلى حذف حسابك نهائياً وإزالة بياناتك من خوادمنا."
                            </AlertDialogDescription>
                        </AlertDialogHeader>

                        <AlertDialogFooter>
                            <AlertDialogClose class="w-full sm:w-fit">"إلغاء"</AlertDialogClose>
                            <Button attr:r#type="submit" class="w-full sm:w-fit">
                                "متابعة"
                            </Button>
                        </AlertDialogFooter>
                    </AlertDialogBody>
                </AlertDialogContent>
            </AlertDialog>
        </DirectionProvider>
    }
}
