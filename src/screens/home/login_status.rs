use crate::context::Context;
use ratatui::layout::HorizontalAlignment;
use ratatui::{
    Frame,
    layout::Rect,
    style::{Color, Style},
    widgets::Paragraph,
};

pub fn draw(f: &mut Frame, area: Rect, ctx: &Context) {
    let (text, color) = if !ctx.is_authenticated() {
        ("Logged out", Color::Red)
    } else {
        ("Logged in", Color::Green)
    };

    let para = Paragraph::new(text)
        .alignment(HorizontalAlignment::Right)
        .style(Style::default().fg(color));
    f.render_widget(para, area);
}
