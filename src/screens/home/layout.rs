use ratatui::prelude::*;

// Splits the home area into 3 chunks and returns them, allowing for draw functions to be called on those chunks
// Layout:
/*

Frame:
-----------------------
| horizontal_split[0] | 50%
-----------------------
| horizontal_split[1] | 50%
-----------------------

horizontal_split[1]:
-------------------------------------------------------
| bottom_vertical_split[0] | bottom_vertical_split[1] |
-------------------------------------------------------
            50%                        50%
*/
pub fn split_home_area(area: Rect) -> [Rect; 3] {
    // 1. Splits the home view into top and bottom halves
    let horizontal_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(33), Constraint::Percentage(50)])
        .split(area);

    // 2. Splits the bottom half into left and right halves
    let bottom_vertical_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(horizontal_split[1]);

    [
        horizontal_split[0],
        bottom_vertical_split[0],
        bottom_vertical_split[1],
    ]
}
