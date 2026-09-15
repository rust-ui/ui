use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, List, ListItem, Scrollbar, ScrollbarOrientation};

use super::super::app::App;
use super::super::widgets::checked_popup::draw_confirm_dialog;
use super::super::widgets::detail_panel::draw_detail_panel;
use super::super::widgets::helpers::{filter_items, get_item_at_visual_index, get_selected_item};
use super::super::widgets::search_input::draw_search_input;

pub fn draw_tab_components(frame: &mut Frame, app: &mut App, area: Rect) {
    // Horizontal split: sidenav on left, detail on right
    let horizontal_chunks =
        Layout::horizontal([Constraint::Percentage(35), Constraint::Percentage(65)]).split(area);

    let (Some(&left_panel), Some(&right_panel)) = (horizontal_chunks.first(), horizontal_chunks.get(1))
    else {
        return;
    };

    // Split left panel vertically: search input at top, list below
    let left_chunks = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).split(left_panel);

    let (Some(&search_area), Some(&list_area)) = (left_chunks.first(), left_chunks.get(1)) else {
        return;
    };

    // Filter components based on search query (prefix matching)
    let components_refs: Vec<&str> = app.components.iter().map(|s| s.as_str()).collect();
    let filtered_components = filter_items(&components_refs, &app.components_search_query);

    // Ensure scroll doesn't exceed filtered list bounds
    if !filtered_components.is_empty() && app.components_scroll >= filtered_components.len() {
        app.components_scroll = filtered_components.len().saturating_sub(1);
    }

    // Update scrollbar state with filtered content length
    app.components_scroll_state = app.components_scroll_state.content_length(filtered_components.len());

    // Left side: Component list
    let items: Vec<ListItem> = filtered_components
        .iter()
        .map(|component| {
            let is_installed = app.installed.contains(*component);
            let is_checked = app.components_checked.contains(*component);

            let (icon, color) = if is_checked {
                ("☑", Color::Green) // Selected
            } else if is_installed {
                ("✓", Color::Cyan) // Already installed (not selected)
            } else {
                ("☐", Color::DarkGray) // Not selected
            };

            let suffix = if is_installed { " (installed)" } else { "" };
            ListItem::new(Span::styled(format!("  {icon} {component}{suffix}"), Style::default().fg(color)))
        })
        .collect();

    let checked_count = app.components_checked.len();
    let installed_count = app.components.iter().filter(|c| app.installed.contains(*c)).count();

    let title = {
        let base = if app.components_search_query.is_empty() {
            format!("Components ({})", app.components.len())
        } else {
            format!("Components ({}/{})", filtered_components.len(), app.components.len())
        };

        let mut parts = vec![base];
        if installed_count > 0 {
            parts.push(format!("{installed_count} installed"));
        }
        if checked_count > 0 {
            parts.push(format!("{checked_count} selected"));
        }
        parts.join(" · ")
    };

    let list = List::new(items)
        .block(Block::bordered().title(title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    // Update list state
    if !filtered_components.is_empty() {
        app.components_list_state.select(Some(app.components_scroll));
    }

    // Draw search input in left panel
    draw_search_input(frame, &app.components_search_query, app.components_search_active, search_area);

    // Render list in left panel
    frame.render_stateful_widget(list, list_area, &mut app.components_list_state);

    // Render scrollbar in left panel
    frame.render_stateful_widget(
        Scrollbar::new(ScrollbarOrientation::VerticalRight).begin_symbol(Some("↑")).end_symbol(Some("↓")),
        list_area,
        &mut app.components_scroll_state,
    );

    // Right side: Detail panel
    let selected_component = filtered_components.get(app.components_scroll).copied();
    let dependencies = selected_component.and_then(|c| app.get_dependencies(c));
    draw_detail_panel(frame, selected_component, app.components_checked.len(), "component", dependencies, right_panel);

    // Render confirmation dialog if show_popup is true and there are checked components
    if app.show_popup && !app.components_checked.is_empty() {
        let mut checked_list: Vec<String> = app.components_checked.iter().cloned().collect();
        checked_list.sort();
        draw_confirm_dialog(
            frame,
            &checked_list,
            " Add Components ",
            "component",
            app.popup_confirm_focused,
            area,
        );
    }
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

pub fn get_selected_component(app: &App) -> Option<String> {
    let components_refs: Vec<&str> = app.components.iter().map(|s| s.as_str()).collect();
    get_selected_item(&components_refs, app.components_scroll, &app.components_search_query)
        .map(|s| s.to_string())
}

pub fn get_component_at_visual_index(app: &App, visual_index: usize) -> Option<String> {
    let components_refs: Vec<&str> = app.components.iter().map(|s| s.as_str()).collect();
    get_item_at_visual_index(&components_refs, visual_index, &app.components_search_query)
        .map(|s| s.to_string())
}
