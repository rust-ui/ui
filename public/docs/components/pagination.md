+++
title = "Pagination"
description = "Rust/UI component that displays a pagination component."
tags = ["navigation"]
is_new = false
image = "/images/thumbnails/pagination.webp"
image_dark = "/images/thumbnails/pagination-dark.webp"
+++

<StaticPagination />

## Installation

<StaticInstallPagination />

## Usage

```rust
use crate::ui::pagination::{
    Pagination, PaginationList, PaginationItem,
    PaginationLink, PaginationPrev, PaginationNext,
};
```

```rust
let mut page = use_signal(|| 1u32);

rsx! {
    Pagination {
        PaginationList {
            PaginationItem {
                PaginationPrev {
                    disabled: page() == 1,
                    onclick: move |_| { if page() > 1 { page.set(page() - 1); } },
                }
            }
            for p in 1..=5 {
                PaginationItem {
                    PaginationLink {
                        page: p,
                        is_active: page() == p,
                        onclick: move |_| page.set(p),
                    }
                }
            }
            PaginationItem {
                PaginationNext {
                    disabled: page() == 5,
                    onclick: move |_| { if page() < 5 { page.set(page() + 1); } },
                }
            }
        }
    }
}
```

## See Also

- [Button](/components/button)
- [Tabs](/components/tabs)
