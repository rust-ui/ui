use leptos::prelude::*;
use leptos_ui::variants;

// TODO 💪 Loading state (demo_use_timeout_fn.rs and demo_button.rs)

variants! {
    Button {
        base: "inline-flex items-center justify-center gap-2 whitespace-nowrap rounded-md text-sm font-medium transition-all disabled:pointer-events-none disabled:opacity-50 [&_svg]:pointer-events-none [&_svg:not([class*='size-'])]:size-4 shrink-0 [&_svg]:shrink-0 outline-none focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-[3px] aria-invalid:ring-destructive/20 dark:aria-invalid:ring-destructive/40 aria-invalid:border-destructive  w-fit  hover:cursor-pointer active:scale-[0.98] active:opacity-100 touch-manipulation [-webkit-tap-highlight-color:transparent] select-none [-webkit-touch-callout:none]", // Using hover:cursor-pointer as workaround for href_support.
        variants: {
            variant: {
                Default: "bg-primary text-primary-foreground shadow-xs hover:bg-primary/90",
                Destructive: "bg-destructive text-white shadow-xs hover:bg-destructive/90 focus-visible:ring-destructive/20 dark:focus-visible:ring-destructive/40 dark:bg-destructive/60",
                Outline: "border bg-background shadow-xs hover:bg-accent hover:text-accent-foreground dark:bg-input/30 dark:border-input dark:hover:bg-input/5",
                Secondary: "bg-secondary text-secondary-foreground shadow-xs hover:bg-secondary/80",
                Ghost: "hover:bg-accent hover:text-accent-foreground dark:hover:bg-accent/50",
                Accent: "bg-accent text-accent-foreground hover:bg-accent/80",
                Link: "text-primary underline-offset-4 hover:underline",
                //
                Warning: "bg-warning text-warning-foreground hover:bg-warning/90",
                Success: "bg-success text-success-foreground hover:bg-success/90",
                Bordered: "bg-transparent border border-zinc-200 text-muted-foreground",
            },
            size: {
                Default: "h-9 px-4 py-2 has-[>svg]:px-3",
                Sm: "h-8 rounded-md gap-1.5 px-3 has-[>svg]:px-2.5",
                Lg: "h-10 rounded-md px-6 has-[>svg]:px-4",
                Icon: "size-9",
                //
                Mobile: "px-6 py-3 rounded-[24px]",
                Badge: "px-2.5 py-0.5 text-xs"
            }
        },
        component: {
            element: button,
            support_href: true,
            support_aria_current: true
        }
    }
}
