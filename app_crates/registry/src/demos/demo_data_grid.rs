use std::collections::HashSet;
use std::sync::LazyLock;

use dioxus::prelude::*;
use icons::{Copy, Eraser, Plus, Scissors, Settings2, Trash2};
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
use crate::ui::skeleton::Skeleton;
use crate::ui::toast_custom::toaster::expect_toaster;

/* ========================================================== */
/*                     TYPES                                  */
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

    fn set_value(self, row: &mut RowData, value: String) {
        match self {
            Self::Name => row.name = value,
            Self::Age => row.age = value.parse().unwrap_or_default(),
            Self::Email => row.email = value,
            Self::Website => row.website = value,
            Self::Notes => row.notes = value,
            Self::Salary => row.salary = value.parse().unwrap_or_default(),
            Self::Department => row.department = value,
            Self::Status => row.status = value,
            Self::Skills => {
                row.skills =
                    value.split(',').map(str::trim).filter(|item| !item.is_empty()).map(ToOwned::to_owned).collect();
            }
            Self::IsActive => {
                let normalized = value.trim().to_ascii_lowercase();
                row.is_active = matches!(normalized.as_str(), "true" | "1" | "yes" | "on");
            }
            Self::StartDate => row.start_date = value,
            Self::Select | Self::Attachments => {}
        }
    }

    #[allow(dead_code)]
    fn clear_value(self, row: &mut RowData) {
        match self {
            Self::Name => row.name.clear(),
            Self::Age => row.age = 0,
            Self::Email => row.email.clear(),
            Self::Website => row.website.clear(),
            Self::Notes => row.notes.clear(),
            Self::Salary => row.salary = 0,
            Self::Department => row.department.clear(),
            Self::Status => row.status.clear(),
            Self::Skills => row.skills.clear(),
            Self::IsActive => row.is_active = false,
            Self::StartDate => row.start_date.clear(),
            Self::Select | Self::Attachments => {}
        }
    }
}

static GRID_STYLE: LazyLock<String> = LazyLock::new(generate_grid_style::<Column>);

/* ========================================================== */
/*                     ✨ SERVER FUNCTION ✨                  */
/* ========================================================== */

pub async fn get_data_grid_rows() -> Result<Vec<RowData>, ServerFnError> {
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

/* ========================================================== */
/*                     FUNCTIONS                              */
/* ========================================================== */

#[component]
pub fn DemoDataGrid() -> Element {
    rsx! { DataGridFull {} }
}

#[component]
pub fn DataGridFull() -> Element {
    let rows_resource = use_resource(get_data_grid_rows);
    let mut rows_signal = use_signal(Vec::<RowData>::new);
    let mut selected_indices_signal = use_signal(HashSet::<usize>::new);

    let ColumnState { sort_signals, pinned_columns_signal, visible_columns_signal } =
        use_column_state::<Column>(PINNABLE_COLUMNS);

    let DataGridState { cell_selection, mut drag_selection, mut copy_value_signal, mut grid_wrapper_element } =
        use_data_grid_state::<Column>();

    let copy_to_clipboard = use_copy_clipboard(None).0;
    let _cell_edit = use_cell_edit::<Column>();

    // Sync loaded rows into mutable rows_signal (matches Leptos Resource → RwSignal pattern)
    use_effect(move || {
        if let Some(Ok(rows)) = &*rows_resource.read() {
            rows_signal.set(rows.clone());
        }
    });

    let sort_signals_for_rows = sort_signals.clone();
    let sorted_rows_signal = use_memo(move || {
        let mut rows: Vec<RowData> = rows_signal();
        for (col, _) in PINNABLE_COLUMNS {
            if let Some(signal) = sort_signals_for_rows.get(col) {
                col.sort_rows(&mut rows, signal());
            }
        }
        rows
    });

    let row_count_signal = use_memo(move || rows_signal.read().len());
    let grid_body_height = use_memo(move || format!("height: {}px", (row_count_signal() + 1) * 36));
    let selected_count_signal = use_memo(move || selected_indices_signal.read().len());

    let handle_select_all = move |checked: bool| {
        let row_count = row_count_signal();
        selected_indices_signal.with_mut(|selected| {
            if checked {
                for index in 0..row_count {
                    selected.insert(index);
                }
            } else {
                selected.clear();
            }
        });
    };

    let handle_delete_rows = move |names: Vec<String>| {
        rows_signal.with_mut(|rows| {
            rows.retain(|r| !names.contains(&r.name));
        });
    };

    rsx! {
        div { class: "container flex flex-col py-4",
            ToolbarDataGrid { visible_columns_signal }

            match &*rows_resource.read() {
                None => rsx! {
                    div { class: "rounded-md border overflow-hidden",
                        // Header skeleton
                        div { class: "flex items-center gap-3 border-b px-3 h-9 bg-muted/40",
                            Skeleton { class: "size-4 rounded" }
                            Skeleton { class: "h-4 w-24" }
                            Skeleton { class: "h-4 w-8" }
                            Skeleton { class: "h-4 w-36" }
                            Skeleton { class: "h-4 w-28" }
                            Skeleton { class: "h-4 w-20" }
                            Skeleton { class: "h-4 w-16" }
                        }
                        // Row skeletons
                        for _ in 0..14u32 {
                            div { class: "flex items-center gap-3 border-b last:border-0 px-3 h-9",
                                Skeleton { class: "size-4 rounded shrink-0" }
                                Skeleton { class: "h-4 w-24 shrink-0" }
                                Skeleton { class: "h-4 w-8 shrink-0" }
                                Skeleton { class: "h-4 w-36 shrink-0" }
                                Skeleton { class: "h-4 w-28 shrink-0" }
                                Skeleton { class: "h-4 w-16 shrink-0" }
                                Skeleton { class: "h-4 w-20 shrink-0" }
                            }
                        }
                    }
                },
                Some(Err(_)) => rsx! { p { "Error loading data." } },
                Some(Ok(_)) => rsx! {
            div {
                onmounted: move |event| {
                    if let Some(element) = event.data().downcast::<web_sys::Element>().cloned() {
                        grid_wrapper_element.set(Some(element));
                    }
                },
                onmouseup: move |_| drag_selection.stop_dragging(),
                onmouseleave: move |_| drag_selection.stop_dragging(),
                GridWrapper {
                    button {
                        r#type: "button",
                        id: "radix-_R_mlubsqlb_",
                        "aria-haspopup": "menu",
                        "aria-expanded": "false",
                        "data-state": "closed",
                        "data-slot": "dropdown-menu-trigger",
                        style: "position:fixed;left:0px;top:0px;width:1px;height:1px;padding:0;margin:0;border:none;background:transparent;pointer-events:none;opacity:0",
                    }
                    Grid {
                        rowcount: Signal::new(i32::try_from(row_count_signal()).unwrap_or(0) + 1),
                        colcount: i32::try_from(Column::iter().count()).unwrap_or(0),
                        style: GRID_STYLE.as_str(),

                        GridHeaderDataGrid {
                            row_count_signal: Signal::new(row_count_signal()),
                            selected_count_signal: Signal::new(selected_count_signal()),
                            handle_select_all,
                            sort_signals: sort_signals.clone(),
                            pinned_columns_signal,
                            visible_columns_signal,
                        }

                        GridBody { style: Signal::new(grid_body_height()),
                            for (index, row) in sorted_rows_signal().into_iter().enumerate() {
                                {
                                    let index_signal = Signal::new(index);
                                    let rowindex = index + 2;
                                    let is_selected = selected_indices_signal.read().contains(&index);
                                    let row_for_render = row.clone();
                                    let row_for_cells = row.clone();
                                    let copy_to_clipboard = copy_to_clipboard.clone();
                                    let row_name_for_save = row.name.clone();

                                    let render_cell_content = move |col: Column| -> Element {
                                        match col {
                                            Column::Website => {
                                                let url = row_for_render.website.clone();
                                                let url_display = url.clone();
                                                rsx! {
                                                    div {
                                                        "data-slot": "grid-cell-content",
                                                        class: "overflow-hidden size-full",
                                                        a {
                                                            href: "{url}",
                                                            target: "_blank",
                                                            rel: "noopener noreferrer",
                                                            class: "underline truncate text-primary decoration-primary/30 underline-offset-2 hover:decoration-primary/60",
                                                            "{url_display}"
                                                        }
                                                    }
                                                }
                                            }
                                            Column::Skills => {
                                                let skills = row_for_render.skills.clone();
                                                rsx! {
                                                    div { class: "flex overflow-hidden flex-wrap gap-1 items-center",
                                                        for skill in skills {
                                                            Badge { variant: BadgeVariant::Secondary, "{skill}" }
                                                        }
                                                    }
                                                }
                                            }
                                            Column::IsActive => {
                                                let row_name = row_for_render.name.clone();
                                                rsx! {
                                                    Checkbox {
                                                        checked: row_for_render.is_active,
                                                        on_checked_change: move |checked| {
                                                            rows_signal.with_mut(|rows| {
                                                                if let Some(target) = rows.iter_mut().find(|r| r.name == row_name) {
                                                                    target.is_active = checked;
                                                                }
                                                            });
                                                        },
                                                        aria_label: "Active",
                                                    }
                                                }
                                            }
                                            Column::Select | Column::Attachments => rsx! { div {} },
                                            _ => {
                                                let value = col.get_value(&row_for_render);
                                                let row_name_for_save = row_name_for_save.clone();
                                                rsx! {
                                                    EditableCellContent {
                                                        row_idx: index,
                                                        col,
                                                        value,
                                                        on_save: move |(row_idx, column, new_value): (usize, Column, String)| {
                                                            let _ = row_idx;
                                                            rows_signal.with_mut(|rows| {
                                                                if let Some(target) = rows.iter_mut().find(|r| r.name == row_name_for_save) {
                                                                    column.set_value(target, new_value);
                                                                }
                                                            });
                                                        },
                                                    }
                                                }
                                            }
                                        }
                                    };

                                    let copy_to_clipboard = copy_to_clipboard.clone();

                                    rsx! {
                                        ContextMenu {
                                            ContextMenuTrigger {
                                                on_open: move |_| {
                                                    cell_selection.clone().start_contextmenu();
                                                },
                                                GridRow { rowindex, index: index_signal,
                                                    GridSelectCell {
                                                        div { class: "py-1.5 px-3 size-full",
                                                            Checkbox {
                                                                checked: is_selected,
                                                                on_checked_change: move |checked| {
                                                                    selected_indices_signal.with_mut(|selected| {
                                                                        if checked { selected.insert(index); } else { selected.remove(&index); }
                                                                    });
                                                                },
                                                                aria_label: "Select row",
                                                            }
                                                        }
                                                    }

                                                    for (col, _width) in get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal) {
                                                        GridPinnedCell { col, pinned_columns_signal,
                                                            GridCellWrapper { class: col.wrapper_class(),
                                                                {render_cell_content(col)}
                                                            }
                                                        }
                                                    }

                                                    for (col, _width) in PINNABLE_COLUMNS.iter().copied() {
                                                        {
                                                            let row_for_copy = row_for_cells.clone();
                                                            rsx! {
                                                                GridCell {
                                                                    colindex: col.colindex(),
                                                                    column: col.css_safe_name(),
                                                                    visible: col.is_visible(pinned_columns_signal, visible_columns_signal),
                                                                    active: cell_selection.clone().is_active(index, col),
                                                                    current: cell_selection.clone().is_context_menu(index, col),
                                                                    in_range: drag_selection.is_cell_in_range(index, col),
                                                                    on_click: move |_| {
                                                                        cell_selection.clone().handle_click(index, col);
                                                                        drag_selection.clear_selection();
                                                                    },
                                                                    on_contextmenu: move |_| {
                                                                        if drag_selection.handle_contextmenu(index, col) {
                                                                            cell_selection.clone().set_active(index, col);
                                                                        }
                                                                        cell_selection.clone().set_context_menu(index, col);
                                                                        copy_value_signal.set(col.get_value(&row_for_copy));
                                                                    },
                                                                    on_mousedown: move |_| {
                                                                        cell_selection.clone().set_active(index, col);
                                                                        drag_selection.start_drag(index, col);
                                                                    },
                                                                    on_mouseenter: move |_| {
                                                                        drag_selection.update_drag(index, col);
                                                                    },
                                                                    GridCellWrapper { class: col.wrapper_class(),
                                                                        {render_cell_content(col)}
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            ContextMenuContent {
                                                on_close: move |_| {
                                                    cell_selection.clone().handle_contextmenu_close();
                                                },
                                                ContextMenuGroup {
                                                    ContextMenuItem {
                                                        ContextMenuAction {
                                                            onclick: move |_| {
                                                                let value = drag_selection
                                                                    .collect_selection_values(&sorted_rows_signal(), PINNABLE_COLUMNS, |row, col| col.get_value(row))
                                                                    .unwrap_or_else(|| copy_value_signal());
                                                                if !value.is_empty() {
                                                                    copy_to_clipboard(&value);
                                                                    expect_toaster().success("Copied to clipboard");
                                                                }
                                                            },
                                                            Copy {}
                                                            span { "Copy" }
                                                        }
                                                    }
                                                    ContextMenuItem {
                                                        ContextMenuAction {
                                                            Scissors {}
                                                            span { "Cut" }
                                                        }
                                                    }
                                                    ContextMenuItem {
                                                        ContextMenuAction {
                                                            Eraser {}
                                                            span { "Clear" }
                                                        }
                                                    }
                                                }
                                                Separator { class: "my-1" }
                                                ContextMenuItem { class: "p-0",
                                                    PressHoldDeleteRow {
                                                        index: index_signal,
                                                        drag_selection,
                                                        cell_selection,
                                                        sorted_rows_signal: Signal::new(sorted_rows_signal()),
                                                        handle_delete_rows,
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            GridAddRow { rows_signal, row_count_signal: Signal::new(row_count_signal()) }
                        }
                    }
                }
            }
            } // closes Some(Ok(_)) => rsx!
        } // closes match
        }
    }
}

#[component]
pub fn ToolbarDataGrid(visible_columns_signal: Signal<HashSet<String>>) -> Element {
    rsx! {
        DataGridToolbar { class: "justify-end",
            MultiSelect { values: visible_columns_signal, align: MultiSelectAlign::End,
                MultiSelectTrigger {
                    Settings2 { class: "text-muted-foreground" }
                    span { "View" }
                }
                MultiSelectContent {
                    MultiSelectGroup {
                        for column in Column::iter().filter(|c| *c != Column::Select) {
                            {
                                let column_str = column.to_string();
                                let is_disabled = column.is_disabled();
                                rsx! {
                                    MultiSelectItem {
                                        MultiSelectOption { value: column_str.clone(), disabled: is_disabled,
                                            "{column_str}"
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
pub fn GridHeaderDataGrid(
    row_count_signal: Signal<usize>,
    selected_count_signal: Signal<usize>,
    handle_select_all: EventHandler<bool>,
    sort_signals: std::collections::HashMap<Column, Signal<SortDirection>>,
    pinned_columns_signal: Signal<HashSet<Column>>,
    visible_columns_signal: Signal<HashSet<String>>,
) -> Element {
    let all_checked = {
        let row_count = row_count_signal();
        row_count > 0 && selected_count_signal() == row_count
    };

    rsx! {
        div { role: "rowgroup", "data-slot": "grid-header", class: "grid sticky top-0 z-10 border-b bg-background",
            div { role: "row", "aria-rowindex": "1", "data-slot": "grid-header-row", tabindex: "-1", class: "flex w-full",
                GridSelectHeaderCell {
                    div { class: "py-1.5 px-3 size-full",
                        Checkbox {
                            checked: all_checked,
                            on_checked_change: move |checked| handle_select_all.call(checked),
                            aria_label: "Select all",
                        }
                    }
                }

                for (col, _width) in get_pinned_visible_columns(pinned_columns_signal, visible_columns_signal) {
                    {
                        let sort_signal = sort_signals.get(&col).copied();
                        let left = get_pinned_left_position(col, &pinned_columns_signal.read());
                        let width = get_column_width(col);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridPinnedHeaderCell { left, width,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                        is_pinned: true,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }

                for (col, _width) in PINNABLE_COLUMNS.iter().copied() {
                    {
                        let sort_signal = sort_signals.get(&col).copied();
                        let is_visible = col.is_visible(pinned_columns_signal, visible_columns_signal);
                        if let Some(sort_signal) = sort_signal {
                            rsx! {
                                GridHeaderCell {
                                    colindex: col.colindex(),
                                    column: col.css_safe_name(),
                                    visible: is_visible,
                                    PinnableSortableHeaderCell {
                                        column: col,
                                        label: col.to_string(),
                                        sort_signal,
                                        pinned_columns_signal,
                                        visible_columns_signal,
                                    }
                                }
                            }
                        } else {
                            rsx! { div {} }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn PressHoldDeleteRow(
    index: Signal<usize>,
    drag_selection: crate::hooks::use_drag_selection::UseDragSelection<Column>,
    cell_selection: crate::hooks::use_cell_selection::UseCellSelection<Column>,
    sorted_rows_signal: Signal<Vec<RowData>>,
    handle_delete_rows: EventHandler<Vec<String>>,
) -> Element {
    let on_delete = Callback::new(move |_: ()| {
        let idx = index();
        let (min_row, max_row) =
            drag_selection.get_selection_bounds().map(|(min, max, _, _)| (min, max)).unwrap_or((idx, idx));

        let sorted = sorted_rows_signal();
        let names_to_delete: Vec<String> =
            (min_row..=max_row).filter_map(|i| sorted.get(i).map(|r| r.name.clone())).collect();

        let count = names_to_delete.len();
        handle_delete_rows.call(names_to_delete);
        drag_selection.clear_selection();
        cell_selection.clone().clear_all();

        let suffix = if count == 1 { "" } else { "s" };
        expect_toaster().success(format!("Deleted {} row{}", count, suffix));
    });

    let press_hold = use_press_hold(1500, on_delete, false);

    let ph1 = press_hold.clone();
    let ph2 = press_hold.clone();
    let ph3 = press_hold.clone();
    let ph4 = press_hold.clone();

    let progress_style = move || {
        let width_percent = (press_hold.progress_signal)() * 100.0;
        format!(
            "position: absolute; left: 0; top: 0; bottom: 0; width: {width_percent:.1}%; background: rgba(239, 68, 68, 0.3); pointer-events: none; border-radius: inherit;"
        )
    };

    let is_multi_row =
        move || drag_selection.get_selection_bounds().is_some_and(|(min_row, max_row, _, _)| max_row > min_row);

    rsx! {
        button {
            class: "flex relative gap-2 items-center py-1.5 px-2 w-full text-sm rounded-sm transition-colors select-none text-destructive hover:bg-destructive/10",
            onpointerdown: move |_| ph1.on_pointer_down(),
            onpointerup: move |_| ph2.on_pointer_up(),
            onpointerleave: move |_| ph3.on_pointer_up(),
            onpointercancel: move |_| ph4.on_pointer_up(),
            span { style: "{progress_style()}" }
            span { class: "flex relative z-10 gap-2 items-center",
                Trash2 { class: "size-4" }
                if is_multi_row() { "Hold to delete rows" } else { "Hold to delete row" }
            }
        }
    }
}

#[component]
fn GridAddRow(mut rows_signal: Signal<Vec<RowData>>, row_count_signal: Signal<usize>) -> Element {
    let rowindex = row_count_signal() + 2;
    let translate_y = row_count_signal() * 36;

    rsx! {
        div {
            role: "row",
            "aria-rowindex": "{rowindex}",
            "data-slot": "grid-add-row",
            tabindex: "-1",
            class: "flex absolute w-full border-b transition-colors cursor-pointer bg-muted/30 hover:bg-muted/50",
            style: "height: 36px; transform: translateY({translate_y}px);",
            onclick: move |_| {
                rows_signal.with_mut(|rows| {
                    let new_row_num = rows.len() + 1;
                    let new_name = format!("New Row {new_row_num}");
                    rows.push(RowData { name: new_name, ..Default::default() });
                });
            },
            div { class: "flex sticky left-0 gap-2 items-center px-3 text-muted-foreground",
                Plus { class: "size-3.5" }
                span { class: "text-sm", "Add row" }
            }
        }
    }
}
