use ratatui::layout::HorizontalAlignment;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};
pub fn draw(f: &mut Frame, area: Rect) {
    let text = "Logged out";
    let para = Paragraph::new(text)
        .alignment(HorizontalAlignment::Right)
        .style(Style::default().fg(Color::Red));
    f.render_widget(para, area);
}
