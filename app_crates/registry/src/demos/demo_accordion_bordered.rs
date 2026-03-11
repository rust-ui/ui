use leptos::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordionBordered() -> impl IntoView {
    view! {
        <Accordion class="overflow-hidden rounded-lg border bg-background max-w-[400px]">
            <AccordionItem>
                <AccordionTrigger open=true class="peer-checked:bg-accent hover:bg-accent">
                    <AccordionTitle>Accordion item 01</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-2">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger class="peer-checked:bg-accent hover:bg-accent">
                    <AccordionTitle>Accordion item 02</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-2">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger class="peer-checked:bg-accent hover:bg-accent">
                    <AccordionTitle>Accordion item 03</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-2">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
