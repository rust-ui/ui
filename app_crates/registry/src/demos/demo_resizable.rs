use leptos::prelude::*;

#[component]
pub fn DemoResizable() -> impl IntoView {
    view! {
        <style>
            {".direction__rtl {
            direction: rtl;
            }
            
            #behind-scene-toggle:checked ~ .wrapper .resizer {
            opacity: 1;
            }
            "}
        </style>

        <div>
            <input type="checkbox" id="behind-scene-toggle" class="hidden" />
            <div class="flex h-screen">
                <div class="overflow-y-scroll flex-1 p-2.5 h-screen">

                    <h1 class="text-3xl font-bold lg:text-4xl text-pretty">CSS-only resizable panels</h1>
                    <p>
                        This is an example of a CSS-only resizeable panels solution. It uses a combination of the flexbox model, CSS
                        <code>resize</code>property, and <code>position: absolute</code>for the resized panel contents.
                    </p>
                    <h2 class="text-2xl font-bold lg:text-3xl text-pretty">No JavaScript!</h2>
                    <p>This is a pure CSS solution!</p>
                    <h2 class="text-2xl font-bold lg:text-3xl text-pretty">Browser support</h2>
                    <p>
                        <a href="https://caniuse.com/css-resize">All browsers supporting <code>resize</code></a>
                    </p>
                </div>
                <div class="relative bg-red-200 aside min-w-[300px]">
                    <div class="flex overflow-y-scroll absolute top-0 bottom-0 right-2.5 left-2.5 flex-col m-0 aside-inner z-2">

                        <h1 class="text-3xl font-bold lg:text-4xl text-pretty">Side Panel</h1>
                        <p>
                            Drag left/right the <span class="resize-icon">"⇆"</span>
                            handle in order to resize this panel!
                        </p>
                        <label class="cursor-pointer" for="behind-scene-toggle">
                            Click here to see behind the scene!
                        </label>
                    </div>
                    <div class="inline-block absolute top-1/2 p-0.5 m-0 h-3 leading-3 text-white bg-black rounded-full resize-icon margin-left-[-7px]">
                        "⇆"
                    </div>
                    <div class="overflow-hidden relative top-1/2 w-full h-3 bg-red-500 opacity-0 resize-x margin-left-[-7px] direction__rtl resizer cursor-ew-resize"></div>
                </div>
            </div>
        </div>
    }
}
