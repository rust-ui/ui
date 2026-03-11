use std::collections::HashSet;

use icons::{ArrowUpDown, Ellipsis, Plus, Trash};
use leptos::portal::Portal;
use leptos::prelude::*;
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
pub fn DemoDataTable() -> impl IntoView {
    let selected_emails_signal = RwSignal::new(HashSet::<&'static str>::new());
    let fake_payment_count_signal = RwSignal::new(0_usize);
    let columns_signal = RwSignal::new(HashSet::from(COLUMNS.map(|c| c.to_string())));
    let sort_order_signal = RwSignal::new(SortOrder::default());
    let email_filter_signal = RwSignal::new(String::new());
    let deleted_ids_signal = RwSignal::new(HashSet::<usize>::new());

    let sorted_payments_signal = Memo::new(move |_| {
        let filter = email_filter_signal.get().to_lowercase();
        let mut payments: Vec<Payment> = INITIAL_PAYMENTS.to_vec();

        let count = fake_payment_count_signal.get();
        for i in 0..count {
            payments.push(Payment::new(1000 + i));
        }

        // Filter out deleted payments
        payments.retain(|payment| {
            !deleted_ids_signal.with(|deleted| deleted.contains(&payment.id))
                && payment.email.to_lowercase().contains(&filter)
        });

        match sort_order_signal.get() {
            SortOrder::Asc => payments.sort_by(|a, b| a.email.cmp(b.email)),
            SortOrder::Desc => payments.sort_by(|a, b| b.email.cmp(a.email)),
        }
        payments
    });

    let selected_count_signal = Signal::derive(move || {
        sorted_payments_signal.with(|payments| {
            selected_emails_signal
                .with(|selected| payments.iter().filter(|payment| selected.contains(&payment.email)).count())
        })
    });

    let handle_select_all = Callback::new(move |checked: bool| {
        selected_emails_signal.update(|selected| {
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
    });

    view! {
        <div class="py-8 w-full">
            <div class="flex gap-4 items-center mb-4">
                <Input
                    class="max-w-sm"
                    attr:placeholder="Filter emails..."
                    prop:value=move || email_filter_signal.get()
                    on:input=move |ev| email_filter_signal.set(event_target_value(&ev))
                />
                <MultiSelect values=columns_signal>
                    <MultiSelectTrigger class="ml-auto w-[150px]">
                        <MultiSelectValue placeholder="Columns" />
                    </MultiSelectTrigger>
                    <MultiSelectContent>
                        <MultiSelectGroup>
                            {COLUMNS
                                .into_iter()
                                .map(|column| {
                                    let is_email = column == "Email";
                                    view! {
                                        <MultiSelectItem>
                                            <MultiSelectOption value=column attr:disabled=is_email>
                                                {column}
                                            </MultiSelectOption>
                                        </MultiSelectItem>
                                    }
                                })
                                .collect_view()}
                        </MultiSelectGroup>
                    </MultiSelectContent>
                </MultiSelect>
                <Button
                    size=ButtonSize::Icon
                    variant=ButtonVariant::Outline
                    on:click=move |_| {
                        fake_payment_count_signal.update(|count| *count += 1);
                    }
                >
                    <Plus class="text-muted-foreground" />
                </Button>
            </div>

            <DataTableWrapper>
                <DataTable>
                    <DataTableHeader>
                        <DataTableRow>
                            <DataTableHead class="px-4">
                                <Checkbox
                                    attr:aria-label="Select all"
                                    checked=Signal::derive(move || {
                                        sorted_payments_signal
                                            .with(|payments| {
                                                !payments.is_empty() && selected_count_signal.get() == payments.len()
                                            })
                                    })
                                    on_checked_change=handle_select_all
                                />
                            </DataTableHead>
                            {move || {
                                columns_signal
                                    .get()
                                    .contains("Status")
                                    .then(|| {
                                        view! { <DataTableHead class="px-4">Status</DataTableHead> }
                                    })
                            }}

                            {move || {
                                columns_signal
                                    .get()
                                    .contains("Email")
                                    .then(|| {
                                        view! {
                                            <DataTableHead class="px-4">
                                                <Button
                                                    variant=ButtonVariant::Ghost
                                                    on:click=move |_| {
                                                        sort_order_signal
                                                            .update(|order| {
                                                                *order = match *order {
                                                                    SortOrder::Asc => SortOrder::Desc,
                                                                    SortOrder::Desc => SortOrder::Asc,
                                                                };
                                                            });
                                                    }
                                                >
                                                    <span>Email</span>
                                                    <ArrowUpDown class="ml-2" />
                                                </Button>
                                            </DataTableHead>
                                        }
                                    })
                            }}

                            {move || {
                                columns_signal
                                    .get()
                                    .contains("Amount")
                                    .then(|| {
                                        view! { <DataTableHead class="px-4 text-right">Amount</DataTableHead> }
                                    })
                            }}

                            <DataTableHead class="px-4">""</DataTableHead>
                        </DataTableRow>
                    </DataTableHeader>
                    <DataTableBody>
                        <For each=move || sorted_payments_signal.get() key=|payment| payment.id let:payment>
                            {
                                let is_selected = Signal::derive(move || {
                                    selected_emails_signal.with(|selected| selected.contains(payment.email))
                                });
                                view! {
                                    <DataTableRow attr:data-state=move || {
                                        if is_selected.get() { "selected" } else { "" }
                                    }>
                                        <DataTableCell>
                                            <Checkbox
                                                checked=is_selected
                                                on_checked_change=Callback::new(move |checked| {
                                                    selected_emails_signal
                                                        .update(|selected| {
                                                            if checked {
                                                                selected.insert(payment.email);
                                                            } else {
                                                                selected.remove(payment.email);
                                                            }
                                                        });
                                                })
                                            />
                                        </DataTableCell>
                                        {move || {
                                            columns_signal
                                                .get()
                                                .contains("Status")
                                                .then(|| {
                                                    view! {
                                                        <DataTableCell>{payment.status.to_string()}</DataTableCell>
                                                    }
                                                })
                                        }}

                                        {move || {
                                            columns_signal
                                                .get()
                                                .contains("Email")
                                                .then(|| {
                                                    view! { <DataTableCell>{payment.email}</DataTableCell> }
                                                })
                                        }}

                                        {move || {
                                            columns_signal
                                                .get()
                                                .contains("Amount")
                                                .then(|| {
                                                    view! {
                                                        <DataTableCell class="font-medium text-right">
                                                            {format!("${:.2}", payment.amount)}
                                                        </DataTableCell>
                                                    }
                                                })
                                        }}

                                        <DataTableCell>
                                            <DropdownMenu>
                                                <DropdownMenuTrigger class="flex justify-center items-center p-0 w-8 h-8">
                                                    <span class="hidden">Open menu</span>
                                                    <Ellipsis class="size-4" />
                                                </DropdownMenuTrigger>
                                                <DropdownMenuContent class="w-[160px]">
                                                    <DropdownMenuLabel>Actions</DropdownMenuLabel>
                                                    <DropdownMenuGroup class="mt-2">
                                                        <DropdownMenuItem class="p-0">
                                                            <AlertDialog class="w-full">
                                                                <AlertDialogTrigger class="w-full">
                                                                    <Trash class="text-destructive" />
                                                                    <span>Delete</span>
                                                                </AlertDialogTrigger>

                                                                <Portal>
                                                                    <AlertDialogContent class="w-[425px]">
                                                                        <AlertDialogBody>
                                                                            <AlertDialogHeader>
                                                                                <AlertDialogTitle>
                                                                                    "Are you absolutely sure?"
                                                                                </AlertDialogTitle>

                                                                                <AlertDialogDescription>
                                                                                    "This action cannot be undone. This will permanently delete the payment record for "
                                                                                    <strong>{payment.email}</strong> "."
                                                                                </AlertDialogDescription>
                                                                            </AlertDialogHeader>

                                                                            <AlertDialogFooter>
                                                                                <AlertDialogClose>"Cancel"</AlertDialogClose>
                                                                                <Button
                                                                                    variant=ButtonVariant::Destructive
                                                                                    on:click=move |_| {
                                                                                        deleted_ids_signal
                                                                                            .update(|deleted| {
                                                                                                deleted.insert(payment.id);
                                                                                            });
                                                                                        selected_emails_signal
                                                                                            .update(|selected| {
                                                                                                selected.remove(payment.email);
                                                                                            });
                                                                                    }
                                                                                >
                                                                                    "Delete"
                                                                                </Button>
                                                                            </AlertDialogFooter>
                                                                        </AlertDialogBody>
                                                                    </AlertDialogContent>
                                                                </Portal>
                                                            </AlertDialog>
                                                        </DropdownMenuItem>
                                                    </DropdownMenuGroup>
                                                </DropdownMenuContent>
                                            </DropdownMenu>
                                        </DataTableCell>
                                    </DataTableRow>
                                }
                            }

                        </For>
                    </DataTableBody>
                </DataTable>
            </DataTableWrapper>

            <div class="flex justify-end items-center pt-4 space-x-2">
                <div class="flex-1 text-sm text-muted-foreground">
                    {move || {
                        format!(
                            "{} of {} row(s) selected.",
                            selected_count_signal.get(),
                            sorted_payments_signal.with(|p| p.len()),
                        )
                    }}

                </div>
                <div class="space-x-2">
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm attr:disabled>
                        Previous
                    </Button>
                    <Button variant=ButtonVariant::Outline size=ButtonSize::Sm attr:disabled>
                        Next
                    </Button>
                </div>
            </div>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
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
