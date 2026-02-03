mod auth;

pub use auth::AuthPopup;
use crossterm::event::KeyEvent;
use ratatui::Frame;
use ratatui::layout::Rect;

pub enum Popup {
    Auth(AuthPopup),
}

impl Popup {
    pub fn draw(&self, f: &mut Frame, area: Rect) {
        match self {
            Popup::Auth(p) => p.draw(f, area),
        }
    }
    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        match self {
            Popup::Auth(p) => p.handle_key(key),
        }
    }
}
