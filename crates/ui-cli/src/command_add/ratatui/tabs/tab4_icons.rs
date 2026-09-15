use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style};
use ratatui::text::Span;
use ratatui::widgets::{Block, List, ListItem, ListState, Paragraph};

use super::super::app::App;

const ICON_ITEMS: &[&str] = &["A Arrow Up", "Alarm Clock Check"];

pub fn draw_tab_icons(frame: &mut Frame, app: &mut App, area: Rect) {
    // Horizontal flex layout: list on left, content on right
    let chunks = Layout::horizontal([Constraint::Percentage(30), Constraint::Percentage(70)]).split(area);

    let (Some(&left_panel), Some(&right_panel)) = (chunks.first(), chunks.get(1)) else {
        return;
    };

    // Left panel: list of icons
    let items: Vec<ListItem> = ICON_ITEMS
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.icons_selected {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };
            ListItem::new(Span::styled(format!("  • {item}"), style))
        })
        .collect();

    let list = List::new(items)
        .block(Block::bordered().title("Icons"))
        .highlight_style(Style::default().fg(Color::White));

    let mut state = ListState::default();
    state.select(Some(app.icons_selected));

    frame.render_stateful_widget(list, left_panel, &mut state);

    // Right panel: icon preview
    let content_block = Block::bordered().title("Preview");
    let inner_area = content_block.inner(right_panel);
    frame.render_widget(content_block, right_panel);

    let filename = match app.icons_selected {
        0 => "a_arrow_up.svg",
        1 => "alarm_clock_check.svg",
        _ => "",
    };

    if !filename.is_empty() {
        match svg_to_halfblocks(filename) {
            Some(text) => {
                let paragraph = Paragraph::new(text).style(Style::default().fg(Color::Rgb(255, 165, 0)));
                frame.render_widget(paragraph, inner_area);
            }
            None => {
                let paragraph = Paragraph::new(format!("Failed to load {filename}"))
                    .style(Style::default().fg(Color::Red));
                frame.render_widget(paragraph, inner_area);
            }
        }
    } else {
        let paragraph = Paragraph::new("Select an icon").style(Style::default().fg(Color::Gray));
        frame.render_widget(paragraph, inner_area);
    }
}

fn svg_to_halfblocks(filename: &str) -> Option<String> {
    let svg_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(filename);
    let svg_data = std::fs::read(&svg_path).ok()?;

    let tree = resvg::usvg::Tree::from_data(&svg_data, &resvg::usvg::Options::default()).ok()?;
    let size = tree.size();

    // Scale for visibility (24x24 SVG -> 36x36 pixels)
    let scale = 1.5_f32;
    let width = (size.width() * scale) as u32;
    let height = (size.height() * scale) as u32;

    let mut pixmap = resvg::tiny_skia::Pixmap::new(width, height)?;
    let transform = resvg::tiny_skia::Transform::from_scale(scale, scale);
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // Convert to half-block characters (2 pixels per character vertically)
    let mut result = String::new();
    let pixels = pixmap.pixels();

    for y in (0..height).step_by(2) {
        for x in 0..width {
            let top_idx = (y * width + x) as usize;
            let bot_idx = ((y + 1) * width + x) as usize;

            let top = pixels.get(top_idx).map(|p| p.alpha() > 128).unwrap_or(false);
            let bot = pixels.get(bot_idx).map(|p| p.alpha() > 128).unwrap_or(false);

            let ch = match (top, bot) {
                (true, true) => '█',
                (true, false) => '▀',
                (false, true) => '▄',
                (false, false) => ' ',
            };
            result.push(ch);
        }
        result.push('\n');
    }

    Some(result)
}
