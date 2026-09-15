use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Clear, Paragraph, Wrap};

use super::super::app::App;
use super::super::header::Tab;
use super::super::widgets::popup::popup_area;
use super::{tab1_components, tab2_hooks, tab3_blocks, tab4_icons, tab5_demos, tab9_settings};

pub fn render(frame: &mut Frame, app: &mut App) {
    let chunks =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0), Constraint::Length(1)]).split(frame.area());

    let (Some(header_area), Some(content_area), Some(footer_area)) =
        (chunks.first(), chunks.get(1), chunks.get(2))
    else {
        return;
    };

    // Render header with tabs
    app.header.render(frame, *header_area);

    match app.header.tabs.current {
        Tab::Components => tab1_components::draw_tab_components(frame, app, *content_area),
        Tab::Demos => tab5_demos::draw_tab_demos(frame, app, *content_area),
        Tab::Hooks => tab2_hooks::draw_tab_hooks(frame, app, *content_area),
        Tab::Blocks => tab3_blocks::draw_tab_blocks(frame, app, *content_area),
        Tab::Icons => tab4_icons::draw_tab_icons(frame, app, *content_area),
        Tab::Settings => tab9_settings::draw_tab_settings(frame, app, *content_area),
    };

    // Render footer with shortcuts
    draw_footer(frame, app, *footer_area);

    // Render help popup on top of everything
    if app.show_help_popup {
        draw_help_popup(frame, frame.area());
    }
}

fn draw_footer(frame: &mut Frame, app: &App, area: Rect) {
    let key_style = Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD);
    let sep_style = Style::default().fg(Color::DarkGray);
    let text_style = Style::default().fg(Color::Gray);

    let shortcuts = match app.header.tabs.current {
        Tab::Components | Tab::Demos | Tab::Hooks => {
            if app.show_popup {
                vec![
                    Span::styled("←/→", key_style),
                    Span::styled(" Switch ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" Enter", key_style),
                    Span::styled(" Confirm ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" Esc", key_style),
                    Span::styled(" Cancel", text_style),
                ]
            } else {
                vec![
                    Span::styled("Space", key_style),
                    Span::styled(" Select ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" /", key_style),
                    Span::styled(" Search ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" Ctrl+a-z", key_style),
                    Span::styled(" Jump ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" ?", key_style),
                    Span::styled(" Help ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" Enter", key_style),
                    Span::styled(" Confirm ", text_style),
                    Span::styled("│", sep_style),
                    Span::styled(" q", key_style),
                    Span::styled(" Quit", text_style),
                ]
            }
        }
        _ => {
            vec![
                Span::styled("←/→", key_style),
                Span::styled(" Tabs ", text_style),
                Span::styled("│", sep_style),
                Span::styled(" ?", key_style),
                Span::styled(" Help ", text_style),
                Span::styled("│", sep_style),
                Span::styled(" q", key_style),
                Span::styled(" Quit", text_style),
            ]
        }
    };

    let footer = Paragraph::new(Line::from(shortcuts)).style(Style::default().bg(Color::DarkGray));
    frame.render_widget(footer, area);
}

fn draw_help_popup(frame: &mut Frame, area: Rect) {
    let popup_block =
        Block::bordered().title("⌨️  Keyboard Shortcuts").style(Style::default().fg(Color::Cyan));
    let popup_area = popup_area(area, 75, 85);

    // Clear the background
    frame.render_widget(Clear, popup_area);

    let help_text = r#"
Global Commands:
  ?            Show this help menu
  q            Quit the application
  h / ←        Navigate to previous tab
  l / →        Navigate to next tab
  j / ↓        Scroll down
  k / ↑        Scroll up

Components Tab:
  /            Activate search mode
  Space        Toggle checkbox for selected component
  Enter        View checked components (when components are selected)
  Esc Esc      Double-tap Escape to deselect all components
  Double-click Toggle checkbox on clicked component
  Mouse Wheel  Scroll up/down

Search Mode (Components Tab):
  j / ↓        Navigate down while searching
  k / ↑        Navigate up while searching
  Esc          Exit search mode
  Backspace    Delete character
  Any char     Add to search query


Press ESC to close this help menu
"#;

    let popup_paragraph = Paragraph::new(help_text)
        .block(popup_block)
        .wrap(Wrap { trim: true })
        .style(Style::default().fg(Color::White));

    frame.render_widget(popup_paragraph, popup_area);
}
