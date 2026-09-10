use dioxus::prelude::*;

use crate::ui::direction_provider::{Direction, DirectionProvider};
use crate::ui::table::{Table, TableBody, TableCaption, TableCell, TableFooter, TableHead, TableHeader, TableRow};

const INVOICES_RTL: &[(&str, &str, &str, &str)] = &[
    ("INV001", "مدفوع", "بطاقة ائتمان", "$250.00"),
    ("INV002", "معلق", "PayPal", "$150.00"),
    ("INV003", "غير مدفوع", "تحويل بنكي", "$350.00"),
    ("INV004", "مدفوع", "بطاقة ائتمان", "$450.00"),
    ("INV005", "مدفوع", "PayPal", "$550.00"),
];

#[component]
pub fn DemoTableRtl() -> Element {
    rsx! {
        DirectionProvider { dir: Direction::Rtl, class: "w-full max-w-2xl",
            Table {
                TableCaption { "قائمة فواتيرك الأخيرة." }
                TableHeader {
                    TableRow {
                        TableHead { "الفاتورة" }
                        TableHead { "الحالة" }
                        TableHead { "طريقة الدفع" }
                        TableHead { class: "text-left", "المبلغ" }
                    }
                }
                TableBody {
                    for (invoice, status, method, amount) in INVOICES_RTL.iter() {
                        TableRow {
                            TableCell { {*invoice} }
                            TableCell { {*status} }
                            TableCell { {*method} }
                            TableCell { class: "text-left", {*amount} }
                        }
                    }
                }
                TableFooter {
                    TableRow {
                        TableCell { "الإجمالي" }
                        TableCell { "" }
                        TableCell { "" }
                        TableCell { class: "text-left", "$2,500.00" }
                    }
                }
            }
        }
    }
}
