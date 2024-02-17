use dialler_rs::app::config::AppResult;
use dialler_rs::app::App;
use dialler_rs::event::EventHandler;
use dialler_rs::tui::Tui;
use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use std::io;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> AppResult<()> {
    let file_appender = tracing_appender::rolling::daily("/home/shane/logs", "dialler.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = FmtSubscriber::builder()
        .with_writer(non_blocking)
        .without_time()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Could not set default tracing subscriber");

    info!("Started Application");
    let mut app = App::new()?;
    app.get_contacts()?;

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
