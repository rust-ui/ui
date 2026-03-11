+++
title = "Breadcrumb"
description = "Rust/UI component that displays the path to the current resource using a hierarchy of links."
tags = ["navigation"]
is_new = false
image = "/images/thumbnails/breadcrumb.webp"
image_dark = "/images/thumbnails/breadcrumb-dark.webp"
+++

<StaticBreadcrumb />





## Installation

<StaticInstallBreadcrumb />




## Components

The Breadcrumb component is composed of several subcomponents:

- **Breadcrumb**: Main navigation wrapper component
- **BreadcrumbList**: Ordered list container for breadcrumb items
- **BreadcrumbItem**: Individual breadcrumb item wrapper
- **BreadcrumbLink**: Clickable link for navigation to parent pages
- **BreadcrumbPage**: Current page indicator (non-clickable)
- **BreadcrumbSeparator**: Visual separator between items
- **BreadcrumbEllipsis**: Collapsed items indicator for long paths



## Usage

```rust
use crate::components::ui::breadcrumb::{
    Breadcrumb,
    BreadcrumbEllipsis,
    BreadcrumbItem,
    BreadcrumbLink,
    BreadcrumbList,
    BreadcrumbPage,
    BreadcrumbSeparator,
};
```

```rust
<Breadcrumb>
    <BreadcrumbList>
        <BreadcrumbItem>
            <BreadcrumbLink href="/">"Home"</BreadcrumbLink>
        </BreadcrumbItem>
        <BreadcrumbSeparator />
        <BreadcrumbItem>
            <BreadcrumbPage>"Current Page"</BreadcrumbPage>
        </BreadcrumbItem>
    </BreadcrumbList>
</Breadcrumb>
```


## Examples

### Dropdown

Collapse middle breadcrumb items into a dropdown for long navigation paths.

<StaticBreadcrumbDropdown />


### RTL

Breadcrumb trail in Arabic. Separator chevrons flip to point in the correct reading direction and the trail reads right-to-left.

<StaticBreadcrumbRtl />

## See Also

- [Pagination](/docs/components/pagination)
- [Bottom Nav](/docs/components/bottom-nav)
