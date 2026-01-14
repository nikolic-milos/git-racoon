use crate::assets::JOHNNY;
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum CurrentScreen {
    Home,
    RecentPopup,
    Repository,
}

pub const HOME_MENU_OPTIONS: [&str; 4] = ["Open", "Clone", "Recent Repositories", "Exit"];

pub struct App {
    pub should_quit: bool,
    pub status_text: String,
    pub home_menu_index: usize,
    current_screen: CurrentScreen,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            current_screen: CurrentScreen::Home,
            status_text: "Current directory is not a git repository".to_string(),
            home_menu_index: 0,
        }
    }

    pub fn draw(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(1),
            ])
            .split(f.area());

        // Title
        let title = Paragraph::new("Git Raccoon")
            .block(Block::default().borders(Borders::ALL).title(" Meow "));
        f.render_widget(title, chunks[0]);

        // Main content
        match self.current_screen {
            CurrentScreen::Home => self.draw_home_view(f, chunks[1]),
            CurrentScreen::Repository => self.draw_repository_view(f, chunks[1]),
            CurrentScreen::RecentPopup => self.draw_recent_repositories_popup(f, chunks[1]),
        }

        // Footer
        let help_text = match self.current_screen {
            CurrentScreen::Home => " ↑↓ / jk = move • ⏎ = select • q = quit ",
            CurrentScreen::Repository => " Esc / q = back to home ",
            CurrentScreen::RecentPopup => " Any key = close ",
        };

        let help = Paragraph::new(help_text)
            .style(Style::default().fg(Color::DarkGray))
            .alignment(HorizontalAlignment::Center);

        f.render_widget(help, chunks[2]);
    }

    fn draw_home_view(&self, f: &mut Frame, area: Rect) {
        let horizontal = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(area);

        // Left: Johnny ASCII
        let johnny = Paragraph::new(JOHNNY)
            .alignment(HorizontalAlignment::Center)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(johnny, horizontal[0]);

        // Right: Menu
        let menu_lines: Vec<String> = HOME_MENU_OPTIONS
            .iter()
            .enumerate()
            .map(|(i, &option)| {
                if i == self.home_menu_index {
                    format!(" ► {} ◄ ", option)
                } else {
                    format!("   {}   ", option)
                }
            })
            .collect();

        let menu = Paragraph::new(menu_lines.join("\n"))
            .alignment(HorizontalAlignment::Center)
            .block(Block::default().borders(Borders::ALL).title(" Main Menu "));

        f.render_widget(menu, horizontal[1]);
    }

    fn draw_repository_view(&self, f: &mut Frame, area: Rect) {
        let content = Paragraph::new(format!("Repository View\n\n{}", self.status_text))
            .block(Block::default().borders(Borders::ALL).title(" Repository "))
            .alignment(HorizontalAlignment::Center);

        f.render_widget(content, area);
    }

    fn draw_recent_repositories_popup(&self, f: &mut Frame, area: Rect) {
        let text = "Recent repositories list coming soon...\n\n(press any key to close)";

        let popup = Paragraph::new(text)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(" Recent Repositories "),
            )
            .alignment(HorizontalAlignment::Center)
            .style(Style::default().fg(Color::Cyan));

        let popup_area = centered_rect(60, 40, area);
        f.render_widget(popup, popup_area);
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        match self.current_screen {
            CurrentScreen::Home => self.handle_home_keys(key),
            CurrentScreen::Repository => self.handle_repository_keys(key),
            CurrentScreen::RecentPopup => {
                self.current_screen = CurrentScreen::Home;
            }
        }
    }

    fn handle_home_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.should_quit = true;
            }
            KeyCode::Char('j') | KeyCode::Down => {
                if self.home_menu_index < HOME_MENU_OPTIONS.len() - 1 {
                    self.home_menu_index += 1;
                }
            }
            KeyCode::Char('k') | KeyCode::Up => {
                self.home_menu_index = self.home_menu_index.saturating_sub(1);
            }
            KeyCode::Enter => match self.home_menu_index {
                0 | 1 => self.current_screen = CurrentScreen::Repository,
                2 => self.current_screen = CurrentScreen::RecentPopup,
                3 => self.should_quit = true,
                _ => {}
            },
            _ => {}
        }
    }

    fn handle_repository_keys(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Esc => {
                self.current_screen = CurrentScreen::Home;
            }
            _ => {}
        }
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_width = r.width * percent_x / 100;
    let popup_height = r.height * percent_y / 100;

    let left_margin = (r.width.saturating_sub(popup_width)) / 2;
    let top_margin = (r.height.saturating_sub(popup_height)) / 2;

    Rect {
        x: r.x + left_margin,
        y: r.y + top_margin,
        width: popup_width,
        height: popup_height,
    }
}
