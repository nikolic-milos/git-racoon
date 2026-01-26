use crate::{
    components::command_bar::{CommandBar, CommandBarAction},
    screens::{
        self, Action, Screen,
        home::{home, state::HomeWindow},
    },
};
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

pub struct App {
    pub should_quit: bool,
    pub screen_stack: Vec<Box<dyn Screen>>,
    pub status_text: String,
    command_bar: Option<CommandBar>,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            screen_stack: vec![Box::new(HomeWindow::new())],
            status_text: "Current directory is not a git repository".to_string(),
            command_bar: None,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        if let Some(bar) = &self.command_bar {
            let command_bar_height = 3;
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(command_bar_height),
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(f.area());

            bar.draw(f, chunks[0]);

            if let Some(screen) = self.screen_stack.last() {
                screen.draw(f, chunks[1]);
            }
        } else {
            if let Some(screen) = self.screen_stack.last() {
                screen.draw(f, f.area());
            }
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let Some(ref mut bar) = self.command_bar {
            match bar.handle_key(key) {
                CommandBarAction::Cancel => {
                    self.command_bar = None;
                }
                CommandBarAction::Submit(input) => {
                    // TODO: Implement submit command..
                    self.command_bar = None
                }
                CommandBarAction::Continue => {}
            }
            return;
        }

        if let Some(screen) = self.screen_stack.last_mut() {
            let action = screen.handle_keys(key);
            self.handle_action(action);
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::GoBack => {
                if self.screen_stack.len() > 1 {
                    self.screen_stack.pop();
                }
            }
            Action::None => {}
            Action::NavigateTo => {}
        }
    }
}
