use leptos::prelude::*;

use crate::ui::drawer::DrawerHandle;

#[component]
pub fn DemoDrawerNested() -> impl IntoView {
    view! {
        // <link rel="stylesheet" href="/drawer.css" />
        <link rel="stylesheet" href="/drawer_nested.css" />

        <div>
            <div class="text-center">
                <h1 class="mb-4 text-4xl font-bold text-foreground">Nested Drawer Demo</h1>
                <p class="mb-8 text-muted-foreground">Click the button below to open the drawer</p>
                <button
                    id="drawer-trigger"
                    class="py-3 px-6 font-medium rounded-lg transition-opacity hover:opacity-90 bg-primary text-primary-foreground"
                >
                    Open Drawer
                </button>
            </div>

            // <!-- Drawer Overlay -->
            <div
                id="drawer-overlay"
                class="hidden drawer-overlay"
                data-vaul-overlay=""
                data-vaul-snap-points="false"
                data-vaul-animate="true"
                data-state="closed"
            ></div>

            // <!-- Drawer Content -->
            <div
                id="drawer-content"
                class="hidden bg-white drawer-content rounded-t-[10px]"
                data-vaul-drawer=""
                data-vaul-drawer-direction="bottom"
                data-vaul-snap-points="false"
                data-vaul-animate="true"
                data-state="closed"
                style="--initial-transform: 100%;"
            >
                // <!-- Drawer Body -->
                <div class="flex overflow-y-auto flex-col flex-1 gap-4 p-6">
                    // <!-- Handle inside DrawerBody -->
                    <DrawerHandle />

                    <h2 class="text-2xl font-bold text-foreground">Parent Drawer</h2>
                    <p class="text-muted-foreground">
                        Click the button below to open a nested drawer and see the parent drawer scale down.
                    </p>

                    <button
                        id="open-nested-drawer"
                        class="py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-secondary text-secondary-foreground focus:ring-secondary"
                    >
                        Open Nested Drawer
                    </button>

                    <button
                        id="drawer-close"
                        class="py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground focus:ring-primary"
                    >
                        Close
                    </button>
                </div>
            </div>

            // <!-- Nested Drawer Overlay -->
            <div
                id="nested-drawer-overlay"
                class="hidden drawer-overlay overlay-nested"
                data-vaul-overlay=""
                data-vaul-snap-points="false"
                data-vaul-animate="true"
                data-state="closed"
            ></div>

            // <!-- Nested Drawer Content -->
            <div
                id="nested-drawer-content"
                class="hidden bg-white drawer-content drawer-nested rounded-t-[10px]"
                data-vaul-drawer=""
                data-vaul-drawer-direction="bottom"
                data-vaul-snap-points="false"
                data-vaul-animate="true"
                data-state="closed"
                style="--initial-transform: 100%;"
            >
                // <!-- Nested Drawer Body -->
                <div class="flex overflow-y-auto flex-col flex-1 gap-4 p-6">
                    // <!-- Handle inside DrawerBody -->
                    <DrawerHandle />

                    <h2 class="text-2xl font-bold text-foreground">Nested Drawer</h2>
                    <p class="text-muted-foreground">
                        Notice how the parent drawer scales down when this nested drawer opens.
                    </p>

                    <button
                        id="nested-drawer-close"
                        class="py-2 px-4 w-full font-medium rounded-md transition-opacity hover:opacity-90 focus:ring-2 focus:ring-offset-2 focus:outline-none bg-primary text-primary-foreground focus:ring-primary"
                    >
                        Close Nested Drawer
                    </button>
                </div>
            </div>
        </div>

        <script type="module" src="/drawer_v3.js"></script>
    }
}
