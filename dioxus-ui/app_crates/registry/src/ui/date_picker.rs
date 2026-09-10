use dioxus::prelude::*;
use time::Date;
use tw_merge::tw_merge;

#[component]
pub fn DatePicker(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!(
        "flex flex-col gap-4 p-3 rounded-lg border bg-card text-card-foreground shadow-sm w-fit",
        class.as_deref().unwrap_or("")
    );
    rsx! { div { "data-name": "DatePicker", class: "{merged_class}", {children} } }
}

#[component]
pub fn DatePickerNavButton(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] title: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "inline-flex items-center justify-center p-0 text-sm font-medium transition-colors bg-transparent border rounded-md opacity-50 whitespace-nowrap ring-offset-background focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50 disabled:cursor-not-allowed border-input hover:bg-accent hover:text-accent-foreground size-7 hover:opacity-100 [&_svg:not([class*='size-'])]:size-4",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        button {
            "data-name": "DatePickerNavButton",
            class: "{merged_class}",
            title: title.as_deref().unwrap_or(""),
            "aria-label": aria_label.as_deref().unwrap_or(""),
            onclick: move |e| onclick.call(e),
            {children}
        }
    }
}

#[component]
pub fn DatePickerTitle(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] role: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("text-sm font-medium text-center", class.as_deref().unwrap_or(""));
    rsx! {
        span {
            "data-name": "DatePickerTitle",
            class: "{merged_class}",
            role: role.as_deref().unwrap_or(""),
            {children}
        }
    }
}

#[component]
pub fn DatePickerHeader(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("grid grid-cols-[auto_1fr_auto] items-center pt-1", class.as_deref().unwrap_or(""));
    rsx! { header { "data-name": "DatePickerHeader", class: "{merged_class}", {children} } }
}

#[component]
pub fn DatePickerWeekDay(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let merged_class =
        tw_merge!("text-muted-foreground rounded-md w-9 font-normal text-[0.8rem]", class.as_deref().unwrap_or(""));
    rsx! {
        th {
            "data-name": "DatePickerWeekDay",
            class: "{merged_class}",
            "aria-label": aria_label.as_deref().unwrap_or(""),
            {children}
        }
    }
}

#[component]
pub fn DatePickerRow(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class = tw_merge!("flex w-full mt-2", class.as_deref().unwrap_or(""));
    rsx! { tr { "data-name": "DatePickerRow", class: "{merged_class}", {children} } }
}

#[component]
pub fn DatePickerCell(
    day: u8,
    year: i32,
    month: time::Month,
    disabled: bool,
    start_date: Signal<Date>,
    end_date: Signal<Date>,
    on_click: EventHandler<u8>,
    #[props(into, optional)] class: Option<String>,
) -> Element {
    let current_date = if day > 0 && !disabled { Date::from_calendar_date(year, month, day).ok() } else { None };

    let is_current = current_date.map(|d| d == *start_date.read() || d == *end_date.read()).unwrap_or(false);
    let is_selected = current_date.map(|d| d > *start_date.read() && d < *end_date.read()).unwrap_or(false);

    let cell_class = tw_merge!(
        "inline-flex items-center justify-center text-sm size-9 rounded-md select-none",
        "hover:cursor-pointer hover:bg-accent",
        "aria-disabled:pointer-events-none aria-disabled:opacity-50 aria-disabled:cursor-not-allowed",
        "aria-current:bg-primary aria-current:hover:bg-primary aria-current:text-primary-foreground",
        if is_selected { "bg-accent rounded-none" } else { "" },
        class.as_deref().unwrap_or("")
    );

    rsx! {
        td {
            "data-name": "DatePickerCell",
            class: "{cell_class}",
            "aria-current": if is_current { "true" } else { "false" },
            "aria-disabled": if disabled { "true" } else { "false" },
            onclick: move |_| {
                if !disabled {
                    on_click.call(day);
                }
            },
            "{day}"
        }
    }
}

#[component]
pub fn DatePickerMonth(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class =
        tw_merge!("flex flex-col items-center justify-start gap-2 size-full", class.as_deref().unwrap_or(""));
    rsx! { div { "data-name": "DatePickerMonth", class: "{merged_class}", {children} } }
}

#[component]
pub fn DatePickerTable(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] role: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!("w-full space-y-1 border-collapse", class.as_deref().unwrap_or(""));
    rsx! {
        table {
            "data-name": "DatePickerTable",
            class: "{merged_class}",
            role: role.as_deref().unwrap_or(""),
            {children}
        }
    }
}

#[component]
pub fn DatePickerWeekNumberHeader(
    #[props(into, optional)] class: Option<String>,
    #[props(into, optional)] aria_label: Option<String>,
    children: Element,
) -> Element {
    let merged_class = tw_merge!(
        "text-muted-foreground rounded-md w-6 font-normal text-[0.8rem] select-none",
        class.as_deref().unwrap_or("")
    );
    rsx! {
        th {
            "data-name": "DatePickerWeekNumberHeader",
            class: "{merged_class}",
            "aria-label": aria_label.as_deref().unwrap_or(""),
            {children}
        }
    }
}

#[component]
pub fn DatePickerWeekNumberCell(#[props(into, optional)] class: Option<String>, children: Element) -> Element {
    let merged_class =
        tw_merge!("w-6 text-center text-[0.8rem] text-muted-foreground select-none", class.as_deref().unwrap_or(""));
    rsx! { td { "data-name": "DatePickerWeekNumberCell", class: "{merged_class}", {children} } }
}
