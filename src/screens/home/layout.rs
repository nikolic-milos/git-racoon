use ratatui::prelude::*;

// Splits the home area into chunks and returns them so they can be used for positioning components
/*

Home:
-----------------------
| horizontal_split[0] | 30% - Banner
|---------------------|
| horizontal_split[1] | 40% - Menus
|---------------------|
| horizontal_split[2] | 10% - Tip of the day
|---------------------|
| horizontal_split[3] | 20% - Navigation and login status
-----------------------

horizontal_split[1]:
-------------------------------------------------------
| bottom_vertical_split[0] | bottom_vertical_split[1] |
-------------------------------------------------------
            50%                        50%
             |                          |
     Recent Repositories            Main Menu
*/
pub fn split_home_area(area: Rect) -> [Rect; 6] {
    // 1. Splits the home view into the 4 areas
    let horizontal_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(30),
            Constraint::Percentage(40),
            Constraint::Percentage(10),
            Constraint::Length(2),
        ])
        .split(area);

    // 2. Splits the menu area into left and right halves
    let menu_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_split[1]);

    // 3. Splits the navigation and login status into bottom and top halves
    let bottom_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_split[3]);

    [
        horizontal_split[0], // Banner
        horizontal_split[2], // Tip
        bottom_split[0],     // Navigation bar
        bottom_split[1],     // Login status
        menu_split[0],       // Recent Repositories Menu
        menu_split[1],       // Main Menu
    ]
}
