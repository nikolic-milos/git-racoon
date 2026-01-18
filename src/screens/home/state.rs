#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HomeWindowTab {
    MainMenu,
    RecentRepositories,
}

pub const RECENT_REPOS_MAX_SIZE: usize = 5;

#[derive(Debug, PartialEq, Clone)]
pub struct HomeWindow {
    pub active_tab: HomeWindowTab,
    pub main_cursor_index: usize,
    pub recent_cursor_index: usize,
    pub recent_repositories: Vec<String>,
}
