use leptos::prelude::*;

use crate::ui::button::Button;

#[component]
pub fn DemoButtonStateful() -> impl IntoView {
    view! {
        <style>
            {r#"
            @media (prefers-reduced-motion: no-preference) {
            ::view-transition-old(button-stateful),
            ::view-transition-new(button-stateful) {
            height: 100%;
            width: 100%;
            }
            }
            "#}
        </style>

        <Button attr:id="ButtonStateful" attr:style="view-transition-name: button-stateful;">
            Do some hard work
        </Button>

        <script>
            {r#"
            (() => {
            const STATES = {
            idle: "Do some hard work",
            working: "⏳ working...",
            done: "Done! ✅",
            };
            
            ButtonStateful.onclick = () => {
            setState("working");
            setTimeout(() => setState("done"), 2000);
            setTimeout(() => setState("idle"), 4000);
            };
            
            function setState(state) {
            if (!document.startViewTransition) ButtonStateful.innerHTML = STATES[state];
            else document.startViewTransition(() => (ButtonStateful.innerHTML = STATES[state]));
            }
            })();
            "#}
        </script>
    }
}
