use ratatui::prelude::*;

pub struct HomeLayout {
    pub banner: Rect,
    pub main_menu: Rect,
    pub recent_repos: Rect,
    pub activity_feed: Rect,
    pub controls: Rect,
    pub login_status: Rect,
}

/*
Splits the home area into chunks and returns them so they can be used for positioning components

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
           Menus:                   Activity Feed

content_area_split[0]:
-------------------------
|   menu_area_split [0] | 50% - Main Menu
|-----------------------|
|   menu_area_split [1] | 50% - Recent Repositories
-------------------------
*/
pub fn calculate_layout(area: Rect, terminal_height: u16) -> Result<HomeLayout, String> {
    const MIN_WIDTH: u16 = 20;
    const MIN_HEIGHT: u16 = 20;

    if area.height < MIN_HEIGHT || area.width < MIN_WIDTH {
        return Err("Terminal window too small. Resize your terminal.".to_string());
    }

    let upper_height = terminal_height.saturating_sub(2).saturating_sub(area.y);
    let upper_rect = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: upper_height,
    };

    // 1. Split the home into 4 vertical areas and 2 gapsarea();
    let initial_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // [0] Banner
            Constraint::Length(1),
            Constraint::Percentage(90), // [2] Menus & Activity
            Constraint::Length(1),
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
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(content_area_split[0]);

    // 4. Sets the calculation for the fixed position of the navigation bar (second to last line in the terminal)
    let controls_rect = Rect {
        x: 0,
        y: terminal_height - 2,
        width: area.width,
        height: 1,
    };

    // 5. Sets the calculation for the fixed position of the login stats (last line in the terminal)
    let status_rect = Rect {
        x: 0,
        y: terminal_height - 1,
        width: area.width,
        height: 1,
    };

    Ok(HomeLayout {
        banner: initial_split[0],
        main_menu: menu_area_split[0],
        recent_repos: menu_area_split[1],
        activity_feed: content_area_split[1],
        controls: controls_rect,
        login_status: status_rect,
    })
}
