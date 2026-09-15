use ratatui::Frame;
use ratatui::layout::{Alignment, Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, Paragraph};

use super::super::app::App;

pub fn draw_tab_settings(frame: &mut Frame, _app: &mut App, area: Rect) {
    let block = Block::bordered().title("Settings").style(Style::default().fg(Color::White));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let layout = Layout::vertical([Constraint::Percentage(50)]).split(inner_area);

    let Some(&content_area) = layout.first() else {
        return;
    };

    let coming_soon = Paragraph::new("Coming soon")
        .alignment(Alignment::Center)
        .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(coming_soon, content_area);
}
