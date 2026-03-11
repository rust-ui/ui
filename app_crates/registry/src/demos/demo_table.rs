use leptos::prelude::*;

use crate::ui::table::{Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow};

#[component]
pub fn DemoTable() -> impl IntoView {
    view! {
        <Table>
            <TableCaption>{"A list of your recent invoices."}</TableCaption>
            <TableHeader>
                <TableRow>
                    <TableHead>{"Invoice"}</TableHead>
                    <TableHead>{"Status"}</TableHead>
                    <TableHead>{"Method"}</TableHead>
                    <TableHead class="text-right">{"Amount"}</TableHead>
                </TableRow>
            </TableHeader>

            <TableBody>
                {INVOICES
                    .iter()
                    .map(|(invoice, status, method, amount)| {
                        view! {
                            <TableRow>
                                <TableCell>{*invoice}</TableCell>
                                <TableCell>{*status}</TableCell>
                                <TableCell>{*method}</TableCell>
                                <TableCell class="text-right">{*amount}</TableCell>
                            </TableRow>
                        }
                    })
                    .collect::<Vec<_>>()}
            </TableBody>

            <TableFooter>
                <TableRow>
                    // TODO UI Table --> colSpan
                    // <TableCell colSpan="3">{"Total"}</TableCell>
                    <TableCell>{"Total"}</TableCell>
                    <TableCell>{""}</TableCell>
                    <TableCell>{""}</TableCell>
                    <TableCell class="text-right">{"$2,500.00"}</TableCell>
                </TableRow>
            </TableFooter>
        </Table>
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
