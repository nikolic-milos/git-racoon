use crate::screens::ActiveWindow;
use crate::screens::home::state::HomeWindow;
use crossterm::event::KeyEvent;
use ratatui::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct App {
    pub should_quit: bool,
    pub status_text: String,
    pub active_window: ActiveWindow,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            active_window: ActiveWindow::Home(HomeWindow::new()),
            status_text: "Current directory is not a git repository".to_string(),
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        match self.active_window {
            ActiveWindow::Home(ref hw) => hw.draw(f, f.area()),
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.active_window {
            ActiveWindow::Home(ref mut hw) => hw.handle_keys(key, &mut self.should_quit),
        }
    }
}
