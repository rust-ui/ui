+++
title = "Data Table"
description = "Powerful data table with advanced features including filtering, sorting, column visibility, and row selection."
tags = ["table"]
is_new = true
image = "/images/thumbnails/table.webp"
image_dark = "/images/thumbnails/table-dark.webp"
+++


<StaticDataTable />



## With Filters

<StaticDataTableFilters />



## Installation

<StaticInstallTable />



## Components

The DataTable component is composed of several subcomponents:

- **DataTableWrapper**: Outer wrapper with overflow handling
- **DataTable**: Main table component
- **DataTableHeader**: Header section containing column headings
- **DataTableBody**: Body section containing data rows
- **DataTableRow**: Individual table row container
- **DataTableHead**: Header cell for column titles
- **DataTableCell**: Data cell for row content



## Usage

You can use the `DataTable` component in combination with the [AlertDialog](/docs/components/alert-dialog), [Button](/docs/components/button), [Checkbox](/docs/components/checkbox), [DropdownMenu](/docs/components/dropdown-menu), [Input](/docs/components/input), and [MultiSelect](/docs/components/multi-select) components.

```rust
use crate::components::ui::data_table::{
    DataTable,
    DataTableBody,
    DataTableCell,
    DataTableHead,
    DataTableHeader,
    DataTableRow,
    DataTableWrapper,
};
```

```rust
<DataTableWrapper>
    <DataTable>
        <DataTableHeader>
            <DataTableRow>
                <DataTableHead>"Name"</DataTableHead>
                <DataTableHead>"Email"</DataTableHead>
                <DataTableHead>"Status"</DataTableHead>
            </DataTableRow>
        </DataTableHeader>
        <DataTableBody>
            <DataTableRow>
                <DataTableCell>"John Doe"</DataTableCell>
                <DataTableCell>"john@example.com"</DataTableCell>
                <DataTableCell>"Active"</DataTableCell>
            </DataTableRow>
            <DataTableRow>
                <DataTableCell>"Jane Smith"</DataTableCell>
                <DataTableCell>"jane@example.com"</DataTableCell>
                <DataTableCell>"Pending"</DataTableCell>
            </DataTableRow>
        </DataTableBody>
    </DataTable>
</DataTableWrapper>
```


## Features

The Data Table component provides:

- **Filtering**: Search and filter data by email or other fields
- **Sorting**: Click column headers to sort data in ascending or descending order
- **Column Visibility**: Toggle column visibility using the MultiSelect dropdown
- **Row Selection**: Select individual rows or all rows with checkboxes
- **Pagination**: Navigate through large datasets (UI controls included)




## See Also

- [Table](/docs/components/table)
- [Pagination](/docs/components/pagination)
- [Select](/docs/components/select)
- [Checkbox](/docs/components/checkbox)
- [Scroll Area](/docs/components/scroll-area)
