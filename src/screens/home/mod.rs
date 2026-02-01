pub mod activity_feed;
pub mod banner;
pub mod controls;
pub mod layout;
pub mod login_status;
pub mod menu;
pub mod state;
pub mod view;

use crate::context::Context;
use crate::screens::home::menu::HOME_MENU_OPTIONS;
use crate::screens::home::state::*;
use crate::screens::{Action, Screen};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::widgets::Paragraph;
use ratatui::{Frame, layout::Rect};

impl Screen for HomeWindow {
    fn draw(&self, f: &mut Frame, area: Rect, ctx: &Context) {
        match layout::calculate_layout(area, f.area().height) {
            Ok(layout) => {
                banner::draw(f, layout.banner);
                menu::draw_main_menu(f, layout.main_menu, self.main_cursor_index, true);
                menu::draw_recent_repos(
                    f,
                    layout.recent_repos,
                    self.recent_cursor_index,
                    true,
                    &[],
                );
                activity_feed::draw(f, layout.activity_feed, &[]);
                controls::draw(f, layout.controls);
                login_status::draw(f, layout.login_status);
            }

            Err(msg) => {
                let error_paragraph = Paragraph::new(msg)
                    .alignment(ratatui::layout::HorizontalAlignment::Center)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::Red));
                f.render_widget(error_paragraph, area);
            }
        }
    }

    fn handle_keys(&mut self, key: KeyEvent, ctx: &Context) -> Action {
        match key.code {
            KeyCode::Tab => {
                self.active_tab = match self.active_tab {
                    HomeWindowTab::MainMenu => HomeWindowTab::RecentRepositories,
                    HomeWindowTab::RecentRepositories => HomeWindowTab::MainMenu,
                };
                Action::None
            }

            KeyCode::Up => match self.active_tab {
                HomeWindowTab::MainMenu => {
                    if self.main_cursor_index == 0 {
                        self.main_cursor_index = HOME_MENU_OPTIONS.len() - 1
                    } else {
                        self.main_cursor_index = self.main_cursor_index.saturating_sub(1);
                    }
                    Action::None
                }

                HomeWindowTab::RecentRepositories => {
                    if self.recent_cursor_index == 0 {
                        self.recent_cursor_index = RECENT_REPOS_MAX_LINES - 1
                    } else {
                        self.recent_cursor_index = self.recent_cursor_index.saturating_sub(1);
                    }
                    Action::None
                }
            },

            KeyCode::Down => match self.active_tab {
                HomeWindowTab::MainMenu => {
                    if self.main_cursor_index == 3 {
                        self.main_cursor_index = 0;
                    } else {
                        self.main_cursor_index = self.main_cursor_index.saturating_add(1);
                    }
                    Action::None
                }

                HomeWindowTab::RecentRepositories => {
                    if self.recent_cursor_index == RECENT_REPOS_MAX_LINES - 1 {
                        self.recent_cursor_index = 0
                    } else {
                        self.recent_cursor_index = self.recent_cursor_index.saturating_add(1);
                    }
                    Action::None
                }
            },

            KeyCode::Enter => match self.active_tab {
                HomeWindowTab::MainMenu => match self.main_cursor_index {
                    0 => Action::NavigateTo,
                    1 => Action::NavigateTo,
                    2 => Action::NavigateTo,
                    3 => Action::Quit,
                    _ => Action::None,
                },

                // TODO:
                HomeWindowTab::RecentRepositories => match self.recent_cursor_index {
                    0 => Action::None,
                    1 => Action::None,
                    2 => Action::None,
                    3 => Action::None,
                    4 => Action::None,
                    _ => Action::None,
                },
            },

            KeyCode::Char('q') => Action::Quit,
            KeyCode::Esc => Action::GoBack,

            _ => Action::None,
        }
    }
}
