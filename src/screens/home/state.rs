#[derive(Debug, PartialEq, Clone, Copy)]
pub enum HomeWindowTab {
    MainMenu,
    RecentRepositories,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct HomeWindow {
    pub active_tab: HomeWindowTab,
    pub main_cursor_index: usize,
    pub recent_cursor_index: usize,
}
