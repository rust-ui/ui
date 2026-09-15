use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Paragraph};

const SEPARATOR_LEN: usize = 3; // " │ " = 3 chars
const HORIZONTAL_PADDING: usize = 1; // Padding on left and right

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, strum::AsRefStr)]
pub enum Tab {
    #[default]
    Components,
    Demos,
    Hooks,
    Blocks,
    Icons,
    Settings,
}

impl Tab {
    pub const ALL: [Tab; 6] =
        [Tab::Components, Tab::Demos, Tab::Hooks, Tab::Blocks, Tab::Icons, Tab::Settings];

    fn from_index(index: usize) -> Self {
        Self::ALL.get(index).copied().unwrap_or(Tab::Components)
    }

    fn to_index(self) -> usize {
        Self::ALL.iter().position(|&t| t == self).unwrap_or(0)
    }
}

#[derive(Default)]
pub struct TabsState {
    pub current: Tab,
}

impl TabsState {
    pub fn next(&mut self) {
        let index = (self.current.to_index() + 1) % Tab::ALL.len();
        self.current = Tab::from_index(index);
    }

    pub fn previous(&mut self) {
        let index = self.current.to_index();
        let new_index = if index > 0 { index - 1 } else { Tab::ALL.len() - 1 };
        self.current = Tab::from_index(new_index);
    }
}

pub struct Header<'a> {
    pub tabs: TabsState,
    pub title: &'a str,
}

impl<'a> Header<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { tabs: TabsState::default(), title }
    }

    pub fn render(&self, frame: &mut Frame, area: Rect) {
        let block = Block::bordered().title(self.title);
        let inner = block.inner(area);
        frame.render_widget(block, area);

        // Build tab spans dynamically
        let mut spans = Vec::new();
        let mut left_tabs_length = 0;

        // Add left padding
        spans.push(Span::raw(" ".repeat(HORIZONTAL_PADDING)));
        left_tabs_length += HORIZONTAL_PADDING;

        for (index, tab) in Tab::ALL.iter().enumerate() {
            let tab_name = tab.as_ref();
            // Determine style based on whether this tab is active
            let style = if self.tabs.current == *tab {
                Style::default().fg(Color::White)
            } else {
                Style::default().fg(Color::DarkGray)
            };

            if index == Tab::ALL.len() - 1 {
                // Last tab (Settings) - will be added after spacer
                // Calculate spacer: inner_width - left_tabs_length - settings_length - right_padding
                let spacer_len = (inner.width as usize)
                    .saturating_sub(left_tabs_length)
                    .saturating_sub(tab_name.len())
                    .saturating_sub(HORIZONTAL_PADDING);

                spans.push(Span::raw(" ".repeat(spacer_len)));
                spans.push(Span::styled(tab_name, style));
            } else {
                // Left-aligned tabs
                spans.push(Span::styled(tab_name, style));
                left_tabs_length += tab_name.len();

                // Add separator if not the second-to-last tab
                if index < Tab::ALL.len() - 2 {
                    spans.push(separator_span());
                    left_tabs_length += SEPARATOR_LEN;
                }
            }
        }

        let tabs_line = Line::from(spans);
        let tabs_paragraph = Paragraph::new(tabs_line);
        frame.render_widget(tabs_paragraph, inner);
    }

    pub fn handle_click(&mut self, column: u16, terminal_width: u16) {
        let boundaries = self.calculate_tab_boundaries(terminal_width);

        for (index, &(start, end)) in boundaries.iter().enumerate() {
            if column >= start && column < end {
                self.tabs.current = Tab::from_index(index);
                return;
            }
        }
    }

    fn calculate_tab_boundaries(&self, terminal_width: u16) -> Vec<(u16, u16)> {
        // Custom flex layout: [pad][Components][ │ ][Hooks][ │ ][Blocks][ │ ][Icons][Spacer][Settings][pad]
        // Returns (start, end) tuples for each tab
        // All tabs are left-aligned except the last one (Settings) which is right-aligned

        let inner_width = terminal_width.saturating_sub(2); // Account for borders
        let border_offset = 1; // Add 1 to convert from inner area positions to absolute terminal columns

        let mut boundaries = Vec::new();
        let mut current_pos = HORIZONTAL_PADDING as u16; // Start after left padding

        // Calculate boundaries for all tabs except the last one (Settings)
        // Settings is right-aligned, so we handle it separately
        for (index, tab) in Tab::ALL.iter().enumerate() {
            let tab_name = tab.as_ref();
            if index == Tab::ALL.len() - 1 {
                // Last tab (Settings) - right-aligned with right padding
                let tab_len = tab_name.len() as u16;
                let start = inner_width.saturating_sub(tab_len).saturating_sub(HORIZONTAL_PADDING as u16)
                    + border_offset;
                let end = inner_width.saturating_sub(HORIZONTAL_PADDING as u16) + border_offset;
                boundaries.push((start, end));
            } else {
                // Left-aligned tabs
                let tab_len = tab_name.len() as u16;
                let start = current_pos + border_offset;
                let end = current_pos + tab_len + border_offset;
                boundaries.push((start, end));

                // Move position forward: tab length + separator length
                current_pos += tab_len + SEPARATOR_LEN as u16;
            }
        }

        boundaries
    }
}

fn separator_span() -> Span<'static> {
    Span::styled(" │ ", Style::default().fg(Color::DarkGray))
}
