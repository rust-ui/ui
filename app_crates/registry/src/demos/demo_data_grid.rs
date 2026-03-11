use std::collections::HashSet;
use std::sync::LazyLock;

use icons::{Copy, Eraser, Plus, Scissors, Settings2, Trash2};
use leptos::prelude::*;
use serde::{Deserialize, Serialize};
use strum::{AsRefStr, Display, EnumIter, IntoEnumIterator};

use crate::hooks::use_cell_edit::use_cell_edit;
use crate::hooks::use_column_state::{ColumnState, use_column_state};
use crate::hooks::use_copy_clipboard::use_copy_clipboard;
use crate::hooks::use_data_grid_state::{DataGridState, use_data_grid_state};
use crate::hooks::use_press_hold::use_press_hold;
use crate::ui::badge::{Badge, BadgeVariant};
use crate::ui::checkbox::Checkbox;
use crate::ui::context_menu::{
    ContextMenu, ContextMenuAction, ContextMenuContent, ContextMenuGroup, ContextMenuItem, ContextMenuTrigger,
};
use crate::ui::data_grid::{
    DataGridColumn, DataGridToolbar, EditableCellContent, Grid, GridBody, GridCell, GridCellWrapper, GridHeaderCell,
    GridPinnedCell, GridPinnedHeaderCell, GridRow, GridSelectCell, GridSelectHeaderCell, GridWrapper, PinnableColumn,
    PinnableSortableHeaderCell, SortDirection, SortableColumn, generate_grid_style, get_column_width,
    get_pinned_left_position, get_pinned_visible_columns,
};
use crate::ui::multi_select::{
    MultiSelect, MultiSelectAlign, MultiSelectContent, MultiSelectGroup, MultiSelectItem, MultiSelectOption,
    MultiSelectTrigger,
};
use crate::ui::separator::Separator;
use crate::ui::toast_custom::toaster::expect_toaster;

/* ========================================================== */
/*                     ✨ TYPES ✨                            */
/* ========================================================== */

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Display, AsRefStr, EnumIter)]
#[strum(serialize_all = "title_case")]
pub enum Column {
    Select,
    Name,
    Age,
    Email,
    Website,
    Notes,
    Salary,
    Department,
    Status,
    Skills,
    IsActive,
    StartDate,
    Attachments,
}

impl DataGridColumn for Column {
    fn colindex(self) -> i32 {
        self as i32 + 1
    }

    fn is_disabled(self) -> bool {
        matches!(self, Self::Select | Self::Name | Self::Email)
    }
}

impl SortableColumn<RowData> for Column {
    fn compare(self, a: &RowData, b: &RowData) -> Option<std::cmp::Ordering> {
        match self {
            Self::Name => Some(a.name.cmp(&b.name)),
            Self::Age => Some(a.age.cmp(&b.age)),
            Self::Email => Some(a.email.cmp(&b.email)),
            Self::Website => Some(a.website.cmp(&b.website)),
            Self::Notes => Some(a.notes.cmp(&b.notes)),
            Self::Salary => Some(a.salary.cmp(&b.salary)),
            Self::Department => Some(a.department.cmp(&b.department)),
            Self::Status => Some(a.status.cmp(&b.status)),
            Self::Skills => Some(a.skills.len().cmp(&b.skills.len())),
            Self::IsActive => Some(a.is_active.cmp(&b.is_active)),
            Self::StartDate => Some(a.start_date.cmp(&b.start_date)),
            Self::Select | Self::Attachments => None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct RowData {
    pub name: String,
    pub age: i32,
    pub email: String,
    pub website: String,
    pub notes: String,
    pub salary: i32,
    pub department: String,
    pub status: String,
    pub skills: Vec<String>,
    pub is_active: bool,
    pub start_date: String,
}

/// Pinnable columns with their widths in order they should appear when pinned
const PINNABLE_COLUMNS: &[(Column, i32)] = &[
    (Column::Name, 180),
    (Column::Age, 150),
    (Column::Email, 240),
    (Column::Website, 240),
    (Column::Notes, 200),
    (Column::Salary, 180),
    (Column::Department, 180),
    (Column::Status, 180),
    (Column::Skills, 240),
    (Column::IsActive, 110),
    (Column::StartDate, 150),
    (Column::Attachments, 240),
];

impl PinnableColumn for Column {
    fn pinnable_columns() -> &'static [(Self, i32)] {
        PINNABLE_COLUMNS
    }
}

impl Column {
    fn wrapper_class(self) -> &'static str {
        match self {
            Self::IsActive => "flex justify-center",
            _ => "",
        }
    }

    fn get_value(self, row: &RowData) -> String {
        match self {
            Self::Name => row.name.clone(),
            Self::Age => row.age.to_string(),
            Self::Email => row.email.clone(),
            Self::Website => row.website.clone(),
            Self::Notes => row.notes.clone(),
            Self::Salary => row.salary.to_string(),
            Self::Department => row.department.clone(),
            Self::Status => row.status.clone(),
            Self::Skills => row.skills.join(", "),
            Self::IsActive => row.is_active.to_string(),
            Self::StartDate => row.start_date.clone(),
            Self::Select | Self::Attachments => String::new(),
        }
    }
}

/// CSS custom properties for column sizes, generated from PINNABLE_COLUMNS.
static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);

/* ========================================================== */
/*                     ✨ SERVER FUNCTION ✨                  */
/* ========================================================== */

#[server]
pub async fn get_data_grid_rows() -> Result<Vec<RowData>, ServerFnError> {
    // Simulate server-side data fetching
    let rows = vec![
        RowData {
            name: "Toby Deckow".to_string(),
            age: 50,
            email: "toby.deckow32@yahoo.com".to_string(),
            website: "https://young-plain.net".to_string(),
            notes: "Relocated from the Seattle office last month. Adjusting well to the new team dynamics and company culture.".to_string(),
            salary: 42953,
            department: "Finance".to_string(),
            status: "In Office".to_string(),
            skills: vec!["Docker".to_string(), "JavaScript".to_string(), "SQL".to_string()],
            is_active: true,
            start_date: "12/13/2020".to_string(),
        },
        RowData {
            name: "Montserrat Kutch".to_string(),
            age: 52,
            email: "montserrat_kutch@yahoo.com".to_string(),
            website: "https://major-footrest.net".to_string(),
            notes: "Transferred from the marketing department. Bringing valuable cross-functional experience to the team.".to_string(),
            salary: 64820,
            department: "Marketing".to_string(),
            status: "In Office".to_string(),
            skills: vec!["AWS".to_string(), "Git".to_string(), "SQL".to_string(), "React".to_string()],
            is_active: true,
            start_date: "6/28/2023".to_string(),
        },
        RowData {
            name: "Alice Chen".to_string(),
            age: 34,
            email: "alice.chen@gmail.com".to_string(),
            website: "https://devblog-alice.io".to_string(),
            notes: "Senior developer with expertise in distributed systems. Leading the backend migration project.".to_string(),
            salary: 95000,
            department: "Engineering".to_string(),
            status: "Remote".to_string(),
            skills: vec!["Rust".to_string(), "Kubernetes".to_string(), "PostgreSQL".to_string(), "Go".to_string()],
            is_active: true,
            start_date: "3/15/2019".to_string(),
        },
        RowData {
            name: "James Rodriguez".to_string(),
            age: 28,
            email: "j.rodriguez@company.com".to_string(),
            website: "https://jamesux.design".to_string(),
            notes: "UI/UX designer focused on accessibility and user research. Recently completed design system overhaul.".to_string(),
            salary: 72500,
            department: "Design".to_string(),
            status: "Hybrid".to_string(),
            skills: vec!["Figma".to_string(), "CSS".to_string(), "TypeScript".to_string()],
            is_active: false,
            start_date: "9/01/2022".to_string(),
        },
    ];

    Ok(rows)
}

#[component]
pub fn DemoDataGrid() -> impl IntoView {
    view! { <DataGridFull /> }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn DataGridFull() -> impl IntoView {
    let rows_resource = Resource::new(|| (), |_| async move { get_data_grid_rows().await });
    let rows_signal = RwSignal::new(Vec::<RowData>::new());

    let selected_indices_signal = RwSignal::new(HashSet::<usize>::new());

    let ColumnState { sort_signals, pinned_columns_signal, visible_columns_signal } =
        use_column_state(PINNABLE_COLUMNS);

    // Grid state (cell selection, drag selection, click outside)
    let DataGridState { cell_selection, drag_selection, copy_value_signal, grid_wrapper_ref } =
        use_data_grid_state::<Column>();

    // Copy to clipboard
    let (copy_to_clipboard, _) = use_copy_clipboard(None);
    let copy_to_clipboard = StoredValue::new(copy_to_clipboard);

    // Cell edit hook (double-click to edit)
    let _cell_edit = use_cell_edit::<Column>();

    // Create sorted rows memo
    let sorted_rows_signal = Memo::new(move |_| {
        let mut rows: Vec<RowData> = rows_signal.get();

        // Apply sorting in order (last one with a value takes precedence)
        sort_signals.with_value(|signals| {
            for (col, _) in PINNABLE_COLUMNS {
                if let Some(signal) = signals.get(col) {
                    col.sort_rows(&mut rows, signal.get());
                }
            }
        });

        rows
    });

    let row_count_signal = Signal::derive(move || rows_signal.with(|rows| rows.len()));

    // Dynamic height for virtual scrolling: (row_count + 1) * 36px to include add row button
    let grid_body_height = Signal::derive(move || {
        let row_count = row_count_signal.get();
        format!("height: {}px", (row_count + 1) * 36)
    });

    let selected_count_signal = Signal::derive(move || selected_indices_signal.with(|selected| selected.len()));

    let handle_select_all = Callback::new(move |checked: bool| {
        let row_count = row_count_signal.get();
        selected_indices_signal.update(|selected| {
            if checked {
                for index in 0..row_count {
                    selected.insert(index);
                }
            } else {
                selected.clear();
            }
        });
    });

    // Delete rows handler (swap this for server action when using real DB)
    let handle_delete_rows = Callback::new(move |names: Vec<String>| {
        rows_signal.update(|rows| {
            rows.retain(|r| !names.contains(&r.name));
        });
    });

    view! {
        <div class="container flex flex-col py-4">
            <ToolbarDataGrid visible_columns_signal />

            <Transition fallback=move || {
                view! { <p class="text-gray-500">"Loading data..."</p> }
            }>
                {move || {
                    rows_resource
                        .get()
                        .map(|result| {
                            match result {
                                Ok(rows) => {
                                    rows_signal.set(rows);
                                    view! {
                                        <div
                                            node_ref=grid_wrapper_ref
                                            on:mouseup=move |_| drag_selection.stop_dragging()
                                            on:mouseleave=move |_| drag_selection.stop_dragging()
                                        >
                                            <GridWrapper>
                                                <button
                                                    type="button"
                                                    id="radix-_R_mlubsqlb_"
                                                    aria-haspopup="menu"
                                                    aria-expanded="false"
                                                    data-state="closed"
                                                    data-slot="dropdown-menu-trigger"
                                                    style="position:fixed;left:0px;top:0px;width:1px;height:1px;padding:0;margin:0;border:none;background:transparent;pointer-events:none;opacity:0"
                                                ></button>
                                                <Grid
                                                    rowcount=Signal::derive(move || {
                                                        i32::try_from(row_count_signal.get()).unwrap_or(0) + 1
                                                    })
                                                    colcount=i32::try_from(Column::iter().count()).unwrap_or(0)
                                                    style=GRID_STYLE.as_str()
                                                >
                                                    // ------------- GRID HEADER ----------- //
                                                    <GridHeaderDataGrid
                                                        row_count_signal
                                                        selected_count_signal
                                                        handle_select_all
                                                        sort_signals
                                                        pinned_columns_signal
                                                        visible_columns_signal
                                                    />

                                                    // ------------- GRID BODY ----------- //
                                                    <GridBody style=grid_body_height>
                                                        <For
                                                            each=move || sorted_rows_signal.get()
                                                            key=|row| row.name.clone()
                                                            children=move |row| {
                                                                let row_name = row.name.clone();
                                                                let index = Signal::derive(move || {
                                                                    sorted_rows_signal
                                                                        .with(|rows| {
                                                                            rows.iter().position(|r| r.name == row_name).unwrap_or(0)
                                                                        })
                                                                });
                                                                let rowindex = Signal::derive(move || index.get() + 2);
                                                                let is_active_signal = RwSignal::new(row.is_active);
                                                                let is_selected = Signal::derive(move || {
                                                                    selected_indices_signal
                                                                        .with(|selected| selected.contains(&index.get()))
                                                                });
                                                                let row = StoredValue::new(row);
                                                                let render_cell_content = move |col: Column| -> AnyView {
                                                                    match col {
                                                                        Column::Website => {
                                                                            let url = row.with_value(|r| r.website.clone());
                                                                            let url_display = url.clone();
                                                                            // Derive index reactively so it updates when rows are deleted
                                                                            // Custom rendering: Website (link), Skills (badges), IsActive (checkbox)
                                                                            // Default: all other columns use GridCellContent with col.get_value()
                                                                            view! {
                                                                                <div
                                                                                    data-slot="grid-cell-content"
                                                                                    class="overflow-hidden size-full"
                                                                                >
                                                                                    <a
                                                                                        href=url
                                                                                        target="_blank"
                                                                                        rel="noopener noreferrer"
                                                                                        class="underline truncate text-primary decoration-primary/30 underline-offset-2 hover:decoration-primary/60"
                                                                                    >
                                                                                        {url_display}
                                                                                    </a>
                                                                                </div>
                                                                            }
                                                                                .into_any()
                                                                        }
                                                                        Column::Skills => {
                                                                            view! {
                                                                                <div class="flex overflow-hidden flex-wrap gap-1 items-center">
                                                                                    {row
                                                                                        .with_value(|r| r.skills.clone())
                                                                                        .into_iter()
                                                                                        .map(|skill| {
                                                                                            view! {
                                                                                                <Badge variant=BadgeVariant::Secondary>{skill}</Badge>
                                                                                            }
                                                                                        })
                                                                                        .collect_view()}
                                                                                </div>
                                                                            }
                                                                                .into_any()
                                                                        }
                                                                        Column::IsActive => {
                                                                            view! {
                                                                                <Checkbox
                                                                                    checked=is_active_signal
                                                                                    on_checked_change=Callback::new(move |checked| {
                                                                                        is_active_signal.set(checked)
                                                                                    })
                                                                                    aria_label="Active"
                                                                                />
                                                                            }
                                                                                .into_any()
                                                                        }
                                                                        Column::Select | Column::Attachments => {
                                                                            view! { <div /> }.into_any()
                                                                        }
                                                                        _ => {
                                                                            let value = row.with_value(|r| col.get_value(r));
                                                                            view! {
                                                                                <EditableCellContent
                                                                                    row_idx=index.get()
                                                                                    col=col
                                                                                    value=value
                                                                                />
                                                                            }
                                                                                .into_any()
                                                                        }
                                                                    }
                                                                };

                                                                // rowindex starts at 2 because row 1 is the header
                                                                view! {
                                                                    <ContextMenu>
                                                                        <ContextMenuTrigger>
                                                                            <GridRow rowindex=rowindex.get() index=index>
                                                                                <GridSelectCell>
                                                                                    <div class="py-1.5 px-3 size-full">
                                                                                        <Checkbox
                                                                                            checked=is_selected
                                                                                            on_checked_change=Callback::new(move |checked| {
                                                                                                let idx = index.get();
                                                                                                selected_indices_signal
                                                                                                    .update(|selected| {
                                                                                                        if checked {
                                                                                                            selected.insert(idx);
                                                                                                        } else {
                                                                                                            selected.remove(&idx);
                                                                                                        }
                                                                                                    });
                                                                                            })
                                                                                            aria_label="Select row"
                                                                                        />
                                                                                    </div>
                                                                                </GridSelectCell>
                                                                                // Pinned cells (dynamic loop) - only show if both pinned AND visible
                                                                                <For
                                                                                    each=move || get_pinned_visible_columns(
                                                                                        pinned_columns_signal,
                                                                                        visible_columns_signal,
                                                                                    )
                                                                                    key=|(col, _)| *col
                                                                                    children=move |(col, _width)| {
                                                                                        view! {
                                                                                            <GridPinnedCell
                                                                                                col=col
                                                                                                pinned_columns_signal=pinned_columns_signal
                                                                                            >
                                                                                                <GridCellWrapper class=col
                                                                                                    .wrapper_class()>
                                                                                                    {render_cell_content(col)}
                                                                                                </GridCellWrapper>
                                                                                            </GridPinnedCell>
                                                                                        }
                                                                                            .into_any()
                                                                                    }
                                                                                />
                                                                                // Non-pinned cells (dynamic loop)
                                                                                <For
                                                                                    each=move || { PINNABLE_COLUMNS.to_vec() }
                                                                                    key=|(col, _)| *col
                                                                                    children=move |(col, _width)| {
                                                                                        view! {
                                                                                            <GridCell
                                                                                                colindex=col.colindex()
                                                                                                column=col.css_safe_name()
                                                                                                visible=col
                                                                                                    .is_visible(pinned_columns_signal, visible_columns_signal)
                                                                                                active=Signal::derive(move || {
                                                                                                    cell_selection.is_active(index.get(), col)
                                                                                                })
                                                                                                current=Signal::derive(move || {
                                                                                                    cell_selection.is_context_menu(index.get(), col)
                                                                                                })
                                                                                                in_range=Signal::derive(move || {
                                                                                                    drag_selection.is_cell_in_range(index.get(), col)
                                                                                                })
                                                                                                on_click=Callback::new(move |_| {
                                                                                                    cell_selection.handle_click(index.get(), col);
                                                                                                    drag_selection.clear_selection();
                                                                                                })
                                                                                                on_contextmenu=Callback::new(move |_| {
                                                                                                    cell_selection.start_contextmenu();
                                                                                                    if drag_selection.handle_contextmenu(index.get(), col) {
                                                                                                        cell_selection.set_active(index.get(), col);
                                                                                                    }
                                                                                                    cell_selection.set_context_menu(index.get(), col);
                                                                                                    let value = row.with_value(|r| col.get_value(r));
                                                                                                    copy_value_signal.set(value);
                                                                                                })
                                                                                                on_mousedown=Callback::new(move |_| {
                                                                                                    cell_selection.set_active(index.get(), col);
                                                                                                    drag_selection.start_drag(index.get(), col);
                                                                                                })
                                                                                                on_mouseenter=Callback::new(move |_| {
                                                                                                    drag_selection.update_drag(index.get(), col);
                                                                                                })
                                                                                            >
                                                                                                <GridCellWrapper class=col
                                                                                                    .wrapper_class()>
                                                                                                    {render_cell_content(col)}
                                                                                                </GridCellWrapper>
                                                                                            </GridCell>
                                                                                        }
                                                                                            .into_any()
                                                                                    }
                                                                                />
                                                                            </GridRow>
                                                                        </ContextMenuTrigger>
                                                                        // Clear cell highlights when context menu closes (unless reopening on another cell)
                                                                        <ContextMenuContent on_close=Callback::new(move |_| {
                                                                            cell_selection.handle_contextmenu_close();
                                                                        })>
                                                                            <ContextMenuGroup>
                                                                                <ContextMenuItem on:click=move |_| {
                                                                                    let value = drag_selection
                                                                                        .collect_selection_values(
                                                                                            &sorted_rows_signal.get(),
                                                                                            PINNABLE_COLUMNS,
                                                                                            |row, col| col.get_value(row),
                                                                                        )
                                                                                        .unwrap_or_else(|| copy_value_signal.get());
                                                                                    if !value.is_empty() {
                                                                                        copy_to_clipboard.with_value(|copy_fn| copy_fn(&value));
                                                                                        expect_toaster().success("Copied to clipboard");
                                                                                    }
                                                                                }>
                                                                                    <ContextMenuAction>
                                                                                        <Copy />
                                                                                        <span>"Copy"</span>
                                                                                    </ContextMenuAction>
                                                                                </ContextMenuItem>
                                                                                <ContextMenuItem>
                                                                                    <ContextMenuAction>
                                                                                        <Scissors />
                                                                                        <span>"Cut"</span>
                                                                                    </ContextMenuAction>
                                                                                </ContextMenuItem>
                                                                                <ContextMenuItem>
                                                                                    <ContextMenuAction>
                                                                                        <Eraser />
                                                                                        <span>"Clear"</span>
                                                                                    </ContextMenuAction>
                                                                                </ContextMenuItem>
                                                                            </ContextMenuGroup>
                                                                            <Separator class="my-1" />
                                                                            <ContextMenuItem class="p-0">
                                                                                <PressHoldDeleteRow
                                                                                    index=index
                                                                                    drag_selection=drag_selection
                                                                                    cell_selection=cell_selection
                                                                                    sorted_rows_signal=sorted_rows_signal
                                                                                    handle_delete_rows=handle_delete_rows
                                                                                />
                                                                            </ContextMenuItem>
                                                                        </ContextMenuContent>
                                                                    </ContextMenu>
                                                                }
                                                            }
                                                        />
                                                        // Add row button at the bottom
                                                        <GridAddRow rows_signal row_count_signal />
                                                    </GridBody>
                                                </Grid>
                                            </GridWrapper>
                                        </div>
                                    }
                                        .into_any()
                                }
                                Err(err) => {
                                    view! {
                                        <div class="py-3 px-4 text-red-700 bg-red-100 rounded border border-red-400">
                                            <p class="font-medium">"Error loading data: " {err.to_string()}</p>
                                        </div>
                                    }
                                        .into_any()
                                }
                            }
                        })
                }}
            </Transition>
        </div>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

#[component]
pub fn ToolbarDataGrid(visible_columns_signal: RwSignal<HashSet<String>>) -> impl IntoView {
    view! {
        <DataGridToolbar class="justify-end">
            // Column visibility multi-select
            <MultiSelect values=visible_columns_signal align=MultiSelectAlign::End>
                <MultiSelectTrigger>
                    <Settings2 class="text-muted-foreground" />
                    <span>"View"</span>
                </MultiSelectTrigger>
                <MultiSelectContent>
                    <MultiSelectGroup>
                        {Column::iter()
                            .filter(|c| *c != Column::Select)
                            .map(|column| {
                                let column_str = column.to_string();
                                let is_disabled = column.is_disabled();
                                view! {
                                    <MultiSelectItem>
                                        <MultiSelectOption value=column_str.clone() attr:disabled=is_disabled>
                                            {column_str}
                                        </MultiSelectOption>
                                    </MultiSelectItem>
                                }
                            })
                            .collect_view()}
                    </MultiSelectGroup>
                </MultiSelectContent>
            </MultiSelect>
        </DataGridToolbar>
    }
}

#[component]
pub fn GridHeaderDataGrid(
    row_count_signal: Signal<usize>,
    selected_count_signal: Signal<usize>,
    handle_select_all: Callback<bool>,
    sort_signals: StoredValue<std::collections::HashMap<Column, RwSignal<SortDirection>>>,
    pinned_columns_signal: RwSignal<HashSet<Column>>,
    visible_columns_signal: RwSignal<HashSet<String>>,
) -> impl IntoView {
    view! {
        <div role="rowgroup" data-slot="grid-header" class="grid sticky top-0 z-10 border-b bg-background">
            <div role="row" aria-rowindex="1" data-slot="grid-header-row" tabindex="-1" class="flex w-full">
                <GridSelectHeaderCell>
                    <div class="py-1.5 px-3 size-full">
                        <Checkbox
                            checked=Signal::derive(move || {
                                let row_count = row_count_signal.get();
                                row_count > 0 && selected_count_signal.get() == row_count
                            })
                            on_checked_change=handle_select_all
                            aria_label="Select all"
                        />
                    </div>
                </GridSelectHeaderCell>
                // Pinned headers (dynamic loop) - only show if both pinned AND visible
                <For
                    each=move || get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal)
                    key=|(col, _)| *col
                    children=move |(col, _width)| {
                        let sort_signal = sort_signals.with_value(|s| s.get(&col).copied());
                        let Some(sort_signal) = sort_signal else {
                            return view! { <div></div> }.into_any();
                        };
                        let left = pinned_columns_signal.with(|p| get_pinned_left_position(col, p));
                        let width = get_column_width(col);

                        view! {
                            <GridPinnedHeaderCell left=left width=width>
                                <PinnableSortableHeaderCell
                                    column=col
                                    label=col.to_string()
                                    sort_signal=sort_signal
                                    pinned_columns_signal=pinned_columns_signal
                                    visible_columns_signal=visible_columns_signal
                                    is_pinned=true
                                />
                            </GridPinnedHeaderCell>
                        }
                            .into_any()
                    }
                />
                // Non-pinned headers (dynamic loop)
                <For
                    each=move || PINNABLE_COLUMNS.to_vec()
                    key=|(col, _)| *col
                    children=move |(col, _width)| {
                        let sort_signal = sort_signals.with_value(|s| s.get(&col).copied());
                        let Some(sort_signal) = sort_signal else {
                            return view! { <div></div> }.into_any();
                        };
                        view! {
                            <GridHeaderCell
                                colindex=col.colindex()
                                column=col.css_safe_name()
                                visible=col.is_visible(pinned_columns_signal, visible_columns_signal)
                            >
                                <PinnableSortableHeaderCell
                                    column=col
                                    label=col.to_string()
                                    sort_signal=sort_signal
                                    pinned_columns_signal=pinned_columns_signal
                                    visible_columns_signal=visible_columns_signal
                                />
                            </GridHeaderCell>
                        }
                            .into_any()
                    }
                />
            // grid-header-row
            </div>
        </div>
    }
}

#[component]
fn PressHoldDeleteRow(
    index: Signal<usize>,
    drag_selection: crate::hooks::use_drag_selection::UseDragSelection<Column>,
    cell_selection: crate::hooks::use_cell_selection::UseCellSelection<Column>,
    sorted_rows_signal: Memo<Vec<RowData>>,
    handle_delete_rows: Callback<Vec<String>>,
) -> impl IntoView {
    let on_delete = Callback::new(move |_| {
        let idx = index.get();
        let (min_row, max_row) =
            drag_selection.get_selection_bounds().map(|(min, max, _, _)| (min, max)).unwrap_or((idx, idx));

        let sorted = sorted_rows_signal.get();
        let names_to_delete: Vec<String> =
            (min_row..=max_row).filter_map(|i| sorted.get(i).map(|r| r.name.clone())).collect();

        let count = names_to_delete.len();
        handle_delete_rows.run(names_to_delete);
        drag_selection.clear_selection();
        cell_selection.clear_all();

        let suffix = if count == 1 { "" } else { "s" };
        expect_toaster().success(format!("Deleted {} row{}", count, suffix));
    });

    let disabled = Signal::derive(|| false);
    let press_hold = use_press_hold(1500, on_delete, disabled);

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold.clone();

    let progress_style = move || {
        let width_percent = press_hold.progress_signal.get() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(239, 68, 68, 0.3); pointer-events: none; border-radius: inherit;"
        )
    };

    let is_multi_row =
        move || drag_selection.get_selection_bounds().is_some_and(|(min_row, max_row, _, _)| max_row > min_row);

    view! {
        <button
            class="flex relative gap-2 items-center py-1.5 px-2 w-full text-sm rounded-sm transition-colors select-none text-destructive hover:bg-destructive/10"
            on:pointerdown=move |_| ph1.on_pointer_down()
            on:pointerup=move |_| ph2.on_pointer_up()
            on:pointerleave=move |_| ph3.on_pointer_up()
            on:pointercancel=move |_| ph4.on_pointer_up()
        >
            <span style=progress_style></span>
            <span class="flex relative z-10 gap-2 items-center">
                <Trash2 class="size-4" />
                {move || if is_multi_row() { "Hold to delete rows" } else { "Hold to delete row" }}
            </span>
        </button>
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

/// Add row button at the bottom of the grid using absolute positioning.
#[component]
fn GridAddRow(rows_signal: RwSignal<Vec<RowData>>, row_count_signal: Signal<usize>) -> impl IntoView {
    let rowindex = Signal::derive(move || row_count_signal.get() + 2);

    let handle_add_row = move |_| {
        rows_signal.update(|rows| {
            // Generate unique name for new row
            let new_row_num = rows.len() + 1;
            let new_name = format!("New Row {new_row_num}");
            rows.push(RowData { name: new_name, ..Default::default() });
        });
    };

    view! {
        <div
            role="row"
            aria-rowindex=move || rowindex.get().to_string()
            data-slot="grid-add-row"
            tabindex="-1"
            class="flex absolute w-full border-b transition-colors cursor-pointer bg-muted/30 hover:bg-muted/50"
            style=move || {
                let translate_y = row_count_signal.get() * 36;
                format!("height: 36px; transform: translateY({translate_y}px);")
            }
            on:click=handle_add_row
        >
            <div class="flex sticky left-0 gap-2 items-center px-3 text-muted-foreground">
                <Plus class="size-3.5" />
                <span class="text-sm">"Add row"</span>
            </div>
        </div>
    }
}
