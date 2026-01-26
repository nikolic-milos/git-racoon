#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HomeWindowTab {
    MainMenu,
    RecentRepositories,
}

pub const RECENT_REPOS_MAX_LINES: usize = 5;
pub const ACTIVITY_FEED_MAX_LINES: usize = 10;

#[derive(Debug, PartialEq, Clone)]
pub struct HomeWindow {
    pub active_tab: HomeWindowTab,
    pub main_cursor_index: usize,
    pub recent_cursor_index: usize,
}
