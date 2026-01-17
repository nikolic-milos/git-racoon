use ratatui::{Frame, prelude::*};

pub fn split_home_area(area: Rect) -> [Rect; 2] {
    let vertical_split = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let bottom_horizontal_split = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_split[1]);

    [bottom_horizontal_split[0], bottom_horizontal_split[1]]
}
