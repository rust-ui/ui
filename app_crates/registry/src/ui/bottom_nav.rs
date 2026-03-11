use leptos::prelude::*;
use leptos_ui::clx;

mod components {
    use super::*;
    clx! {BottomNav, nav, "z-50 mx-auto w-full max-w-lg border-t border-border bg-background pb-[env(safe-area-inset-bottom,0px)]" }
    clx! {BottomNavGrid, div, "grid grid-flow-col auto-cols-fr h-[var(--bottom__nav__height)] font-medium" }
    clx! {BottomNavLabel, span, "text-sm text-muted-foreground group-hover:text-primary group-aria-[current=page]:text-primary"}
    clx! {BottomNavButton, button,
        "inline-flex flex-col justify-center items-center px-5 group [&_svg]:mb-2 [&_svg]:text-muted-foreground hover:[&_svg]:text-primary aria-[current=page]:[&_svg]:text-primary active:scale-[0.98]",
        "touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]",
        // * SHORTFIX 🚑 iOS Safari: push icons closer to home indicator
        "supports-[-webkit-touch-callout:none]:justify-end supports-[-webkit-touch-callout:none]:pb-0 supports-[-webkit-touch-callout:none]:translate-y-1"
    }
}

pub use components::*;
