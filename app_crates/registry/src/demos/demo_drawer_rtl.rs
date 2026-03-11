use leptos::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::drawer::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerHandle, DrawerHeader, DrawerTitle,
    DrawerTrigger,
};

#[component]
pub fn DemoDrawerRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Drawer>
                <DrawerTrigger>"فتح الدرج"</DrawerTrigger>

                <DrawerContent>
                    <DrawerHandle />
                    <DrawerBody>
                        <DrawerHeader>
                            <DrawerTitle>"عنوان الدرج"</DrawerTitle>
                            <DrawerDescription>
                                "اسحب للأسفل للإغلاق أو انقر خارج الدرج."
                            </DrawerDescription>
                        </DrawerHeader>

                        <DrawerClose>"إغلاق"</DrawerClose>
                    </DrawerBody>
                </DrawerContent>
            </Drawer>
        </DirectionProvider>
    }
}
