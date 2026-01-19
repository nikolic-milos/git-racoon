use crate::screens::home::{
    banner, controls, login_status, menu,
    state::{HomeWindow, HomeWindowTab},
    tip,
};
use ratatui::prelude::*;

// Splits the home area into chunks and returns them so they can be used for positioning components
/*

Home:
-----------------------
| upper_split[0] | 30% - Banner
|---------------------|
| upper_split[1] | 35% - Menus
|---------------------|
| upper_split[2] | 10% - Tip of the day
|---------------------|
| upper_split[3] | 20% - Navigation and login status
-----------------------

upper_split[1]:
-------------------------------------------------------
| bottom_vertical_split[0] | bottom_vertical_split[1] |
-------------------------------------------------------
            50%                        50%
             |                          |
     Recent Repositories            Main Menu
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

    // 1. Splits the home into 4 vertical areas and 2 gaps
    let upper_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(25), // Banner
            Constraint::Length(1),      // Gap
            Constraint::Percentage(20), // Menus
            Constraint::Length(1),      // Gap
            Constraint::Percentage(15), // Tip
            Constraint::Min(0),
        ])
        .split(upper_rect);

    // 2. Splits menus into left/right halves
    let menu_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(upper_split[2]);

    // 3. Sets the calculation for the fixed position of the navigation bar (second to last line in the terminal)
    let controls_rect = Rect {
        x: 0,
        y: terminal_rect.height - 2,
        width: terminal_rect.width,
        height: 1,
    };

    // 4. Sets the calculation for the fixed position of the login stats (last lne in the terminal)
    let status_rect = Rect {
        x: 0,
        y: terminal_rect.height - 1,
        width: terminal_rect.width,
        height: 1,
    };

    banner::draw(f, upper_split[0]);
    tip::draw(f, upper_split[4]);
    controls::draw(f, controls_rect);
    login_status::draw(f, status_rect);
    menu::draw_main_menu(
        f,
        menu_split[1],
        state.main_cursor_index,
        state.active_tab == HomeWindowTab::MainMenu,
    );
    menu::draw_recent_repos(
        f,
        menu_split[0],
        state.recent_cursor_index,
        state.active_tab == HomeWindowTab::RecentRepositories,
        &state.recent_repositories,
    );
}
