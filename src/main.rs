use app::App;
use crossterm::event::{self, Event, EventStream, KeyEventKind};
use futures_util::StreamExt;
use ratatui::DefaultTerminal;
mod app;
mod components;
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
        terminal.draw(|frame| app.draw(frame))?;

        if let Some(Ok(Event::Key(key))) = stream.next().await {
            if key.kind == KeyEventKind::Press {
                app.handle_key(key);
            }
        }
    }
    Ok(())
}
