use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use super::popup::popup_area;

/// Renders a confirmation dialog with Cancel and Confirm buttons
pub fn draw_confirm_dialog(
    frame: &mut Frame,
    items: &[String],
    title: &str,
    item_type: &str,
    confirm_focused: bool,
    area: Rect,
) {
    let popup_rect = popup_area(area, 50, 60);

    // Clear the background
    frame.render_widget(Clear, popup_rect);

    // Main block with title
    let block = Block::bordered().title(title).style(Style::default().fg(Color::White));
    let inner = block.inner(popup_rect);
    frame.render_widget(block, popup_rect);

    // Split inner area: content on top, buttons at bottom
    let chunks = Layout::vertical([Constraint::Min(3), Constraint::Length(3)]).split(inner);

    let (Some(&content_area), Some(&button_area)) = (chunks.first(), chunks.get(1)) else {
        return;
    };

    // Content: list of items
    let item_count = items.len();
    let item_type_plural = if item_count == 1 { item_type } else { &format!("{item_type}s") };

    let mut lines: Vec<Line> = vec![
        Line::from(Span::styled(
            format!("Add {item_count} {item_type_plural}?"),
            Style::default().fg(Color::Yellow),
        )),
        Line::from(""),
    ];

    // Add items (limit display if too many)
    let max_display = 12;
    for (i, item) in items.iter().enumerate() {
        if i >= max_display {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", item_count - max_display),
                Style::default().fg(Color::DarkGray),
            )));
            break;
        }
        lines.push(Line::from(Span::styled(format!("  • {item}"), Style::default().fg(Color::White))));
    }

    let content = Paragraph::new(lines).wrap(Wrap { trim: true });
    frame.render_widget(content, content_area);

    // Buttons - styled as bordered buttons
    let cancel_style = if !confirm_focused {
        Style::default().fg(Color::Black).bg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let confirm_style = if confirm_focused {
        Style::default().fg(Color::Black).bg(Color::Green).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };

    let button_line = Line::from(vec![
        Span::styled(if !confirm_focused { "│" } else { " " }, cancel_style),
        Span::styled(" Cancel ", cancel_style),
        Span::styled(if !confirm_focused { "│" } else { " " }, cancel_style),
        Span::raw("   "),
        Span::styled(if confirm_focused { "│" } else { " " }, confirm_style),
        Span::styled(" Confirm ", confirm_style),
        Span::styled(if confirm_focused { "│" } else { " " }, confirm_style),
    ]);

    let buttons = Paragraph::new(button_line).centered();
    frame.render_widget(buttons, button_area);
}
