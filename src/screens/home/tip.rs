use ratatui::layout::Alignment;

use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};
pub fn draw(f: &mut Frame, area: Rect) {
    let text = "Johnny's Tip: Use Tab to switch menus!";
    let para = Paragraph::new(text)
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(para, area);
}

// https://gemini.google.com/app/389bde6d554b6552
