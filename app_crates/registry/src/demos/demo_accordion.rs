use leptos::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};

#[component]
pub fn DemoAccordion() -> impl IntoView {
    view! {
        <Accordion class="max-w-[400px]">
            <AccordionItem>
                <AccordionTrigger open=true>
                    <AccordionTitle>Accordion item 01</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>
                    <AccordionTitle>Accordion item 02</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
            <AccordionItem>
                <AccordionTrigger>
                    <AccordionTitle>Accordion item 03</AccordionTitle>
                </AccordionTrigger>
                <AccordionContent class="pt-0">
                    <AccordionDescription>"This is the Accordion description"</AccordionDescription>
                </AccordionContent>
            </AccordionItem>
        </Accordion>
    }
}
