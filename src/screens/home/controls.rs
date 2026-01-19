use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};
pub fn draw(f: &mut Frame, area: Rect) {
    let text = "↑↓←→: Navigate | tab: Switch | q: Quit | l: Login";
    let para =
        Paragraph::new(text).style(Style::default().bg(Color::White).fg(Color::Black).bold());
    f.render_widget(para, area);
}
