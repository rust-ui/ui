use std::collections::HashSet;
use std::time::Instant;

use ratatui::widgets::{ListState, ScrollbarState};

use super::DependencyMap;
use super::header::{Header, Tab};

pub struct App<'a> {
    pub should_quit: bool,
    pub header: Header<'a>,
    // Installed components (already in project)
    pub installed: HashSet<String>,
    // Dependencies map (component -> its dependencies)
    pub dependencies: DependencyMap,
    // Components (non-demo items)
    pub components: Vec<String>,
    pub components_scroll: usize,
    pub components_scroll_state: ScrollbarState,
    pub components_list_state: ListState,
    pub components_search_query: String,
    pub components_search_active: bool,
    pub components_checked: HashSet<String>,
    // Demos (demo_* items)
    pub demos: Vec<String>,
    pub demos_scroll: usize,
    pub demos_scroll_state: ScrollbarState,
    pub demos_list_state: ListState,
    pub demos_search_query: String,
    pub demos_search_active: bool,
    pub demos_checked: HashSet<String>,
    // Hooks
    pub hooks_scroll: usize,
    pub hooks_scroll_state: ScrollbarState,
    pub hooks_list_state: ListState,
    pub hooks_search_query: String,
    pub hooks_search_active: bool,
    pub hooks_checked: HashSet<String>,
    // Other
    pub terminal_width: u16,
    pub icons_selected: usize,
    pub show_popup: bool,
    pub popup_confirm_focused: bool, // true = Confirm button, false = Cancel button
    pub show_help_popup: bool,
    pub last_click_time: Option<Instant>,
    pub last_click_pos: Option<(u16, u16)>,
    pub last_escape_time: Option<Instant>,
}

impl<'a> App<'a> {
    pub fn new(
        title: &'a str,
        all_items: Vec<String>,
        installed: HashSet<String>,
        dependencies: DependencyMap,
    ) -> Self {
        // Separate demos from components
        let (demos, components): (Vec<_>, Vec<_>) =
            all_items.into_iter().partition(|s| s.starts_with("demo_"));

        App {
            should_quit: false,
            header: Header::new(title),
            // Installed
            installed,
            // Dependencies
            dependencies,
            // Components
            components,
            components_scroll: 0,
            components_scroll_state: ScrollbarState::default(),
            components_list_state: ListState::default(),
            components_search_query: String::new(),
            components_search_active: false,
            components_checked: HashSet::new(),
            // Demos
            demos,
            demos_scroll: 0,
            demos_scroll_state: ScrollbarState::default(),
            demos_list_state: ListState::default(),
            demos_search_query: String::new(),
            demos_search_active: false,
            demos_checked: HashSet::new(),
            // Hooks
            hooks_scroll: 0,
            hooks_scroll_state: ScrollbarState::default(),
            hooks_list_state: ListState::default(),
            hooks_search_query: String::new(),
            hooks_search_active: false,
            hooks_checked: HashSet::new(),
            // Other
            terminal_width: 0,
            icons_selected: 0,
            show_popup: false,
            popup_confirm_focused: true, // Default to Confirm button
            show_help_popup: false,
            last_click_time: None,
            last_click_pos: None,
            last_escape_time: None,
        }
    }

    pub fn on_up(&mut self) {
        match self.header.tabs.current {
            Tab::Components => {
                self.components_scroll = self.components_scroll.saturating_sub(1);
                self.components_scroll_state = self.components_scroll_state.position(self.components_scroll);
            }
            Tab::Demos => {
                self.demos_scroll = self.demos_scroll.saturating_sub(1);
                self.demos_scroll_state = self.demos_scroll_state.position(self.demos_scroll);
            }
            Tab::Hooks => {
                self.hooks_scroll = self.hooks_scroll.saturating_sub(1);
                self.hooks_scroll_state = self.hooks_scroll_state.position(self.hooks_scroll);
            }
            Tab::Icons => {
                self.icons_selected = self.icons_selected.saturating_sub(1);
            }
            Tab::Blocks | Tab::Settings => {}
        }
    }

    pub fn on_down(&mut self) {
        match self.header.tabs.current {
            Tab::Components => {
                self.components_scroll = self.components_scroll.saturating_add(1);
                self.components_scroll_state = self.components_scroll_state.position(self.components_scroll);
            }
            Tab::Demos => {
                self.demos_scroll = self.demos_scroll.saturating_add(1);
                self.demos_scroll_state = self.demos_scroll_state.position(self.demos_scroll);
            }
            Tab::Hooks => {
                self.hooks_scroll = self.hooks_scroll.saturating_add(1);
                self.hooks_scroll_state = self.hooks_scroll_state.position(self.hooks_scroll);
            }
            Tab::Icons => {
                if self.icons_selected < 1 {
                    self.icons_selected += 1;
                }
            }
            Tab::Blocks | Tab::Settings => {}
        }
    }

    pub fn on_right(&mut self) {
        self.header.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.header.tabs.previous();
    }

    pub fn on_key(&mut self, c: char) {
        if c == 'q' {
            self.should_quit = true;
        }
    }

    pub fn on_tick(&mut self) {}

    pub fn on_mouse_click(&mut self, column: u16, row: u16, terminal_width: u16) {
        self.terminal_width = terminal_width;

        // Tab area is the first 3 lines (Constraint::Length(3) in _render.rs)
        if row < 3 {
            self.header.handle_click(column, terminal_width);
        }
    }

    pub fn toggle_components_search(&mut self) {
        self.components_search_active = !self.components_search_active;
        if !self.components_search_active {
            self.components_search_query.clear();
            self.components_scroll = 0;
        }
    }

    pub fn components_search_input(&mut self, c: char) {
        if self.components_search_active {
            self.components_search_query.push(c);
            self.components_scroll = 0;
        }
    }

    pub fn components_search_backspace(&mut self) {
        if self.components_search_active {
            self.components_search_query.pop();
            self.components_scroll = 0;
        }
    }

    pub fn toggle_component_checkbox(&mut self, component: &str) {
        if self.components_checked.contains(component) {
            self.components_checked.remove(component);
        } else {
            self.components_checked.insert(component.to_string());
        }
    }

    pub fn toggle_popup(&mut self) {
        self.show_popup = !self.show_popup;
        if self.show_popup {
            self.popup_confirm_focused = true; // Reset to Confirm when opening
        }
    }

    pub fn toggle_popup_button_focus(&mut self) {
        self.popup_confirm_focused = !self.popup_confirm_focused;
    }

    pub fn toggle_help_popup(&mut self) {
        self.show_help_popup = !self.show_help_popup;
    }

    pub fn deselect_all_components(&mut self) {
        self.components_checked.clear();
    }

    // Demos methods
    pub fn toggle_demos_search(&mut self) {
        self.demos_search_active = !self.demos_search_active;
        if !self.demos_search_active {
            self.demos_search_query.clear();
            self.demos_scroll = 0;
        }
    }

    pub fn demos_search_input(&mut self, c: char) {
        if self.demos_search_active {
            self.demos_search_query.push(c);
            self.demos_scroll = 0;
        }
    }

    pub fn demos_search_backspace(&mut self) {
        if self.demos_search_active {
            self.demos_search_query.pop();
            self.demos_scroll = 0;
        }
    }

    pub fn toggle_demo_checkbox(&mut self, demo: &str) {
        if self.demos_checked.contains(demo) {
            self.demos_checked.remove(demo);
        } else {
            self.demos_checked.insert(demo.to_string());
        }
    }

    pub fn deselect_all_demos(&mut self) {
        self.demos_checked.clear();
    }

    pub fn get_demos_double_click_info(&self, column: u16, row: u16, terminal_width: u16) -> Option<usize> {
        if matches!(self.header.tabs.current, Tab::Demos) && !self.show_popup {
            let left_panel_width = (terminal_width as f32 * 0.35) as u16;

            if column <= left_panel_width && row > 6 {
                let visual_row = (row - 7) as usize;
                let viewport_offset = self.demos_list_state.offset();
                let item_index = visual_row + viewport_offset;
                return Some(item_index);
            }
        }
        None
    }

    pub fn get_components_double_click_info(
        &self,
        column: u16,
        row: u16,
        terminal_width: u16,
    ) -> Option<usize> {
        // Check if double-click is in Components tab left panel
        if matches!(self.header.tabs.current, Tab::Components) && !self.show_popup {
            // Check if click is in left panel (35% of width)
            let left_panel_width = (terminal_width as f32 * 0.35) as u16;

            if column <= left_panel_width && row > 6 {
                // Calculate which item was clicked (accounting for header and search)
                // Row 0-2: Header/tabs, Row 3-5: Search box, Row 6: List border top, Row 7+: List items
                let visual_row = (row - 7) as usize;
                // Add the viewport offset from the ListState to get the actual item index
                let viewport_offset = self.components_list_state.offset();
                let item_index = visual_row + viewport_offset;
                return Some(item_index);
            }
        }
        None
    }

    pub fn toggle_hooks_search(&mut self) {
        self.hooks_search_active = !self.hooks_search_active;
        if !self.hooks_search_active {
            self.hooks_search_query.clear();
            self.hooks_scroll = 0;
        }
    }

    pub fn hooks_search_input(&mut self, c: char) {
        if self.hooks_search_active {
            self.hooks_search_query.push(c);
            self.hooks_scroll = 0;
        }
    }

    pub fn hooks_search_backspace(&mut self) {
        if self.hooks_search_active {
            self.hooks_search_query.pop();
            self.hooks_scroll = 0;
        }
    }

    pub fn toggle_hook_checkbox(&mut self, hook: &str) {
        if self.hooks_checked.contains(hook) {
            self.hooks_checked.remove(hook);
        } else {
            self.hooks_checked.insert(hook.to_string());
        }
    }

    pub fn deselect_all_hooks(&mut self) {
        self.hooks_checked.clear();
    }

    pub fn get_hooks_double_click_info(&self, column: u16, row: u16, terminal_width: u16) -> Option<usize> {
        // Check if double-click is in Hooks tab left panel
        if matches!(self.header.tabs.current, Tab::Hooks) && !self.show_popup {
            // Check if click is in left panel (35% of width)
            let left_panel_width = (terminal_width as f32 * 0.35) as u16;

            if column <= left_panel_width && row > 6 {
                // Calculate which item was clicked (accounting for header and search)
                // Row 0-2: Header/tabs, Row 3-5: Search box, Row 6: List border top, Row 7+: List items
                let visual_row = (row - 7) as usize;
                // Add the viewport offset from the ListState to get the actual item index
                let viewport_offset = self.hooks_list_state.offset();
                let item_index = visual_row + viewport_offset;
                return Some(item_index);
            }
        }
        None
    }

    pub fn get_dependencies(&self, component: &str) -> Option<&Vec<String>> {
        self.dependencies.get(component)
    }

    pub fn jump_to_letter_components(&mut self, letter: char) {
        use super::widgets::helpers::filter_items;
        let components_refs: Vec<&str> = self.components.iter().map(|s| s.as_str()).collect();
        let filtered = filter_items(&components_refs, &self.components_search_query);
        let lower = letter.to_ascii_lowercase();
        if let Some(idx) = filtered.iter().position(|c| c.to_lowercase().starts_with(lower)) {
            self.components_scroll = idx;
            self.components_scroll_state = self.components_scroll_state.position(idx);
        }
    }

    pub fn jump_to_letter_demos(&mut self, letter: char) {
        use super::widgets::helpers::filter_items;
        let demos_refs: Vec<&str> = self.demos.iter().map(|s| s.as_str()).collect();
        let filtered = filter_items(&demos_refs, &self.demos_search_query);
        let lower = letter.to_ascii_lowercase();
        // For demos, skip "demo_" prefix when matching
        if let Some(idx) = filtered.iter().position(|d| {
            d.strip_prefix("demo_")
                .unwrap_or(d)
                .to_lowercase()
                .starts_with(lower)
        }) {
            self.demos_scroll = idx;
            self.demos_scroll_state = self.demos_scroll_state.position(idx);
        }
    }

    pub fn jump_to_letter_hooks(&mut self, letter: char) {
        use super::tabs::tab2_hooks::HOOKS;
        use super::widgets::helpers::filter_items;
        let filtered = filter_items(HOOKS, &self.hooks_search_query);
        let lower = letter.to_ascii_lowercase();
        // For hooks, skip "Use " prefix when matching
        if let Some(idx) = filtered.iter().position(|h| {
            h.strip_prefix("Use ")
                .unwrap_or(h)
                .to_lowercase()
                .starts_with(lower)
        }) {
            self.hooks_scroll = idx;
            self.hooks_scroll_state = self.hooks_scroll_state.position(idx);
        }
    }
}
