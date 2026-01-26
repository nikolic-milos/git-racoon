use crossterm::event::{KeyCode, KeyEvent};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::style::{Color, Style, Stylize};
use ratatui::widgets::{Block, Borders, Paragraph};

pub struct CommandBar {
    pub input: String,
    pub selected_index: usize,
    pub suggestions: Vec<String>,
}

pub enum CommandBarAction {
    Continue,
    Cancel,
    Submit,
}

impl CommandBar {
    pub fn new() -> Self {
        CommandBar {
            input: String::new(),
            selected_index: 0,
            suggestions: Vec::new(),
        }
    }

    // TODO: Dropdown menu with suggestions
    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let input_text = format!("> {}", self.input);
        let line = Paragraph::new(input_text)
            .block(Block::default().borders(Borders::BOTTOM))
            .style(Style::default())
            .fg(Color::White);
        f.render_widget(line, area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> CommandBarAction {
        match key.code {
            KeyCode::Esc => CommandBarAction::Cancel,
            KeyCode::Enter => CommandBarAction::Submit,
            KeyCode::Char(c) => {
                self.input.push(c);
                CommandBarAction::Continue
            }

            KeyCode::Backspace => {
                self.input.pop();
                CommandBarAction::Continue
            }

            KeyCode::Up => {
                if self.selected_index > 0 {
                    self.selected_index -= 1;
                }
                CommandBarAction::Continue
            }

            KeyCode::Down => {
                if !self.suggestions.is_empty() && self.selected_index < self.suggestions.len() - 1
                {
                    self.selected_index += 1;
                }
                CommandBarAction::Continue
            }

            _ => CommandBarAction::Continue,
        }
    }
}
