use dioxus::prelude::*;
use registry::ui::card::{Card, CardContent, CardDescription, CardHeader, CardTitle};
use registry::ui::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};

const PAYMENTS: [(&str, &str, &str, &str); 7] = [
    ("INV001", "Paid", "Credit Card", "$250.00"),
    ("INV002", "Pending", "PayPal", "$150.00"),
    ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
    ("INV004", "Paid", "Credit Card", "$450.00"),
    ("INV005", "Paid", "PayPal", "$550.00"),
    ("INV006", "Pending", "Bank Transfer", "$200.00"),
    ("INV007", "Unpaid", "Credit Card", "$300.00"),
];

#[component]
pub fn CardPaymentsTable() -> Element {
    rsx! {
        Card {
            CardHeader {
                CardTitle { "Payments" }
                CardDescription { "Manage your payments." }
            }
            CardContent { class: "pt-0",
                Table {
                    TableHeader {
                        TableRow {
                            TableHead { "Invoice" }
                            TableHead { "Status" }
                            TableHead { "Method" }
                            TableHead { class: "text-right", "Amount" }
                        }
                    }
                    TableBody {
                        {PAYMENTS.iter().map(|(invoice, status, method, amount)| rsx! {
                            TableRow { key: "{invoice}",
                                TableCell { {*invoice} }
                                TableCell { {*status} }
                                TableCell { {*method} }
                                TableCell { class: "text-right", {*amount} }
                            }
                        })}
                    }
                }
            }
        }
    }
}
