use std::collections::{HashMap, HashSet};

use dioxus::prelude::*;
use icons::{ArrowUpDown, Ellipsis, ListFilter, Plus, Trash, X};
use strum::{AsRefStr, Display, EnumString};

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::button::{Button, ButtonSize, ButtonVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::data_table::{
    DataTable, DataTableBody, DataTableCell, DataTableHead, DataTableHeader, DataTableRow, DataTableWrapper,
};
use crate::ui::dropdown_menu::{
    DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuItem, DropdownMenuLabel, DropdownMenuTrigger,
};
use crate::ui::input::Input;
use crate::ui::multi_select::{
    MultiSelect, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption, MultiSelectTrigger,
    MultiSelectValue,
};
use crate::ui::popover::{Popover, PopoverAlign, PopoverContent, PopoverTrigger};
use crate::ui::separator::{Separator, SeparatorOrientation};

const ALL_STATUSES: [PaymentStatus; 4] =
    [PaymentStatus::Processing, PaymentStatus::Pending, PaymentStatus::Success, PaymentStatus::Failed];

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum SortOrder {
    #[default]
    Asc,
    Desc,
}

const COLUMNS: [&str; 3] = ["Status", "Email", "Amount"];

#[component]
pub fn DemoDataTableFilters() -> Element {
    let mut selected_ids_signal = use_signal(HashSet::<usize>::new);
    let columns_signal = use_signal(|| HashSet::from(COLUMNS.map(|c| c.to_string())));
    let mut sort_order_signal = use_signal(SortOrder::default);
    let mut email_filter_signal = use_signal(String::new);
    let mut status_filter_signal = use_signal(HashSet::<PaymentStatus>::new);
    let mut deleted_ids_signal = use_signal(HashSet::<usize>::new);
    let mut fake_payment_count_signal = use_signal(|| 0_usize);

    // Base data: all non-deleted payments (incl. dynamically added)
    let base_payments_signal = use_memo(move || {
        let count = fake_payment_count_signal();
        let mut payments: Vec<Payment> = INITIAL_PAYMENTS.to_vec();
        for i in 0..count {
            payments.push(Payment::new(1000 + i));
        }
        payments.retain(|p| !deleted_ids_signal.with(|d| d.contains(&p.id)));
        payments
    });

    // Facet counts: base data with email filter applied, but NOT status filter
    let status_facets_signal = use_memo(move || {
        let filter = email_filter_signal().to_lowercase();
        let mut counts = HashMap::<PaymentStatus, usize>::new();
        base_payments_signal.with(|payments| {
            for p in payments {
                if p.email.to_lowercase().contains(&filter) {
                    *counts.entry(p.status).or_insert(0) += 1;
                }
            }
        });
        counts
    });

    // Fully filtered + sorted result
    let filtered_payments_signal = use_memo(move || {
        let email_filter = email_filter_signal().to_lowercase();
        let active_statuses = status_filter_signal();
        let mut payments = base_payments_signal();
        payments.retain(|p| {
            let email_ok = p.email.to_lowercase().contains(&email_filter);
            let status_ok = active_statuses.is_empty() || active_statuses.contains(&p.status);
            email_ok && status_ok
        });
        match sort_order_signal() {
            SortOrder::Asc => payments.sort_by(|a, b| a.email.cmp(b.email)),
            SortOrder::Desc => payments.sort_by(|a, b| b.email.cmp(a.email)),
        }
        payments
    });

    let has_active_filters =
        use_memo(move || !status_filter_signal.with(|s| s.is_empty()) || !email_filter_signal.with(|s| s.is_empty()));

    let active_status_count = use_memo(move || status_filter_signal.with(|s| s.len()));

    let selected_count_signal = use_memo(move || {
        filtered_payments_signal.with(|payments| {
            selected_ids_signal.with(|selected| payments.iter().filter(|p| selected.contains(&p.id)).count())
        })
    });

    let handle_select_all = move |checked: bool| {
        selected_ids_signal.with_mut(|selected| {
            filtered_payments_signal.with(|payments| {
                for p in payments {
                    if checked {
                        selected.insert(p.id);
                    } else {
                        selected.remove(&p.id);
                    }
                }
            });
        });
    };

    rsx! {
        div { class: "py-8 w-full",
            // Toolbar
            div { class: "flex flex-wrap gap-2 items-center mb-4",
                Input {
                    class: "max-w-sm",
                    placeholder: "Filter emails...",
                    value: email_filter_signal(),
                    oninput: move |ev: Event<FormData>| email_filter_signal.set(ev.value()),
                }

                // Status faceted filter
                Popover { align: PopoverAlign::Start,
                    PopoverTrigger { class: "gap-1.5 px-3 h-8 text-xs",
                        ListFilter { class: "size-3.5" }
                        "Status"
                        if active_status_count() > 0 {
                            Separator { orientation: SeparatorOrientation::Vertical, class: "mx-0.5 h-4" }
                            Badge { variant: BadgeVariant::Secondary, class: "px-1 font-normal rounded-sm",
                                {active_status_count().to_string()}
                            }
                        }
                    }
                    PopoverContent { class: "p-0 min-h-0 w-[180px]",
                        div { class: "p-1",
                            {ALL_STATUSES.iter().map(|&status| {
                                let is_checked = status_filter_signal.with(|s| s.contains(&status));
                                let count = status_facets_signal.with(|f| f.get(&status).copied().unwrap_or(0));
                                rsx! {
                                    div {
                                        class: "flex gap-2 items-center py-1.5 px-2 text-sm rounded-sm cursor-pointer select-none hover:bg-accent",
                                        onclick: move |_| {
                                            status_filter_signal.with_mut(|s| {
                                                if s.contains(&status) {
                                                    s.remove(&status);
                                                } else {
                                                    s.insert(status);
                                                }
                                            });
                                        },
                                        Checkbox { checked: is_checked, aria_label: status.to_string() }
                                        span { class: "flex-1", {status.to_string()} }
                                        if count > 0 {
                                            Badge {
                                                variant: BadgeVariant::Secondary,
                                                class: "ml-auto font-mono font-normal rounded-sm",
                                                {count.to_string()}
                                            }
                                        }
                                    }
                                }
                            })}
                        }
                        if !status_filter_signal.with(|s| s.is_empty()) {
                            Separator {}
                            div { class: "p-1",
                                Button {
                                    variant: ButtonVariant::Ghost,
                                    class: "justify-center w-full h-8 text-xs",
                                    onclick: move |_| status_filter_signal.set(HashSet::new()),
                                    "Clear filters"
                                }
                            }
                        }
                    }
                }

                // Reset all active filters
                if has_active_filters() {
                    Button {
                        variant: ButtonVariant::Ghost,
                        class: "px-2 h-8 text-xs",
                        onclick: move |_| {
                            email_filter_signal.set(String::new());
                            status_filter_signal.set(HashSet::new());
                        },
                        "Reset"
                        X { class: "size-3.5" }
                    }
                }

                MultiSelect { values: columns_signal,
                    MultiSelectTrigger { class: "ml-auto w-[150px]",
                        MultiSelectValue { placeholder: "Columns" }
                    }
                    MultiSelectContent {
                        MultiSelectGroup {
                            {COLUMNS.into_iter().map(|column| {
                                let is_email = column == "Email";
                                rsx! {
                                    MultiSelectItem {
                                        MultiSelectOption { value: column, disabled: is_email,
                                            {column}
                                        }
                                    }
                                }
                            })}
                        }
                    }
                }

                Button {
                    size: ButtonSize::Icon,
                    variant: ButtonVariant::Outline,
                    onclick: move |_| {
                        fake_payment_count_signal.with_mut(|count| *count += 1);
                    },
                    Plus { class: "text-muted-foreground" }
                }
            }

            // Table
            DataTableWrapper {
                DataTable {
                    DataTableHeader {
                        DataTableRow {
                            DataTableHead { class: "px-4",
                                Checkbox {
                                    aria_label: "Select all",
                                    checked: filtered_payments_signal.with(|payments| {
                                        !payments.is_empty() && selected_count_signal() == payments.len()
                                    }),
                                    on_checked_change: handle_select_all,
                                }
                            }
                            if columns_signal.with(|c| c.contains("Status")) {
                                DataTableHead { class: "px-4", "Status" }
                            }
                            if columns_signal.with(|c| c.contains("Email")) {
                                DataTableHead { class: "px-4",
                                    Button {
                                        variant: ButtonVariant::Ghost,
                                        onclick: move |_| {
                                            sort_order_signal.with_mut(|order| {
                                                *order = match *order {
                                                    SortOrder::Asc => SortOrder::Desc,
                                                    SortOrder::Desc => SortOrder::Asc,
                                                };
                                            });
                                        },
                                        span { "Email" }
                                        ArrowUpDown { class: "ml-2" }
                                    }
                                }
                            }
                            if columns_signal.with(|c| c.contains("Amount")) {
                                DataTableHead { class: "px-4 text-right", "Amount" }
                            }
                            DataTableHead { class: "px-4", "" }
                        }
                    }
                    DataTableBody {
                        {filtered_payments_signal().into_iter().map(|payment| {
                            let is_selected = selected_ids_signal.with(|selected| selected.contains(&payment.id));
                            rsx! {
                                DataTableRow {
                                    data_state: if is_selected { "selected" } else { "" },
                                    DataTableCell {
                                        Checkbox {
                                            checked: is_selected,
                                            on_checked_change: move |checked| {
                                                selected_ids_signal.with_mut(|selected| {
                                                    if checked {
                                                        selected.insert(payment.id);
                                                    } else {
                                                        selected.remove(&payment.id);
                                                    }
                                                });
                                            },
                                        }
                                    }
                                    if columns_signal.with(|c| c.contains("Status")) {
                                        DataTableCell { {payment.status.to_string()} }
                                    }
                                    if columns_signal.with(|c| c.contains("Email")) {
                                        DataTableCell { {payment.email} }
                                    }
                                    if columns_signal.with(|c| c.contains("Amount")) {
                                        DataTableCell { class: "font-medium text-right",
                                            {format!("${:.2}", payment.amount)}
                                        }
                                    }
                                    DataTableCell {
                                        DropdownMenu {
                                            DropdownMenuTrigger { class: "flex justify-center items-center p-0 w-8 h-8",
                                                span { class: "hidden", "Open menu" }
                                                Ellipsis { class: "size-4" }
                                            }
                                            DropdownMenuContent { class: "w-[160px]",
                                                DropdownMenuLabel { "Actions" }
                                                DropdownMenuGroup { class: "mt-2",
                                                    DropdownMenuItem { class: "p-0",
                                                        AlertDialog { class: "w-full",
                                                            AlertDialogTrigger { class: "w-full",
                                                                Trash { class: "text-destructive" }
                                                                span { "Delete" }
                                                            }
                                                            AlertDialogContent { class: "w-[425px]",
                                                                AlertDialogBody {
                                                                    AlertDialogHeader {
                                                                        AlertDialogTitle { "Are you absolutely sure?" }
                                                                        AlertDialogDescription {
                                                                            "This will permanently delete the payment record for "
                                                                            strong { {payment.email} }
                                                                            "."
                                                                        }
                                                                    }
                                                                    AlertDialogFooter {
                                                                        AlertDialogClose { "Cancel" }
                                                                        Button {
                                                                            variant: ButtonVariant::Destructive,
                                                                            onclick: move |_| {
                                                                                deleted_ids_signal.with_mut(|d| {
                                                                                    d.insert(payment.id);
                                                                                });
                                                                                selected_ids_signal.with_mut(|s| {
                                                                                    s.remove(&payment.id);
                                                                                });
                                                                            },
                                                                            "Delete"
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            div { class: "flex justify-end items-center pt-4 space-x-2",
                div { class: "flex-1 text-sm text-muted-foreground",
                    {format!(
                        "{} of {} row(s) selected.",
                        selected_count_signal(),
                        filtered_payments_signal.with(|p| p.len()),
                    )}
                }
                div { class: "space-x-2",
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: true,
                        "Previous"
                    }
                    Button { variant: ButtonVariant::Outline, size: ButtonSize::Sm, disabled: true,
                        "Next"
                    }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     CONSTANTS                              */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, Display, EnumString, AsRefStr)]
enum PaymentStatus {
    #[default]
    Processing,
    Pending,
    Success,
    Failed,
}

#[derive(Clone, PartialEq)]
struct Payment {
    id: usize,
    status: PaymentStatus,
    email: &'static str,
    amount: f64,
}

impl Payment {
    fn new(id: usize) -> Self {
        Self { id, status: PaymentStatus::default(), email: "newuser@example.com", amount: 500.00 }
    }
}

const INITIAL_PAYMENTS: &[Payment] = &[
    Payment { id: 1, status: PaymentStatus::Failed, email: "isabella.n@gmail.com", amount: 874.00 },
    Payment { id: 2, status: PaymentStatus::Success, email: "jackson.lee@email.com", amount: 837.00 },
    Payment { id: 3, status: PaymentStatus::Success, email: "ken99@yahoo.com", amount: 316.00 },
    Payment { id: 4, status: PaymentStatus::Processing, email: "olivia@example.com", amount: 242.00 },
    Payment { id: 5, status: PaymentStatus::Success, email: "william@company.com", amount: 721.00 },
    Payment { id: 6, status: PaymentStatus::Pending, email: "sofia.h@example.com", amount: 430.00 },
    Payment { id: 7, status: PaymentStatus::Processing, email: "liam.j@work.com", amount: 155.00 },
    Payment { id: 8, status: PaymentStatus::Pending, email: "emma.w@email.com", amount: 620.00 },
    Payment { id: 9, status: PaymentStatus::Failed, email: "noah.b@company.com", amount: 990.00 },
    Payment { id: 10, status: PaymentStatus::Success, email: "ava.m@example.com", amount: 375.00 },
];
