use icons::{FileText, Lock, TriangleAlert, X};
use leptos::prelude::*;

use crate::ui::drawer::{Drawer, DrawerContent, DrawerTrigger};

#[component]
pub fn DemoDrawerFamily() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent class="overflow-hidden right-4 left-4 pb-6 mx-auto mb-4 max-w-[361px] rounded-[36px]">
                <header class="flex justify-between items-center mb-4 border-b h-[72px] border-neutral-100">
                    <h2 class="text-lg font-semibold text-foreground">"Options"</h2>
                    <button
                        data-name="DrawerClose"
                        class="flex justify-center items-center rounded-full transition-colors size-8 bg-neutral-100 text-neutral-500 hover:bg-neutral-200"
                    >
                        <X class="size-3" />
                    </button>
                </header>

                <div class="space-y-3">
                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-foreground hover:bg-neutral-200">
                        <Lock class="size-[18px]" />
                        <span>"View Private Key"</span>
                    </button>

                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-foreground hover:bg-neutral-200">
                        <FileText class="size-[18px]" />
                        <span>"View Recovery Phrase"</span>
                    </button>

                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium text-red-600 bg-red-50 rounded-2xl transition-colors hover:bg-red-100">
                        <TriangleAlert class="size-[18px]" />
                        <span>"Remove Wallet"</span>
                    </button>
                </div>
            </DrawerContent>
        </Drawer>
    }
}
