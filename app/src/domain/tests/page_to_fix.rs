use icons::{FileText, Lock, TriangleAlert, X};
use leptos::prelude::*;
use registry::ui::button::Button;
use registry::ui::dialog::{
    Dialog, DialogBody, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle,
    DialogTrigger,
};
use registry::ui::direction_provider::{Direction, DirectionProvider};
use registry::ui::input::Input;
use registry::ui::label::Label;
use registry::ui::textarea::Textarea;

use crate::domain::tests::drawer_tofix::{
    Drawer, DrawerBody, DrawerClose, DrawerContent, DrawerDescription, DrawerFooter, DrawerHandle, DrawerHeader,
    DrawerPosition, DrawerTitle, DrawerTrigger, DrawerVariant,
};

#[component]
pub fn PageToFix() -> impl IntoView {
    view! {
        <div data-vaul-drawer-wrapper="" class="min-h-full">
            <div class="flex flex-col gap-10 p-8 mx-auto w-full max-w-6xl">
                <header class="space-y-3">
                    <h1 class="text-3xl font-bold tracking-tight">"Drawer Parity Test Page"</h1>
                    <p class="max-w-3xl text-sm leading-6 text-muted-foreground">
                        "This page isolates the Rust drawer port parked in "
                        <code class="px-1.5 py-0.5 rounded bg-muted text-foreground">"domain/tests/drawer_tofix.rs"</code>
                        " so behavior can be matched against the original JS implementation before the public drawer is touched again."
                    </p>
                    <div class="flex gap-4 items-center text-sm">
                        <a
                            href="/test"
                            class="underline text-muted-foreground underline-offset-4 hover:text-foreground"
                        >
                            "→ Test"
                        </a>
                        <a
                            href="/docs/components/drawer"
                            class="underline text-muted-foreground underline-offset-4 hover:text-foreground"
                        >
                            "→ Public Drawer Docs"
                        </a>
                    </div>
                </header>

                <div class="grid gap-6 md:grid-cols-2 xl:grid-cols-3">
                    <DrawerDemoCard title="Default Bottom Drawer" description="Baseline open, close, overlay, handle, and focus behavior.">
                        <DemoDrawerToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Dialog Switcher" description="Matches the mobile drawer / desktop dialog split demo.">
                        <DemoDrawerDialogToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Family Sheet" description="Visual card-stack drawer with corrected readable action text.">
                        <DemoDrawerFamilyToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Focus Trap" description="Tab, Shift+Tab, and Escape coverage with real inputs.">
                        <DemoDrawerFocusToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Non-dismissable" description="Overlay click and drag should not close when dismissible is false.">
                        <DemoDrawerNonDismissableToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Scrollable Bottom Drawer" description="Verifies scroll-conflict handling inside vertical content.">
                        <DemoDrawerScrollableToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Right Side Drawer" description="Right-position drawer behavior and side transforms.">
                        <DemoDrawerSideToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Left Side Scrollable" description="Scrollable side drawer for left-position drag logic.">
                        <DemoDrawerSideScrollableToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="Floating Right Drawer" description="Floating variant overlay behavior and offset transform.">
                        <DemoDrawerSideFloatingToFix />
                    </DrawerDemoCard>

                    <DrawerDemoCard title="RTL Drawer" description="Direction-provider surface for focus and trigger text in RTL.">
                        <DemoDrawerRtlToFix />
                    </DrawerDemoCard>
                </div>
            </div>
        </div>
    }
}

#[component]
fn DrawerDemoCard(title: &'static str, description: &'static str, children: Children) -> impl IntoView {
    view! {
        <section class="p-6 rounded-2xl border shadow-sm bg-card border-border">
            <div class="mb-5 space-y-2">
                <h2 class="text-lg font-semibold">{title}</h2>
                <p class="text-sm leading-6 text-muted-foreground">{description}</p>
            </div>
            <div class="flex items-start">{children()}</div>
        </section>
    }
}

#[component]
fn DemoDrawerToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />
                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Drawer Title"</DrawerTitle>
                        <DrawerDescription>"Drag down to close or click outside."</DrawerDescription>
                    </DrawerHeader>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerDialogToFix() -> impl IntoView {
    view! {
        <div class="w-full">
            <div class="md:hidden">
                <Drawer>
                    <DrawerTrigger>"Subscribe"</DrawerTrigger>
                    <DrawerContent>
                        <DrawerHandle />
                        <DrawerBody>
                            <DrawerHeader>
                                <DrawerTitle>"Subscribe"</DrawerTitle>
                                <DrawerDescription>"Get the latest updates delivered to your inbox."</DrawerDescription>
                            </DrawerHeader>
                            <Input attr:r#type="email" attr:placeholder="you@example.com" />
                            <DrawerFooter>
                                <DrawerClose class="w-full sm:w-fit">"Cancel"</DrawerClose>
                                <Button class="w-full sm:w-fit">"Subscribe"</Button>
                            </DrawerFooter>
                        </DrawerBody>
                    </DrawerContent>
                </Drawer>
            </div>

            <div class="hidden md:block">
                <Dialog>
                    <DialogTrigger>"Subscribe"</DialogTrigger>
                    <DialogContent class="sm:max-w-[400px]">
                        <DialogBody>
                            <DialogHeader>
                                <DialogTitle>"Subscribe"</DialogTitle>
                                <DialogDescription>"Get the latest updates delivered to your inbox."</DialogDescription>
                            </DialogHeader>
                            <Input attr:r#type="email" attr:placeholder="you@example.com" />
                            <DialogFooter>
                                <DialogClose class="w-full sm:w-fit">"Cancel"</DialogClose>
                                <Button class="w-full sm:w-fit">"Subscribe"</Button>
                            </DialogFooter>
                        </DialogBody>
                    </DialogContent>
                </Dialog>
            </div>
        </div>
    }
}

#[component]
fn DemoDrawerFamilyToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent class="overflow-hidden right-4 left-4 pb-6 mx-auto mb-4 max-w-[361px] rounded-[36px] bg-white">
                <header class="flex justify-between items-center mb-4 border-b h-[72px] border-neutral-200 px-1">
                    <h2 class="text-lg font-semibold text-neutral-950">"Options"</h2>
                    <button
                        data-name="DrawerClose"
                        class="flex justify-center items-center rounded-full transition-colors size-8 bg-neutral-100 text-neutral-700 hover:bg-neutral-200"
                    >
                        <X class="size-3" />
                    </button>
                </header>

                <div class="space-y-3">
                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-neutral-900 hover:bg-neutral-200">
                        <Lock class="size-[18px] shrink-0 text-neutral-700" />
                        <span class="text-neutral-900">"View Private Key"</span>
                    </button>

                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-neutral-100 text-neutral-900 hover:bg-neutral-200">
                        <FileText class="size-[18px] shrink-0 text-neutral-700" />
                        <span class="text-neutral-900">"View Recovery Phrase"</span>
                    </button>

                    <button class="flex gap-4 items-center px-4 w-full h-12 text-base font-medium rounded-2xl transition-colors bg-red-50 text-red-600 hover:bg-red-100">
                        <TriangleAlert class="size-[18px] shrink-0 text-red-500" />
                        <span class="text-red-600">"Remove Wallet"</span>
                    </button>
                </div>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerFocusToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Focus Drawer"</DrawerTitle>
                        <DrawerDescription>
                            "Test keyboard navigation: Press Tab to cycle through elements, Shift+Tab to go back, and Escape to close."
                        </DrawerDescription>
                    </DrawerHeader>

                    <form class="flex flex-col gap-4">
                        <div class="flex flex-col gap-2">
                            <Label html_for="tofix-text-input">"Text Input"</Label>
                            <Input attr:id="tofix-text-input" attr:placeholder="Type something..." />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="tofix-email">"Email"</Label>
                            <Input attr:id="tofix-email" attr:r#type="email" attr:placeholder="email@example.com" />
                        </div>

                        <div class="flex flex-col gap-2">
                            <Label html_for="tofix-textarea">"Message"</Label>
                            <Textarea attr:id="tofix-textarea" attr:rows="4" attr:placeholder="Write a message..." />
                        </div>
                    </form>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerNonDismissableToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent style="--initial-transform: 100%; pointer-events: auto;" dismissible=false>
                <DrawerHandle />

                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Are you absolutely sure?"</DrawerTitle>
                        <DrawerDescription>
                            "This action cannot be undone. This will permanently delete your account and remove your data from our servers."
                        </DrawerDescription>
                    </DrawerHeader>

                    <DrawerClose>"Confirm"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerScrollableToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent>
                <DrawerHandle />

                <DrawerBody class="overflow-y-auto max-h-[300px]">
                    <DrawerHeader>
                        <DrawerTitle>"Ira Glass on Taste"</DrawerTitle>
                    </DrawerHeader>
                    <DrawerDescription>
                        "Nobody tells this to people who are beginners, I wish someone told me. All of us who do creative work, we get into it because we have good taste."
                    </DrawerDescription>
                    <DrawerDescription>
                        "But there is this gap. For the first couple years you make stuff, it's just not that good. It's trying to be good, it has potential, but it's not. But your taste, the thing that got you into the game, is still killer. And your taste is why your work disappoints you. A lot of people never get past this phase, they quit."
                    </DrawerDescription>
                    <DrawerDescription>
                        "Most people I know who do interesting, creative work went through years of this. We know our work doesn't have this special thing that we want it to have. We all go through this. And if you are just starting out or you are still in this phase, you gotta know its normal and the most important thing you can do is do a lot of work."
                    </DrawerDescription>
                    <DrawerDescription>
                        "Put yourself on a deadline so that every week you will finish one story. It is only by going through a volume of work that you will close that gap, and your work will be as good as your ambitions. And I took longer to figure out how to do this than anyone I've ever met. It's gonna take awhile. It's normal to take awhile. You've just gotta fight your way through."
                    </DrawerDescription>

                    <DrawerClose>"Close"</DrawerClose>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerSideToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Right
                class="top-0 bottom-0 left-auto h-full max-h-full rounded-t-none w-[300px] rounded-l-[10px]"
            >
                <DrawerBody>
                    <DrawerHeader>
                        <DrawerTitle>"Different Directions"</DrawerTitle>
                        <DrawerDescription>"It supports all directions."</DrawerDescription>
                        <DrawerDescription>
                            "This one specifically is not touching the edge of the screen, but that is not required for a side drawer."
                        </DrawerDescription>
                    </DrawerHeader>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerSideScrollableToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Left
                class="top-0 bottom-0 right-auto h-full max-h-full rounded-t-none w-[300px] rounded-r-[10px]"
            >
                <DrawerBody class="overflow-y-auto pr-4 h-full">
                    <DrawerHeader>
                        <DrawerTitle>"Scrollable Side Drawer"</DrawerTitle>
                        <DrawerDescription>
                            "This drawer contains 50 scrollable items to demonstrate side scrolling behavior."
                        </DrawerDescription>
                    </DrawerHeader>

                    <div class="mt-4 space-y-2">
                        {(1..=50)
                            .map(|i| {
                                view! {
                                    <div class="p-3 rounded-md border bg-muted border-border">
                                        <p class="text-sm font-medium">"Item " {i}</p>
                                        <p class="text-xs text-muted-foreground">
                                            "This is item number " {i } " in the list"
                                        </p>
                                    </div>
                                }
                            })
                            .collect_view()}
                    </div>
                </DrawerBody>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerSideFloatingToFix() -> impl IntoView {
    view! {
        <Drawer>
            <DrawerTrigger>"Open Drawer"</DrawerTrigger>

            <DrawerContent
                position=DrawerPosition::Right
                variant=DrawerVariant::Floating
                class="top-2 right-2 bottom-2 left-auto max-h-full outline-none w-[300px] rounded-[16px] rounded-t-[16px]"
                style="--initial-transform: calc(100% + 8px);"
            >
                <DrawerHeader class="mt-4">
                    <DrawerTitle>"Different Directions"</DrawerTitle>
                    <DrawerDescription>"It supports all directions."</DrawerDescription>
                    <DrawerDescription>
                        "This one specifically is not touching the edge of the screen, but that is not required for a side drawer."
                    </DrawerDescription>
                </DrawerHeader>
            </DrawerContent>
        </Drawer>
    }
}

#[component]
fn DemoDrawerRtlToFix() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-sm">
            <Drawer>
                <DrawerTrigger>"فتح الدرج"</DrawerTrigger>

                <DrawerContent>
                    <DrawerHandle />
                    <DrawerBody>
                        <DrawerHeader>
                            <DrawerTitle>"عنوان الدرج"</DrawerTitle>
                            <DrawerDescription>"اسحب للأسفل للإغلاق أو انقر خارج الدرج."</DrawerDescription>
                        </DrawerHeader>

                        <DrawerClose>"إغلاق"</DrawerClose>
                    </DrawerBody>
                </DrawerContent>
            </Drawer>
        </DirectionProvider>
    }
}
