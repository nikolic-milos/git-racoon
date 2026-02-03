use ::crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, Paragraph};

use crate::auth::github::GITHUB_VERIFICATION_URL;

pub struct AuthPopup {
    pub user_code: String,
    pub verification_url: &'static str,
    pub status_text: String,
}

impl AuthPopup {
    pub fn new(user_code: String) -> Self {
        AuthPopup {
            user_code,
            verification_url: crate::auth::github::GITHUB_VERIFICATION_URL,
            status_text: "Waiting for authentication...".to_string(),
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let text = format!(
            "Go to: {}\n\nEnter code: {}\n\n{}\n\nPress ESC to cancel",
            GITHUB_VERIFICATION_URL, self.user_code, self.status_text
        );

        let popup = Paragraph::new(text).block(
            Block::default()
                .borders(Borders::ALL)
                .title("GitHub Authentication"),
        );

        f.render_widget(popup, area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) -> bool {
        matches!(key.code, KeyCode::Esc)
    }
}
