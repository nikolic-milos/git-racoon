use crate::{
    auth::{self, github::delete_token},
    components::command_bar::{CommandBar, CommandBarAction},
    context::Context,
    screens::{Action, Screen, home::state::HomeWindow},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;

pub struct App {
    pub should_quit: bool,
    pub screen_stack: Vec<Box<dyn Screen>>,
    command_bar: Option<CommandBar>,
    pub auth_token: Option<String>,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            screen_stack: vec![Box::new(HomeWindow::new())],
            command_bar: None,
            auth_token: auth::github::load_token().unwrap_or(None),
        }
    }

    pub fn draw(&self, f: &mut Frame, ctx: &Context) {
        if let Some(bar) = &self.command_bar {
            let command_bar_height = 2;
            let chunks = ratatui::layout::Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    ratatui::layout::Constraint::Length(command_bar_height),
                    ratatui::layout::Constraint::Min(0),
                ])
                .split(f.area());

            bar.draw(f, chunks[0]);

            if let Some(screen) = self.screen_stack.last() {
                screen.draw(f, chunks[1], ctx);
            }
        } else if let Some(screen) = self.screen_stack.last() {
            screen.draw(f, f.area(), ctx);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.code == KeyCode::Char('/') && self.command_bar.is_none() {
            self.command_bar = Some(CommandBar::new());
            return;
        }

        if let Some(ref mut bar) = self.command_bar {
            match bar.handle_key(key) {
                CommandBarAction::Cancel => {
                    self.command_bar = None;
                }
                CommandBarAction::Submit => {
                    // TODO: Implement submit command..
                    self.command_bar = None
                }
                CommandBarAction::Continue => {}
            }
            return;
        }

        if let Some(screen) = self.screen_stack.last_mut() {
            let ctx = Context {
                auth_token: self.auth_token.clone(),
            };
            let action = screen.handle_keys(key, &ctx);
            self.handle_action(action);
        }
    }

    fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::GoBack => {
                if self.screen_stack.len() > 1 {
                    self.screen_stack.pop();
                }
            }
            Action::None => {}
            Action::NavigateTo => {}
            Action::Authenticate => {}
            Action::AuthSuccess(token) => {
                self.auth_token = Some(token);
                self.screen_stack.pop();
            }
            Action::Logout => {
                self.auth_token = None;
                let _ = auth::github::delete_token();
            }
            _ => {}
        }
    }
}
