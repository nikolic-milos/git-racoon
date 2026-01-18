use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

const FULL_BANNER_ART: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██████▒▒▒██▒██████▒▒▒▒▒██████▒▒▒█████▒▒▒██████▒▒▒██████▒▒▒██████▒▒▒██████▒▒███▒▒▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒▒▒██▒▒▒██▒▒▒▒▒▒▒██▒▒▒██▒██▒▒▒██▒██▒▒▒▒▒▒▒██▒▒▒▒▒▒▒██▒▒▒▒██▒██▒▒▒▒██▒████▒▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒███▒▒██▒▒▒██▒▒▒▒▒▒▒██████▒▒███████▒██▒▒▒▒▒▒▒██▒▒▒▒▒▒▒██▒▒▒▒██▒██▒▒▒▒██▒██▒██▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒▒██▒▒██▒▒▒██▒▒▒▒▒▒▒██▒▒▒██▒██▒▒▒██▒██▒▒▒▒▒▒▒██▒▒▒▒▒▒▒██▒▒▒▒██▒██▒▒▒▒██▒██▒▒██▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██████▒▒▒██▒▒▒██▒▒▒▒▒▒▒██▒▒▒██▒██▒▒▒██▒▒██████▒▒▒██████▒▒▒██████▒▒▒██████▒▒██▒▒▒████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

const SMALL_BANNER_ART: &str = r#"
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██████▒▒▒██████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒▒▒▒▒▒██▒▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒███▒▒██████▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██▒▒▒▒██▒▒██▒▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒██████▒▒▒██▒▒▒██▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
"#;

fn get_gradient_text(art: &str) -> Text<'static> {
    let start_rgb = (252, 74, 26);
    let end_rgb = (247, 183, 51);

    let lines: Vec<Line> = art
        .lines()
        .map(|line_str| {
            let mut spans = Vec::new();
            let len = line_str.chars().count();

            if len == 0 {
                return Line::default();
            }

            for (i, c) in line_str.chars().enumerate() {
                let t = i as f32 / len as f32;

                // Interpolate the colors to get the gradient
                let r = (start_rgb.0 as f32 + (end_rgb.0 as f32 - start_rgb.0 as f32) * t) as u8;
                let g = (start_rgb.1 as f32 + (end_rgb.1 as f32 - start_rgb.1 as f32) * t) as u8;
                let b = (start_rgb.2 as f32 + (end_rgb.2 as f32 - start_rgb.2 as f32) * t) as u8;

                // If it's a shadow block '▒' dim it to achieve contrast with the text
                let (final_r, final_g, final_b) = if c == '▒' {
                    (r / 3, g / 3, b / 3)
                } else {
                    (r, g, b)
                };

                spans.push(Span::styled(
                    c.to_string(),
                    Style::default().fg(Color::Rgb(final_r, final_g, final_b)),
                ));
            }
            Line::from(spans)
        })
        .collect();

    Text::from(lines)
}

pub fn draw(f: &mut Frame, area: Rect) {
    let terminal_rect = f.area();

    let text = if terminal_rect.width >= 90 {
        get_gradient_text(FULL_BANNER_ART)
    } else {
        get_gradient_text(SMALL_BANNER_ART)
    };

    let para = Paragraph::new(text)
        .alignment(Alignment::Center)
        .block(Block::default().borders(Borders::NONE));

    f.render_widget(para, area);
}
