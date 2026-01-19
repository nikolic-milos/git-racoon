use crate::screens::home::state::ACTIVITY_FEED_MAX_LINES;
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub fn draw(f: &mut Frame, area: Rect, recent_activities: &[String]) {
    let lines = if recent_activities.is_empty() {
        vec!["No recent activites.".to_string()]
    } else {
        recent_activities
            .iter()
            .take(ACTIVITY_FEED_MAX_LINES)
            .map(|activity| format!("  {}  ", activity))
            .collect()
    };

    let activity_feed = Paragraph::new(lines.join("\n"))
        .alignment(HorizontalAlignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Activity Feed "),
        );

    f.render_widget(activity_feed, area);
}
