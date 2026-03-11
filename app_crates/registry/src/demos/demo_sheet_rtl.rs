use leptos::prelude::*;

use crate::ui::button::{Button, ButtonVariant};
use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::sheet::{
    Sheet, SheetClose, SheetContent, SheetDescription, SheetDirection, SheetFooter, SheetHeader, SheetTitle,
    SheetTrigger,
};

#[component]
pub fn DemoSheetRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="max-w-fit">
            <Sheet>
                <SheetTrigger>"فتح اللوحة الجانبية"</SheetTrigger>

                <SheetContent direction=SheetDirection::Right>
                    <SheetHeader>
                        <SheetTitle>"تعديل الملف الشخصي"</SheetTitle>
                        <SheetDescription>
                            "قم بإجراء تغييرات على ملفك الشخصي هنا."
                        </SheetDescription>
                    </SheetHeader>

                    <div class="p-4 text-sm text-muted-foreground">
                        "قم بتحديث اسمك المعروض وصورتك الرمزية وتفاصيل ملفك الشخصي الأخرى."
                    </div>

                    <SheetFooter>
                        <SheetClose variant=ButtonVariant::Outline>"إلغاء"</SheetClose>
                        <Button>"حفظ التغييرات"</Button>
                    </SheetFooter>
                </SheetContent>
            </Sheet>
        </DirectionProvider>
    }
}
