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




## Components

The Pagination component is composed of several subcomponents:

- **Pagination**: Main wrapper component for navigation
- **PaginationList**: Container for pagination items
- **PaginationItem**: Individual item wrapper
- **PaginationLink**: Clickable link to a specific page
- **PaginationNavButton**: Previous/next navigation buttons
- **PageDirection**: Enum for navigation direction (Previous/Next)



## Usage

```rust
use crate::components::ui::pagination::{
    PageDirection, Pagination, PaginationItem, PaginationLink,
    PaginationList, PaginationNavButton,
};
```

```rust
<Pagination>
    <PaginationList>
        <PaginationItem>
            <PaginationNavButton direction=PageDirection::Previous />
        </PaginationItem>
        <PaginationItem>
            <PaginationLink page=1 />
        </PaginationItem>
        <PaginationItem>
            <PaginationLink page=2 />
        </PaginationItem>
        <PaginationItem>
            <PaginationLink page=3 />
        </PaginationItem>
        <PaginationItem>
            <PaginationNavButton direction=PageDirection::Next />
        </PaginationItem>
    </PaginationList>
</Pagination>
```


### RTL

Pagination controls in RTL. Previous and Next arrows flip so navigation flows naturally from right to left.

<StaticPaginationRtl />

## See Also

- [Breadcrumb](/docs/components/breadcrumb)
- [Data Table](/docs/components/data-table)
