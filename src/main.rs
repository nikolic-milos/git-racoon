use app::App;
use crossterm::event::{self, Event};

use ratatui::DefaultTerminal;

mod app;
mod banner;
mod home_layout;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = run(terminal);
    ratatui::restore();
    result
}

fn run(mut terminal: DefaultTerminal) -> color_eyre::Result<()> {
    let mut app = App::new();

    while !app.should_quit {
        terminal.draw(|frame| app.draw(frame))?;

        if let Event::Key(key) = event::read()? {
            if key.kind == crossterm::event::KeyEventKind::Press {
                app.handle_key(key);
            }
        }
    }
    Ok(())
}
