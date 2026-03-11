use leptos::prelude::*;
use leptos_ui::{clx, void};

mod components {
    use super::*;
    clx! {Resizable, div, "border rounded-xl flex flex-row md:touch-none w-full"}
    clx! {ResizableContainer, div, "flex items-center justify-center flex-[1_1_auto] min-w-[150px] bg-transparent h-full"}
    void! {ResizableBackground, div, "flex-[0_0_auto] w-[0px] bg-muted"}
    void! {ResizableHandle, div, "hidden md:flex relative justify-center items-center p-0 w-3 -mr-2 translate-x-2 bg-transparent focus-visible:ring-1 focus-visible:ring-offset-1 focus-visible:outline-none after:inset-y-0 after:left-1/2 after:absolute after:right-0 after:top-1/2 after:h-8 after:w-[6px] after:-translate-y-1/2 after:translate-x-[-3px] after:rounded-full after:bg-neutral-200 dark:after:bg-neutral-600 after:transition-all after:hover:h-10 cursor-col-resize focus-visible:ring-ring transition-transform duration-200 select-none touch-none z-10"}
}

pub use components::*;
