use leptos::prelude::*;

use crate::ui::accordion::{
    Accordion, AccordionContent, AccordionDescription, AccordionItem, AccordionTitle, AccordionTrigger,
};
use crate::ui::direction_provider::{Direction, DirectionProvider};

#[component]
pub fn DemoAccordionRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-[400px]">
            <Accordion class="max-w-[400px]">
                <AccordionItem>
                    <AccordionTrigger open=true>
                        <AccordionTitle>"البند الأول"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>
                        <AccordionTitle>"البند الثاني"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
                <AccordionItem>
                    <AccordionTrigger>
                        <AccordionTitle>"البند الثالث"</AccordionTitle>
                    </AccordionTrigger>
                    <AccordionContent class="pt-0">
                        <AccordionDescription>
                            "هذا هو وصف العنصر القابل للطي."
                        </AccordionDescription>
                    </AccordionContent>
                </AccordionItem>
            </Accordion>
        </DirectionProvider>
    }
}
