---
title: "Demo Table Rtl"
name: "demo_table_rtl"
cargo_dependencies: []
registry_dependencies: ["direction_provider", "table"]
type: "components:demos"
path: "demos/demo_table_rtl.rs"
---

# Demo Table Rtl

This component demo demonstrates practical implementation patterns and provides a concrete usage example for LLMs to understand the code structure and functionality.

## Installation

To add this component demo in your app, run:

```bash
# cargo install ui-cli --force
ui add demo_table_rtl
```

## Component Code

```rust
use leptos::prelude::*;

use crate::components::ui::direction_provider::{Direction, DirectionProvider};
use crate::components::ui::table::{Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow};

#[component]
pub fn DemoTableRtl() -> impl IntoView {
    view! {
        <DirectionProvider dir=Direction::Rtl class="w-full max-w-2xl">
            <Table>
                <TableCaption>{"قائمة فواتيرك الأخيرة."}</TableCaption>
                <TableHeader>
                    <TableRow>
                        <TableHead>{"الفاتورة"}</TableHead>
                        <TableHead>{"الحالة"}</TableHead>
                        <TableHead>{"طريقة الدفع"}</TableHead>
                        <TableHead class="text-left">{"المبلغ"}</TableHead>
                    </TableRow>
                </TableHeader>

                <TableBody>
                    {INVOICES_RTL
                        .iter()
                        .map(|(invoice, status, method, amount)| {
                            view! {
                                <TableRow>
                                    <TableCell>{*invoice}</TableCell>
                                    <TableCell>{*status}</TableCell>
                                    <TableCell>{*method}</TableCell>
                                    <TableCell class="text-left">{*amount}</TableCell>
                                </TableRow>
                            }
                        })
                        .collect::<Vec<_>>()}
                </TableBody>

                <TableFooter>
                    <TableRow>
                        <TableCell>{"الإجمالي"}</TableCell>
                        <TableCell>{""}</TableCell>
                        <TableCell>{""}</TableCell>
                        <TableCell class="text-left">{"$2,500.00"}</TableCell>
                    </TableRow>
                </TableFooter>
            </Table>
        </DirectionProvider>
    }
}

const INVOICES_RTL: [(&str, &str, &str, &str); 5] = [
    ("INV001", "مدفوع", "بطاقة ائتمان", "$250.00"),
    ("INV002", "معلق", "PayPal", "$150.00"),
    ("INV003", "غير مدفوع", "تحويل بنكي", "$350.00"),
    ("INV004", "مدفوع", "بطاقة ائتمان", "$450.00"),
    ("INV005", "مدفوع", "PayPal", "$550.00"),
];
```
