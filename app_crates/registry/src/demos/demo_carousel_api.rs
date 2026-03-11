use leptos::prelude::*;

use crate::ui::carousel::{Carousel, CarouselContent, CarouselIndicator, CarouselItem, CarouselNext, CarouselPrevious};

#[component]
pub fn DemoCarouselApi() -> impl IntoView {
    view! {
        <div class="px-12 mx-auto w-full max-w-xs">
            <Carousel>
                <CarouselContent>
                    {(1..=5_u32)
                        .map(|i| {
                            view! {
                                <CarouselItem>
                                    <div class="flex justify-center items-center p-6 rounded-lg border aspect-square bg-card shadow-xs">
                                        <span class="text-4xl font-semibold text-foreground">{i}</span>
                                    </div>
                                </CarouselItem>
                            }
                        })
                        .collect::<Vec<_>>()}
                </CarouselContent>
                <CarouselPrevious />
                <CarouselNext />
                <CarouselIndicator />
            </Carousel>
        </div>
    }
}
