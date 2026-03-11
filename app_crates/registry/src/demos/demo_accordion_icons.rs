use icons::{AlignHorizontalSpaceAround, Blocks, Compass, LogIn, PanelLeft, Search};
use leptos::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionItem, AccordionLink, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordionIcons() -> impl IntoView {
    view! {
        <Accordion class="overflow-hidden w-full rounded-lg border bg-background max-w-[500px]">
            <AccordionItem>
                <AccordionTrigger open=true class="peer-checked:bg-accent hover:bg-accent">
                    <AccordionTitle>Registry</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="p-0">
                    <ul class="text-sm">
                        <li>
                            <AccordionLink attr:href="/docs/components">
                                <Blocks />
                                <span>Components</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/docs/hooks">
                                <Compass />
                                <span>Hooks</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/icons">
                                <Search />
                                <span>Icons</span>
                            </AccordionLink>
                        </li>
                    </ul>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger class="peer-checked:bg-accent hover:bg-accent">
                    <AccordionTitle>Blocks</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="p-0">
                    <ul class="text-sm">
                        <li>
                            <AccordionLink attr:href="/blocks/login">
                                <LogIn />
                                <span>Login</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/blocks/sidenav">
                                <PanelLeft />
                                <span>Sidenav</span>
                            </AccordionLink>
                        </li>
                        <li>
                            <AccordionLink attr:href="/blocks/parallax">
                                <AlignHorizontalSpaceAround />
                                <span>Parallax</span>
                            </AccordionLink>
                        </li>
                    </ul>
                </AccordionContent>
            </AccordionItem>
            <AccordionLink class="p-3" attr:href="/charts">
                <AccordionTitle>Charts</AccordionTitle>
            </AccordionLink>
            <AccordionLink class="p-3" attr:href="/icons">
                <AccordionTitle>Icons</AccordionTitle>
            </AccordionLink>
        </Accordion>
    }
}
