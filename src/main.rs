use app::App;
use crossterm::event::{Event, EventStream, KeyEventKind};
use futures_util::StreamExt;
use ratatui::DefaultTerminal;

use crate::context::Context;
mod app;
mod auth;
mod components;
mod context;
mod screens;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let result = run(terminal).await;

    ratatui::restore();
    result
}

async fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = App::new();
    let mut stream = EventStream::new();

    while !app.should_quit {
        terminal.draw(|frame| {
            let ctx = Context {
                auth_token: app.auth_token.clone(),
            };
            app.draw(frame, &ctx)
        })?;

        if let Some(action) = app.poll_auth() {
            app.handle_action(action);
        }

        if let Some(Ok(Event::Key(key))) = stream.next().await
            && key.kind == KeyEventKind::Press
        {
            app.handle_key(key);
        }
    }
    Ok(())
}
