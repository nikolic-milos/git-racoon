use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

pub const HOME_MENU_OPTIONS: [&str; 4] =
    ["Open Repository", "Clone Repository", "Settings", "Exit"];

pub fn draw_main_menu(f: &mut Frame, area: Rect, cursor: usize, is_active: bool) {
    let lines: Vec<String> = HOME_MENU_OPTIONS
        .iter()
        .enumerate()
        .map(|(i, &option)| {
            if is_active && i == cursor {
                format!(" ► {} ◄ ", option)
            } else {
                format!("   {}   ", option)
            }
        })
        .collect();
    let menu = Paragraph::new(lines.join("\n"))
        .alignment(HorizontalAlignment::Center)
        .block(Block::default().borders(Borders::ALL).title(" Main Menu "))
        .style(if is_active {
            Style::default()
        } else {
            Style::default().fg(Color::Gray)
        });
    f.render_widget(menu, area);
}

pub fn draw_recent_repos(f: &mut Frame, area: Rect, cursor: usize, is_active: bool) {
    let repos = ["Repo 1", "Repo 2", "Repo 3", "Repo 4", "Repo 5"];

    let lines: Vec<String> = repos
        .iter()
        .enumerate()
        .map(|(i, &repo)| {
            if is_active && i == cursor {
                format!("► {} ◄", repo)
            } else {
                format!("  {}  ", repo)
            }
        })
        .collect();
    let recent = Paragraph::new(lines.join("\n"))
        .alignment(HorizontalAlignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Recent Repositories "),
        )
        .style(if is_active {
            Style::default()
        } else {
            Style::default().fg(Color::Gray)
        });
    f.render_widget(recent, area);
}
