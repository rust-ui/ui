+++
title = "Date Picker"
description = "Rust/UI component that displays a date picker."
tags = []
is_new = false
image = "/images/thumbnails/date-picker.webp"
image_dark = "/images/thumbnails/date-picker-dark.webp"
+++



<StaticDatePicker />








## Installation

<StaticInstallDatePicker />

> **Required:** The `time` crate must enable the `wasm-bindgen` feature so that `OffsetDateTime::now_utc()` works in the browser. Without it, calling the current date at runtime will panic in WASM.
>
> ```toml
> # Cargo.toml
> time = { version = "0.3", features = ["wasm-bindgen"] }
> ```



## Components

The DatePicker component is composed of several subcomponents:

- **DatePicker**: Main wrapper component for the calendar
- **DatePickerHeader**: Header section with navigation and title
- **DatePickerNavButton**: Previous/next month navigation buttons
- **DatePickerTitle**: Current month and year display
- **DatePickerMonth**: Month container wrapper
- **DatePickerTable**: Calendar table structure
- **DatePickerRow**: Table row for weeks
- **DatePickerWeekDay**: Day of week header cell (`Mo`, `Tu`, etc.)
- **DatePickerWeekNumberHeader**: Column header for ISO week numbers (`#`)
- **DatePickerWeekNumberCell**: Cell displaying the ISO week number for each row
- **DatePickerCell**: Individual date cell with selection support



## Usage

```rust
use crate::components::ui::date_picker::{
    DatePicker,
    DatePickerCell,
    DatePickerHeader,
    DatePickerMonth,
    DatePickerNavButton,
    DatePickerRow,
    DatePickerTable,
    DatePickerTitle,
    DatePickerWeekDay,
};
```

```rust
<DatePicker>
    <DatePickerHeader>
        <DatePickerNavButton>"<"</DatePickerNavButton>
        <DatePickerTitle>"January 2025"</DatePickerTitle>
        <DatePickerNavButton>">"</DatePickerNavButton>
    </DatePickerHeader>
    <DatePickerMonth>
        <DatePickerTable>
            <thead>
                <DatePickerRow>
                    <DatePickerWeekDay>"Su"</DatePickerWeekDay>
                    <DatePickerWeekDay>"Mo"</DatePickerWeekDay>
                    <DatePickerWeekDay>"Tu"</DatePickerWeekDay>
                    <DatePickerWeekDay>"We"</DatePickerWeekDay>
                    <DatePickerWeekDay>"Th"</DatePickerWeekDay>
                    <DatePickerWeekDay>"Fr"</DatePickerWeekDay>
                    <DatePickerWeekDay>"Sa"</DatePickerWeekDay>
                </DatePickerRow>
            </thead>
            <tbody>
                <DatePickerRow>
                    <DatePickerCell day=1 year=2025 month=time::Month::January disabled=false start_date=start_date_signal end_date=end_date_signal on_click=|_| {} />
                    // ... more cells
                </DatePickerRow>
            </tbody>
        </DatePickerTable>
    </DatePickerMonth>
</DatePicker>
```



## Examples

### Week Numbers

Displays ISO week numbers in a leftmost column. Uses `DatePickerWeekNumberHeader` for the `#` column header and `DatePickerWeekNumberCell` per row, with `.iso_week()` from the `time` crate to compute each week's number.

<StaticDatePickerWeekNumbers />

### Date Picker Dual

[Popover](/docs/components/popover)-based date picker with dual calendar view for selecting date ranges and comparing periods. This example demonstrates how to build advanced date selection interfaces in Leptos with multiple calendar instances and range selection capabilities for Rust applications.

<StaticDatePickerDual />


### Dropdown Caption

Month and year dropdowns in the calendar header for fast navigation — useful when picking dates far from the current month (e.g., birthdates).

<StaticDatePickerDropdown />

### Date Picker Presets

Quick preset buttons for one-click date selection. Preset buttons navigate the calendar to the selected date's month automatically.

<StaticDatePickerPresets />

### With Time Picker

Combines a date calendar with start/end time inputs in the card footer — ideal for booking, scheduling, or any datetime selection.

<StaticDatePickerTime />

### Booked Dates

Shows unavailable dates as struck-through and non-selectable. Useful for hotel/rental booking flows where certain dates are already taken.

<StaticDatePickerBooked />

## See Also

- [Form](/docs/components/form)
- [Input](/docs/components/input)
- [Popover](/docs/components/popover)
