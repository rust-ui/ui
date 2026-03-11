use leptos::prelude::*;

use crate::ui::carousel::{
    Carousel, CarouselContent, CarouselItem, CarouselNext, CarouselOrientation, CarouselPrevious,
};

#[component]
pub fn DemoCarouselOrientation() -> impl IntoView {
    view! {
        <div class="py-12 mx-auto w-full max-w-xs">
            <Carousel orientation=CarouselOrientation::Vertical>
                <CarouselContent class="h-[200px]">
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem class="md:basis-1/2">
                                    <div class="p-1">
                                        <div class="flex justify-center items-center p-6 rounded-lg border bg-card shadow-xs">
                                            <span class="text-3xl font-semibold text-foreground">{i}</span>
                                        </div>
                                    </div>
                                </CarouselItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
            </Carousel>
        </div>
    }
}
