use crate::auth::auth_error::AuthError;
use crate::auth::github::{self, request_device_code};
use crate::components::popups::AuthPopup;
use crate::components::popups::Popup;
use crate::{
    auth::{self, github::delete_token},
    components::command_bar::{CommandBar, CommandBarAction},
    context::Context,
    screens::{Action, Screen, home::state::HomeWindow},
};
use crossterm::event::{KeyCode, KeyEvent};
use ratatui::prelude::*;
use tokio::sync::mpsc;

pub struct App {
    pub should_quit: bool,
    pub screen_stack: Vec<Box<dyn Screen>>,
    command_bar: Option<CommandBar>,
    pub auth_token: Option<String>,
    device_code_rx: Option<mpsc::Receiver<(String, String, u64)>>,
    auth_rx: Option<mpsc::Receiver<Result<String, AuthError>>>,
    popup: Option<Popup>,
}

impl App {
    pub fn new() -> Self {
        App {
            should_quit: false,
            screen_stack: vec![Box::new(HomeWindow::new())],
            command_bar: None,
            auth_token: auth::github::load_token().unwrap_or(None),
            device_code_rx: None,
            auth_rx: None,
            popup: None,
        }
    }

    pub fn draw(&self, f: &mut Frame, ctx: &Context) {
        let screen_area = if let Some(bar) = &self.command_bar {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Length(2), Constraint::Min(0)])
                .split(f.area());

            bar.draw(f, chunks[0]);
            chunks[1]
        } else {
            f.area()
        };

        if let Some(screen) = self.screen_stack.last() {
            screen.draw(f, screen_area, ctx);
        }

        if let Some(popup) = &self.popup {
            let popup_width = (f.area().width * 60) / 100;
            let popup_height = 15;
            let x = f.area().x + (f.area().width - popup_width) / 2;
            let y = f.area().y + (f.area().height - popup_height) / 2;
            let popup_area = Rect::new(x, y, popup_width, popup_height);

            popup.draw(f, popup_area);
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if let Some(popup) = &mut self.popup {
            if popup.handle_key(key) {
                self.popup = None
            }
            return;
        }

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

    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Quit => self.should_quit = true,
            Action::GoBack => {
                if self.screen_stack.len() > 1 {
                    self.screen_stack.pop();
                }
            }
            Action::None => {}
            Action::NavigateTo => {}
            Action::Authenticate => self.start_auth_popup(),
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

    fn start_auth(&mut self) {
        let (tx, rx) = mpsc::channel(1);
        self.auth_rx = Some(rx);

        tokio::spawn(async move {
            let result = auth::github::authenticate_github().await;
            let _ = tx.send(result).await;
        });
    }

    fn start_auth_popup(&mut self) {
        use crate::auth::github::request_device_code;

        let (tx, rx) = mpsc::channel(1);
        self.device_code_rx = Some(rx);

        tokio::spawn(async move {
            match request_device_code().await {
                Ok(result) => {
                    let _ = tx.send(result).await;
                }
                Err(e) => {
                    eprintln!("Failed to request device code {}", e);
                }
            }
        });
    }

    pub fn poll_device_code(&mut self) {
        if let Some(rx) = &mut self.device_code_rx
            && let Ok((device_code, user_code, interval)) = rx.try_recv()
        {
            self.create_auth_popup(device_code, user_code, interval);
            self.device_code_rx = None;
        }
    }

    fn create_auth_popup(&mut self, device_code: String, user_code: String, interval: u64) {
        self.popup = Some(Popup::Auth(AuthPopup::new(user_code)));

        let (tx, rx) = mpsc::channel(1);
        self.auth_rx = Some(rx);

        tokio::spawn(async move {
            let result = crate::auth::github::poll_for_token(&device_code, interval).await;
            let _ = tx.send(result).await;
        });
    }

    pub fn poll_auth(&mut self) -> Option<Action> {
        if let Some(rx) = &mut self.auth_rx {
            match rx.try_recv() {
                Ok(Ok(token)) => {
                    if let Some(Popup::Auth(popup)) = &mut self.popup {
                        popup.status_text = "Authentication successful!".to_string();
                    }
                    self.auth_token = Some(token);
                    self.auth_rx = None;
                    None
                }
                Ok(Err(e)) => {
                    if let Some(Popup::Auth(popup)) = &mut self.popup {
                        popup.status_text = format!("Error: {}", e);
                    }
                    self.auth_rx = None;
                    None
                }
                Err(mpsc::error::TryRecvError::Empty) => None,
                Err(mpsc::error::TryRecvError::Disconnected) => {
                    self.auth_rx = None;
                    None
                }
            }
        } else {
            None
        }
    }
}
