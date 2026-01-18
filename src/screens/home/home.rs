use crate::screens::home::state::{HomeWindow, HomeWindowTab};
use crate::screens::home::{banner, layout, menu};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

impl HomeWindow {
    pub fn new() -> Self {
        Self {
            active_tab: HomeWindowTab::MainMenu,
            main_cursor_index: 0,
            recent_cursor_index: 0,
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let [banner_area, main_menu_area, recent_repo_area] = layout::split_home_area(area);

        banner::draw(f, banner_area);

        menu::draw_main_menu(
            f,
            main_menu_area,
            self.main_cursor_index,
            self.active_tab == HomeWindowTab::MainMenu,
        );

        menu::draw_recent_repos(
            f,
            recent_repo_area,
            self.recent_cursor_index,
            self.active_tab == HomeWindowTab::RecentRepositories,
        );
    }

    pub fn handle_keys(&mut self, key: KeyEvent, should_quit: &mut bool) {
        match key.code {
            KeyCode::Tab => {
                self.active_tab = match self.active_tab {
                    HomeWindowTab::MainMenu => HomeWindowTab::RecentRepositories,
                    HomeWindowTab::RecentRepositories => HomeWindowTab::MainMenu,
                };
            }

            KeyCode::Up => {
                match self.active_tab {
                    HomeWindowTab::MainMenu => self.main_cursor_index.saturating_sub(1),
                    HomeWindowTab::RecentRepositories => self.recent_cursor_index.saturating_sub(1),
                };
            }

            KeyCode::Down => {
                match self.active_tab {
                    HomeWindowTab::MainMenu => {
                        self.main_cursor_index = (self.main_cursor_index + 1).min(3)
                    }
                    HomeWindowTab::RecentRepositories => {
                        self.recent_cursor_index = (self.recent_cursor_index + 1).min(4)
                    }
                };
            }

            KeyCode::Enter => {
                match self.active_tab {
                    HomeWindowTab::MainMenu => {
                        match self.main_cursor_index {
                            // 0 => app.active_window = ActiveWindow::Repository(RepoState::new()),
                            // 1 => app.active_window = ActiveWindow::Repository(RepoState::new()),
                            // 2 => app.active_window = ActiveWindow::Settings(),
                            3 => *should_quit = true,
                            _ => {}
                        }
                    }

                    HomeWindowTab::RecentRepositories => {
                        match self.recent_cursor_index {
                            // 0 1 2 3 4 => Open Repo
                            _ => {}
                        }
                    }
                }
            }
            _ => (),
        }
    }
}
