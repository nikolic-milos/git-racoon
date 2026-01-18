use crate::screens::home::state::{HomeWindow, HomeWindowTab};
use crate::screens::home::{banner, controls, layout, login_status, menu, tip};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{Frame, layout::Rect};

impl HomeWindow {
    pub fn new() -> Self {
        Self {
            active_tab: HomeWindowTab::MainMenu,
            main_cursor_index: 0,
            recent_cursor_index: 0,
            recent_repositories: Vec::new(),
        }
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let [
            banner_area,
            tip_area,
            controls_area,
            login_status_area,
            recent_repo_area,
            main_menu_area,
        ] = layout::split_home_area(area);

        banner::draw(f, banner_area);
        tip::draw(f, tip_area);
        controls::draw(f, controls_area);
        login_status::draw(f, login_status_area);

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
            &self.recent_repositories,
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
                    HomeWindowTab::MainMenu => {
                        if self.main_cursor_index == 0 {
                            self.main_cursor_index =
                                crate::screens::home::menu::HOME_MENU_OPTIONS.len() - 1;
                        } else {
                            self.main_cursor_index = self.main_cursor_index.saturating_sub(1)
                        }
                    }
                    HomeWindowTab::RecentRepositories => {
                        if self.recent_cursor_index == 0 {
                            self.recent_cursor_index = 0
                        } else {
                            self.recent_cursor_index = self.recent_cursor_index.saturating_sub(1)
                        }
                    }
                };
            }

            KeyCode::Down => {
                match self.active_tab {
                    HomeWindowTab::MainMenu => {
                        if self.main_cursor_index
                            == crate::screens::home::menu::HOME_MENU_OPTIONS.len() - 1
                        {
                            self.main_cursor_index = 0
                        } else {
                            self.main_cursor_index = (self.main_cursor_index + 1).min(3)
                        }
                    }
                    HomeWindowTab::RecentRepositories => {
                        if self.recent_cursor_index == 0 {
                            self.recent_cursor_index = 0
                        } else {
                            self.recent_cursor_index = (self.recent_cursor_index + 1).min(4)
                        }
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
