use leptos::prelude::*;

use crate::ui::image::Image;
use crate::ui::marquee::{Marquee, MarqueeRow, MarqueeWrapper};

#[component]
pub fn DemoMarquee() -> impl IntoView {
    view! {
        <MarqueeWrapper class="max-w-4xl">
            <Marquee>
                <MarqueeRow>
                    <CardFigureExample blockquote="1. Just love it".to_string() />
                    <CardFigureExample blockquote="2. Just love it".to_string() />
                    <CardFigureExample blockquote="3. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow>
                    <CardFigureExample blockquote="4. Just love it".to_string() />
                    <CardFigureExample blockquote="5. Just love it".to_string() />
                    <CardFigureExample blockquote="6. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow>
                    <CardFigureExample blockquote="7. Just love it".to_string() />
                    <CardFigureExample blockquote="8. Just love it".to_string() />
                    <CardFigureExample blockquote="9. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow>
                    <CardFigureExample blockquote="10. Just love it".to_string() />
                    <CardFigureExample blockquote="11. Just love it".to_string() />
                    <CardFigureExample blockquote="12. Just love it".to_string() />
                </MarqueeRow>
            </Marquee>

            // --------------------------------------
            // --------------------------------------
            // --------------------------------------
            // --------------------------------------
            // --------------------------------------
            // --------------------------------------

            <Marquee>
                <MarqueeRow class="[animation-direction:reverse]">
                    <CardFigureExample blockquote="[II] 1. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 2. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 3. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow class="[animation-direction:reverse]">
                    <CardFigureExample blockquote="[II] 4. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 5. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 6. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow class="[animation-direction:reverse]">
                    <CardFigureExample blockquote="[II] 7. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 8. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 9. Just love it".to_string() />
                </MarqueeRow>
                <MarqueeRow class="[animation-direction:reverse]">
                    <CardFigureExample blockquote="[II] 10. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 11. Just love it".to_string() />
                    <CardFigureExample blockquote="[II] 12. Just love it".to_string() />
                </MarqueeRow>
            </Marquee>

        </MarqueeWrapper>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
fn CardFigureExample(blockquote: String) -> impl IntoView {
    view! {
        <figure class="overflow-hidden relative p-4 w-64 rounded-xl border cursor-pointer border-gray-950/[.1] bg-gray-950/[.01] dark:border-gray-50/[.1] dark:bg-gray-50/[.10] dark:hover:bg-gray-50/[.15] hover:bg-gray-950/[.05]">
            <div class="flex gap-2 items-center">
                <Image src="https://avatar.vercel.sh/jack" alt="" width=32 height=32 class="rounded-full" />
                <div class="flex flex-col">
                    <figcaption class="text-sm font-medium dark:text-white">Jack</figcaption>
                    <p class="text-xs font-medium dark:text-white/40">@jack</p>
                </div>
            </div>
            <blockquote class="mt-2 text-sm">{blockquote}</blockquote>
        </figure>
    }
}
