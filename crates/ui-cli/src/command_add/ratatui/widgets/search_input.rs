use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

/// Renders a search input widget with icon and cursor
pub fn draw_search_input(frame: &mut Frame, search_query: &str, search_active: bool, area: Rect) {
    let search_style = if search_active {
        Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::DarkGray)
    };

    let cursor = if search_active { "│" } else { "" };
    let text = if search_query.is_empty() && !search_active {
        "Press / to search...".to_string()
    } else {
        format!("{}{}", search_query, cursor)
    };

    let input = Paragraph::new(Line::from(vec![
        Span::styled("🔍 ", Style::default().fg(Color::Gray)),
        Span::styled(text, search_style),
    ]))
    .block(Block::bordered().title("Search"));

    frame.render_widget(input, area);
}
