use std::collections::HashSet;
use std::error::Error;
use std::io;
use std::time::{Duration, Instant};

use crossterm::event::{self, DisableMouseCapture, EnableMouseCapture, KeyCode, KeyModifiers, MouseEventKind};
use crossterm::execute;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode};
use ratatui::Terminal;
use ratatui::backend::{Backend, CrosstermBackend};

use super::DependencyMap;
use super::app::App;
use super::header::Tab;
use super::tabs::{_render, tab1_components, tab2_hooks, tab5_demos};

pub fn run(
    tick_rate: Duration,
    components: Vec<String>,
    installed: HashSet<String>,
    dependencies: DependencyMap,
) -> Result<Vec<String>, Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app and run it
    let app = App::new("Rust/UI CLI", components, installed, dependencies);
    let app_result = run_app(&mut terminal, app, tick_rate);

    // Restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen, DisableMouseCapture)?;
    terminal.show_cursor()?;

    app_result
}

/* ========================================================== */
/*                     ✨ FUNCTIONS ✨                        */
/* ========================================================== */

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    mut app: App,
    tick_rate: Duration,
) -> Result<Vec<String>, Box<dyn Error>>
where
    <B as Backend>::Error: 'static,
{
    let mut last_tick = Instant::now();
    loop {
        terminal.draw(|frame| _render::render(frame, &mut app))?;

        let timeout = tick_rate.saturating_sub(last_tick.elapsed());
        if !event::poll(timeout)? {
            app.on_tick();
            last_tick = Instant::now();
            continue;
        }
        match event::read()? {
            event::Event::Key(key) if key.kind == event::KeyEventKind::Press => {
                // Handle search mode in Components tab
                if app.components_search_active && matches!(app.header.tabs.current, Tab::Components) {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => app.on_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.on_up(),
                        KeyCode::Char(c) => app.components_search_input(c),
                        KeyCode::Backspace => app.components_search_backspace(),
                        KeyCode::Esc => app.toggle_components_search(),
                        _ => {}
                    }
                // Handle search mode in Demos tab
                } else if app.demos_search_active && matches!(app.header.tabs.current, Tab::Demos) {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => app.on_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.on_up(),
                        KeyCode::Char(c) => app.demos_search_input(c),
                        KeyCode::Backspace => app.demos_search_backspace(),
                        KeyCode::Esc => app.toggle_demos_search(),
                        _ => {}
                    }
                // Handle search mode in Hooks tab
                } else if app.hooks_search_active && matches!(app.header.tabs.current, Tab::Hooks) {
                    match key.code {
                        KeyCode::Char('j') | KeyCode::Down => app.on_down(),
                        KeyCode::Char('k') | KeyCode::Up => app.on_up(),
                        KeyCode::Char(c) => app.hooks_search_input(c),
                        KeyCode::Backspace => app.hooks_search_backspace(),
                        KeyCode::Esc => app.toggle_hooks_search(),
                        _ => {}
                    }
                } else {
                    match key.code {
                        KeyCode::Char('?') => {
                            app.toggle_help_popup();
                        }
                        KeyCode::Char('/') if matches!(app.header.tabs.current, Tab::Components) => {
                            app.toggle_components_search();
                        }
                        KeyCode::Char('/') if matches!(app.header.tabs.current, Tab::Demos) => {
                            app.toggle_demos_search();
                        }
                        KeyCode::Char('/') if matches!(app.header.tabs.current, Tab::Hooks) => {
                            app.toggle_hooks_search();
                        }
                        KeyCode::Char(' ') if matches!(app.header.tabs.current, Tab::Components) => {
                            if let Some(component) = tab1_components::get_selected_component(&app) {
                                app.toggle_component_checkbox(&component);
                            }
                        }
                        KeyCode::Char(' ') if matches!(app.header.tabs.current, Tab::Demos) => {
                            if let Some(demo) = tab5_demos::get_selected_demo(&app) {
                                app.toggle_demo_checkbox(&demo);
                            }
                        }
                        KeyCode::Char(' ') if matches!(app.header.tabs.current, Tab::Hooks) => {
                            if let Some(hook) = tab2_hooks::get_selected_hook(&app) {
                                app.toggle_hook_checkbox(hook);
                            }
                        }
                        // Ctrl+letter to jump to first item starting with that letter
                        KeyCode::Char(c)
                            if key.modifiers.contains(KeyModifiers::CONTROL)
                                && c.is_ascii_alphabetic() =>
                        {
                            match app.header.tabs.current {
                                Tab::Components => app.jump_to_letter_components(c),
                                Tab::Demos => app.jump_to_letter_demos(c),
                                Tab::Hooks => app.jump_to_letter_hooks(c),
                                _ => {}
                            }
                        }
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Components)
                                && !app.show_popup
                                && !app.components_checked.is_empty() =>
                        {
                            app.toggle_popup();
                        }
                        // Handle Enter in popup - confirm or cancel based on button focus
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Components)
                                && app.show_popup
                                && !app.components_checked.is_empty() =>
                        {
                            if app.popup_confirm_focused {
                                let selected: Vec<String> = app.components_checked.into_iter().collect();
                                return Ok(selected);
                            } else {
                                app.toggle_popup(); // Cancel - close popup
                            }
                        }
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Demos)
                                && !app.show_popup
                                && !app.demos_checked.is_empty() =>
                        {
                            app.toggle_popup();
                        }
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Demos)
                                && app.show_popup
                                && !app.demos_checked.is_empty() =>
                        {
                            if app.popup_confirm_focused {
                                let selected: Vec<String> = app.demos_checked.into_iter().collect();
                                return Ok(selected);
                            } else {
                                app.toggle_popup(); // Cancel - close popup
                            }
                        }
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Hooks)
                                && !app.show_popup
                                && !app.hooks_checked.is_empty() =>
                        {
                            app.toggle_popup();
                        }
                        KeyCode::Enter
                            if matches!(app.header.tabs.current, Tab::Hooks)
                                && app.show_popup
                                && !app.hooks_checked.is_empty() =>
                        {
                            if app.popup_confirm_focused {
                                let selected: Vec<String> = app.hooks_checked.into_iter().collect();
                                return Ok(selected);
                            } else {
                                app.toggle_popup(); // Cancel - close popup
                            }
                        }
                        KeyCode::Esc if app.show_help_popup => {
                            app.toggle_help_popup();
                        }
                        KeyCode::Esc
                            if matches!(app.header.tabs.current, Tab::Components) && !app.show_popup =>
                        {
                            // Handle double-tap Escape to deselect all components
                            let now = Instant::now();
                            let is_double_tap = if let Some(last_time) = app.last_escape_time {
                                now.duration_since(last_time).as_millis() < 500
                            } else {
                                false
                            };

                            if is_double_tap && !app.components_checked.is_empty() {
                                app.deselect_all_components();
                                app.last_escape_time = None;
                            } else {
                                app.last_escape_time = Some(now);
                            }
                        }
                        KeyCode::Esc
                            if matches!(app.header.tabs.current, Tab::Components) && app.show_popup =>
                        {
                            app.toggle_popup();
                        }
                        KeyCode::Esc if matches!(app.header.tabs.current, Tab::Demos) && !app.show_popup => {
                            // Handle double-tap Escape to deselect all demos
                            let now = Instant::now();
                            let is_double_tap = if let Some(last_time) = app.last_escape_time {
                                now.duration_since(last_time).as_millis() < 500
                            } else {
                                false
                            };

                            if is_double_tap && !app.demos_checked.is_empty() {
                                app.deselect_all_demos();
                                app.last_escape_time = None;
                            } else {
                                app.last_escape_time = Some(now);
                            }
                        }
                        KeyCode::Esc if matches!(app.header.tabs.current, Tab::Demos) && app.show_popup => {
                            app.toggle_popup();
                        }
                        KeyCode::Esc if matches!(app.header.tabs.current, Tab::Hooks) && !app.show_popup => {
                            // Handle double-tap Escape to deselect all hooks
                            let now = Instant::now();
                            let is_double_tap = if let Some(last_time) = app.last_escape_time {
                                now.duration_since(last_time).as_millis() < 500
                            } else {
                                false
                            };

                            if is_double_tap && !app.hooks_checked.is_empty() {
                                app.deselect_all_hooks();
                                app.last_escape_time = None;
                            } else {
                                app.last_escape_time = Some(now);
                            }
                        }
                        KeyCode::Esc if matches!(app.header.tabs.current, Tab::Hooks) && app.show_popup => {
                            app.toggle_popup();
                        }
                        KeyCode::Char('h') | KeyCode::Left => {
                            if app.show_popup {
                                app.toggle_popup_button_focus();
                            } else if !app.show_help_popup {
                                app.on_left();
                            }
                        }
                        KeyCode::Char('j') | KeyCode::Down => {
                            if !app.show_popup && !app.show_help_popup {
                                app.on_down();
                            }
                        }
                        KeyCode::Char('k') | KeyCode::Up => {
                            if !app.show_popup && !app.show_help_popup {
                                app.on_up();
                            }
                        }
                        KeyCode::Char('l') | KeyCode::Right => {
                            if app.show_popup {
                                app.toggle_popup_button_focus();
                            } else if !app.show_help_popup {
                                app.on_right();
                            }
                        }
                        KeyCode::Tab if app.show_popup => {
                            app.toggle_popup_button_focus();
                        }
                        KeyCode::Char(c) => app.on_key(c),
                        _ => {}
                    }
                }
            }
            event::Event::Mouse(mouse) => {
                match mouse.kind {
                    MouseEventKind::Down(_) => {
                        let terminal_width = terminal.size()?.width;
                        let now = std::time::Instant::now();
                        let current_pos = (mouse.column, mouse.row);

                        // Check for double-click (within 500ms and same position)
                        let is_double_click = if let (Some(last_time), Some(last_pos)) =
                            (app.last_click_time, app.last_click_pos)
                        {
                            now.duration_since(last_time).as_millis() < 500 && last_pos == current_pos
                        } else {
                            false
                        };

                        if is_double_click {
                            // Handle double-click on component list items
                            if let Some(visual_index) =
                                app.get_components_double_click_info(mouse.column, mouse.row, terminal_width)
                                && let Some(component) =
                                    tab1_components::get_component_at_visual_index(&app, visual_index)
                            {
                                app.toggle_component_checkbox(&component);
                            }
                            // Handle double-click on demo list items
                            if let Some(visual_index) =
                                app.get_demos_double_click_info(mouse.column, mouse.row, terminal_width)
                                && let Some(demo) = tab5_demos::get_demo_at_visual_index(&app, visual_index)
                            {
                                app.toggle_demo_checkbox(&demo);
                            }
                            // Handle double-click on hook list items
                            if let Some(visual_index) =
                                app.get_hooks_double_click_info(mouse.column, mouse.row, terminal_width)
                                && let Some(hook) = tab2_hooks::get_hook_at_visual_index(&app, visual_index)
                            {
                                app.toggle_hook_checkbox(hook);
                            }
                            // Reset click tracking after double-click
                            app.last_click_time = None;
                            app.last_click_pos = None;
                        } else {
                            // Single click - update tracking
                            app.on_mouse_click(mouse.column, mouse.row, terminal_width);
                            app.last_click_time = Some(now);
                            app.last_click_pos = Some(current_pos);
                        }
                    }
                    MouseEventKind::ScrollUp => {
                        if !app.show_popup && !app.show_help_popup {
                            app.on_up();
                        }
                    }
                    MouseEventKind::ScrollDown => {
                        if !app.show_popup && !app.show_help_popup {
                            app.on_down();
                        }
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        if app.should_quit {
            return Ok(Vec::new());
        }
    }
}
