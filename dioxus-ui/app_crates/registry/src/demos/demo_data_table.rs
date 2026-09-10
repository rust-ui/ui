use std::collections::HashSet;

use dioxus::prelude::*;
use icons::{ArrowUpDown, Ellipsis, Plus, Trash};
use strum::{AsRefStr, Display, EnumString};

use crate::ui::alert_dialog::{
    AlertDialog, AlertDialogBody, AlertDialogClose, AlertDialogContent, AlertDialogDescription, AlertDialogFooter,
    AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger,
};
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
enum SortOrder {
    #[default]
    Asc,
    Desc,
}

const COLUMNS: [&str; 3] = ["Status", "Email", "Amount"];

#[component]
pub fn DemoDataTable() -> Element {
    let mut selected_emails_signal = use_signal(HashSet::<&'static str>::new);
    let mut fake_payment_count_signal = use_signal(|| 0_usize);
    let columns_signal = use_signal(|| HashSet::from(COLUMNS.map(|c| c.to_string())));
    let mut sort_order_signal = use_signal(SortOrder::default);
    let mut email_filter_signal = use_signal(String::new);
    let mut deleted_ids_signal = use_signal(HashSet::<usize>::new);

    let sorted_payments_signal = use_memo(move || {
        let filter = email_filter_signal().to_lowercase();
        let mut payments: Vec<Payment> = INITIAL_PAYMENTS.to_vec();

        let count = fake_payment_count_signal();
        for i in 0..count {
            payments.push(Payment::new(1000 + i));
        }

        // Filter out deleted payments
        payments.retain(|payment| {
            !deleted_ids_signal.with(|deleted| deleted.contains(&payment.id))
                && payment.email.to_lowercase().contains(&filter)
        });

        match sort_order_signal() {
            SortOrder::Asc => payments.sort_by(|a, b| a.email.cmp(b.email)),
            SortOrder::Desc => payments.sort_by(|a, b| b.email.cmp(a.email)),
        }
        payments
    });

    let selected_count_signal = use_memo(move || {
        sorted_payments_signal.with(|payments| {
            selected_emails_signal
                .with(|selected| payments.iter().filter(|payment| selected.contains(&payment.email)).count())
        })
    });

    let handle_select_all = move |checked: bool| {
        selected_emails_signal.with_mut(|selected| {
            sorted_payments_signal.with(|payments| {
                for payment in payments {
                    if checked {
                        selected.insert(payment.email);
                    } else {
                        selected.remove(payment.email);
                    }
                }
            });
        });
    };

    rsx! {
        div { class: "py-8 w-full",
            div { class: "flex gap-4 items-center mb-4",
                Input {
                    class: "max-w-sm",
                    placeholder: "Filter emails...",
                    value: email_filter_signal(),
                    oninput: move |ev: Event<FormData>| email_filter_signal.set(ev.value()),
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

            DataTableWrapper {
                DataTable {
                    DataTableHeader {
                        DataTableRow {
                            DataTableHead { class: "px-4",
                                Checkbox {
                                    aria_label: "Select all",
                                    checked: sorted_payments_signal.with(|payments| {
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
                        {sorted_payments_signal().into_iter().map(|payment| {
                            let is_selected = selected_emails_signal.with(|selected| selected.contains(&payment.email));
                            rsx! {
                                DataTableRow {
                                    data_state: if is_selected { "selected" } else { "" },
                                    DataTableCell {
                                        Checkbox {
                                            checked: is_selected,
                                            on_checked_change: move |checked| {
                                                selected_emails_signal.with_mut(|selected| {
                                                    if checked {
                                                        selected.insert(payment.email);
                                                    } else {
                                                        selected.remove(payment.email);
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
                                                                            "This action cannot be undone. This will permanently delete the payment record for "
                                                                            strong { {payment.email} }
                                                                            "."
                                                                        }
                                                                    }
                                                                    AlertDialogFooter {
                                                                        AlertDialogClose { "Cancel" }
                                                                        Button {
                                                                            variant: ButtonVariant::Destructive,
                                                                            onclick: move |_| {
                                                                                deleted_ids_signal.with_mut(|deleted| {
                                                                                    deleted.insert(payment.id);
                                                                                });
                                                                                selected_emails_signal.with_mut(|selected| {
                                                                                    selected.remove(payment.email);
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
                        sorted_payments_signal.with(|p| p.len()),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Display, EnumString, AsRefStr)]
enum PaymentStatus {
    #[default]
    Processing,
    Failed,
    Success,
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
];
