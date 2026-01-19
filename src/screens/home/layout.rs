use crate::screens::home::{
    activity_feed, banner, controls, login_status, menu,
    state::{HomeWindow, HomeWindowTab},
};
use ratatui::prelude::*;

// Splits the home area into chunks and returns them so they can be used for positioning components
/*

Home:
-----------------------
|   initial_split[0]  | 30% - Banner
|---------------------|
|   initial_split[2]  | 65% - Main content area
-----------------------


initial_split[2]:
-------------------------------------------------------
| content_area_split[0]    |    content_area_split[1] |
-------------------------------------------------------
            50%                        50%
             |                          |
           Menus                  Activity Feed

content_area_split[0]:
-------------------------
|   menu_area_split [0] | 50% - Main Menu
|-----------------------|
|   menu_area_split [1] | 50% - Recent Repositories
-------------------------
*/
pub fn apply(f: &mut Frame, state: &HomeWindow) {
    let terminal_rect = f.area();
    let upper_height = terminal_rect.height.saturating_sub(2);
    let upper_rect = Rect {
        x: 0,
        y: 0,
        width: terminal_rect.width,
        height: upper_height,
    };

    // 1. Split the home into 4 vertical areas and 2 gaps
    let initial_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30), // [0] Banner
            Constraint::Length(1),      // Gap
            Constraint::Percentage(65), // [2] Menus & Activity
            Constraint::Length(3),      // Gap
            Constraint::Min(0),
        ])
        .split(upper_rect);

    // 2. Split the main content area into left/right halves
    let content_area_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(initial_split[2]);

    // 3. Split the left half into top/bottom halves
    let menu_area_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage((50)), Constraint::Percentage((50))])
        .split(content_area_split[0]);

    // 4. Sets the calculation for the fixed position of the navigation bar (second to last line in the terminal)
    let controls_rect = Rect {
        x: 0,
        y: terminal_rect.height - 2,
        width: terminal_rect.width,
        height: 1,
    };

    // 5. Sets the calculation for the fixed position of the login stats (last lne in the terminal)
    let status_rect = Rect {
        x: 0,
        y: terminal_rect.height - 1,
        width: terminal_rect.width,
        height: 1,
    };

    banner::draw(f, initial_split[0]);
    activity_feed::draw(f, content_area_split[1], &state.recent_activity);
    controls::draw(f, controls_rect);
    login_status::draw(f, status_rect);
    menu::draw_main_menu(
        f,
        menu_area_split[0],
        state.main_cursor_index,
        state.active_tab == HomeWindowTab::MainMenu,
    );
    menu::draw_recent_repos(
        f,
        menu_area_split[1],
        state.recent_cursor_index,
        state.active_tab == HomeWindowTab::RecentRepositories,
        &state.recent_repositories,
    );
}
