use crate::screens::home::state::{HomeWindow, HomeWindowTab};

impl HomeWindow {
    pub fn new() -> Self {
        Self {
            active_tab: HomeWindowTab::MainMenu,
            main_cursor_index: 0,
            recent_cursor_index: 0,
        }
    }
}
