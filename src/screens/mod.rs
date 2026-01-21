use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

pub mod home;

pub trait Screen {
    fn draw(&self, f: &mut Frame, area: Rect);
    fn handle_keys(&mut self, key: KeyEvent) -> Action;
}

pub enum Action {
    None,
    NavigateTo,
    GoBack,
    Quit,
}
