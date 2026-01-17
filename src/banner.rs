use ratatui::{
    Frame,
    prelude::*,
    widgets::{Block, Padding, Paragraph},
};
use tui_banner::{Banner, Style as BannerStyle};

const SMALL_BANNER: &str = "GR";
const FULL_BANNER: &str = "GitRaccoon";

pub fn draw(f: &mut Frame, area: Rect) {
    let (banner_str, style) = if area.width < 20 {
        return;
    } else if area.width >= 80 {
        (FULL_BANNER, BannerStyle::RoyalPurple)
    } else {
        (SMALL_BANNER, BannerStyle::RoyalPurple)
    };

    let banner_text = match Banner::new(banner_str) {
        Ok(b) => b.style(style).render(),
        Err(_) => banner_str.to_string(),
    };

    let text = match ansi_to_tui::IntoText::into_text(&banner_text) {
        Ok(t) => t,
        Err(_) => Text::from(banner_text.as_str()),
    };

    let para = Paragraph::new(text)
        .alignment(HorizontalAlignment::Center)
        .block(Block::new().padding(Padding::uniform(1)));
    f.render_widget(para, area);
}
