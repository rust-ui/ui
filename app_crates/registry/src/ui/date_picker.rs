use leptos::prelude::*;
use leptos_ui::clx;
use time::Date;
use tw_merge::*;

// TODO UI.
// use icons::{chevron_left::ChevronLeft, chevron_right::ChevronRight};

mod components {
    use super::*;
    clx! {DatePicker, div, "flex flex-col gap-4 p-3 rounded-lg border bg-card text-card-foreground shadow-sm w-fit"}
    clx! {DatePickerNavButton, button, "inline-flex items-center justify-center p-0 text-sm font-medium transition-colors bg-transparent border rounded-md opacity-50 whitespace-nowrap ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed border-input hover:bg-accent hover:text-accent-foreground size-7 hover:opacity-100  [&_svg:not([class*='size-'])]:size-4"}
    clx! {DatePickerTitle, span, "text-sm font-medium text-center"}
    clx! {DatePickerHeader, header, "grid grid-cols-[auto_1fr_auto] items-center pt-1"}
    clx! {DatePickerWeekDay, th, "text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]"}
    clx! {DatePickerWeekNumberHeader, th, "text-muted-foreground rounded-md w-6 font-normal text-[0.8rem] select-none"}
    clx! {DatePickerWeekNumberCell, td, "w-6 text-center text-[0.8rem] text-muted-foreground select-none"}
    clx! {DatePickerRow, tr, "flex w-full mt-2"}
    clx! {DatePickerMonth, div, "flex flex-col items-center justify-start gap-2 size-full"}
    clx! {DatePickerTable, table, "w-full space-y-1 border-collapse"}
}

pub use components::*;

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DatePickerCell(
    day: u8,
    year: i32,
    month: time::Month,
    disabled: bool,
    start_date: RwSignal<Date>,
    end_date: RwSignal<Date>,
    on_click: impl Fn(u8) + 'static,
    #[prop(into, optional)] class: String,
) -> impl IntoView {
    let current_date = if day > 0 && !disabled { Date::from_calendar_date(year, month, day).ok() } else { None };

    let is_current = move || {
        if let Some(date) = current_date { date == start_date.get() || date == end_date.get() } else { false }
    };

    let is_selected = move || {
        if let Some(date) = current_date { date > start_date.get() && date < end_date.get() } else { false }
    };

    let merged_class = tw_merge!(
        "inline-flex items-center justify-center text-sm size-9 rounded-md select-none",
        "hover:cursor-pointer hover:bg-accent",
        "aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-disabled:cursor-not-allowed",
        "aria-current:bg-primary aria-current:hover:bg-primary aria-current:text-primary-foreground",
        class
    );

    let cell_class = move || {
        let base = merged_class.clone();
        if is_selected() { format!("{} bg-accent rounded-none", base) } else { base }
    };

    let handle_click = move |_| {
        if !disabled {
            on_click(day);
        }
    };

    view! {
        <td
            data-name="DatePickerCell"
            class=cell_class
            aria-current=move || is_current().to_string()
            aria-disabled=move || disabled.to_string()
            on:click=handle_click
        >
            {day}
        </td>
    }
}
