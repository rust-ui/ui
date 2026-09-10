use dioxus::prelude::*;

use crate::ui::table::{Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow};

#[component]
pub fn DemoTable() -> Element {
    rsx! {
        Table {
            TableCaption { "A list of your recent invoices." }
            TableHeader {
                TableRow {
                    TableHead { "Invoice" }
                    TableHead { "Status" }
                    TableHead { "Method" }
                    TableHead { class: "text-right", "Amount" }
                }
            }
            TableBody {
                {INVOICES.iter().map(|(invoice, status, method, amount)| rsx! {
                    TableRow {
                        TableCell { {*invoice} }
                        TableCell { {*status} }
                        TableCell { {*method} }
                        TableCell { class: "text-right", {*amount} }
                    }
                })}
            }
            TableFooter {
                TableRow {
                    TableCell { "Total" }
                    TableCell { "" }
                    TableCell { "" }
                    TableCell { class: "text-right", "$2,500.00" }
                }
            }
        }
    }
}

/* ========================================================== */
/*                     ✨ CONSTANTS ✨                        */
/* ========================================================== */

const INVOICES: [(&str, &str, &str, &str); 7] = [
    ("INV001", "Paid", "Credit Card", "$250.00"),
    ("INV002", "Pending", "PayPal", "$150.00"),
    ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
    ("INV004", "Paid", "Credit Card", "$450.00"),
    ("INV005", "Paid", "PayPal", "$550.00"),
    ("INV006", "Pending", "Bank Transfer", "$200.00"),
    ("INV007", "Unpaid", "Credit Card", "$300.00"),
];
