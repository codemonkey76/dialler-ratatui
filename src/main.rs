use dialler_rs::app::{App, AppResult};
use dialler_rs::event::EventHandler;
use dialler_rs::tui::Tui;
use std::io;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;

#[tokio::main]
async fn main() -> AppResult<()> {
    let mut app = App::new();

    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(250);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    while app.running {
        tui.draw(&mut app)?;

        let event = tui.events.next().await?;
        app.handle_event(event)?;
    }

    tui.exit()?;
    Ok(())
}
