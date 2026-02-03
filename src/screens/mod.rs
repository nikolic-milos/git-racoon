use crossterm::event::KeyEvent;
use ratatui::{Frame, layout::Rect};

use crate::context::{self, Context};

pub mod home;

pub trait Screen {
    fn draw(&self, f: &mut Frame, area: Rect, ctx: &Context);
    fn handle_keys(&mut self, key: KeyEvent, ctx: &Context) -> Action;
}

pub enum Action {
    None,
    NavigateTo,
    GoBack,
    Quit,
    Authenticate,
    AuthSuccess(String),
    Logout,
}
