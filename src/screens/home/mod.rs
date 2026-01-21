pub mod activity_feed;
pub mod banner;
pub mod controls;
pub mod home;
pub mod layout;
pub mod login_status;
pub mod menu;
pub mod state;

use crate::screens::home::layout::calculate_layout;
use crate::screens::home::state::*;
use crate::screens::{Action, Screen};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

impl Screen for HomeWindow {
    fn draw(&self, f: &mut Frame, area: Rect) {
        let layout = calculate_layout(f);

        banner::draw(f, layout.banner);
        menu::draw_main_menu(f, layout.main_menu, 0, true);
        menu::draw_recent_repos(f, layout.recent_repos, 0, true, &self.recent_repositories);
        controls::draw(f, layout.controls);
        login_status::draw(f, layout.login_status);
    }

    fn handle_keys(&mut self, key: KeyEvent) -> Action {
        match key.code {
            KeyCode::Tab => {
                self.active_tab = match self.active_tab {
                    HomeWindowTab::MainMenu => HomeWindowTab::RecentRepositories,
                    HomeWindowTab::RecentRepositories => HomeWindowTab::MainMenu,
                };
                return Action::None;
            }

            KeyCode::Up => match self.active_tab {
                HomeWindowTab::MainMenu => {
                    if (self.main_cursor_index == 0) {
                        self.main_cursor_index = 3;
                    } else {
                        self.main_cursor_index = self.main_cursor_index.saturating_sub(1);
                    }
                    return Action::None;
                }

                HomeWindowTab::RecentRepositories => {
                    if (self.recent_cursor_index == 0) {
                        self.recent_cursor_index = RECENT_REPOS_MAX_LINES - 1
                    } else {
                        self.recent_cursor_index = self.recent_cursor_index.saturating_add(1);
                    }
                    return Action::None;
                }
            },

            KeyCode::Down => match self.active_tab {
                HomeWindowTab::MainMenu => {
                    if (self.main_cursor_index == 3) {
                        self.main_cursor_index = 0;
                    } else {
                        self.main_cursor_index = self.main_cursor_index.saturating_add(1);
                    }
                    return Action::None;
                }

                HomeWindowTab::RecentRepositories => {
                    if (self.recent_cursor_index == RECENT_REPOS_MAX_LINES - 1) {
                        self.recent_cursor_index = 0
                    } else {
                        self.recent_cursor_index = self.recent_cursor_index.saturating_sub(1);
                    }
                    return Action::None;
                }
            },

            _ => Action::None,
        }
    }
}
