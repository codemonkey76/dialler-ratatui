use crate::app::{App, AppMode};
use crate::error::AppResult;
use crate::event::EventHandler;
use crate::renderer::Renderer;
use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::backend::Backend;
use ratatui::Terminal;
use std::io;
use std::panic;

#[derive(Debug)]
pub struct Tui<B: Backend> {
    terminal: Terminal<B>,
    pub events: EventHandler,
}

impl<B: Backend> Tui<B> {
    pub fn new(terminal: Terminal<B>, events: EventHandler) -> Self {
        Self { terminal, events }
    }

    pub fn init(&mut self) -> AppResult<()> {
        terminal::enable_raw_mode()?;
        crossterm::execute!(io::stderr(), EnterAlternateScreen, EnableMouseCapture)?;

        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));

        self.terminal.hide_cursor()?;
        self.terminal.clear()?;
        Ok(())
    }

    pub fn draw(&mut self, app: &mut App) -> AppResult<()> {
        let mode = app.mode.clone();
        match mode {
            AppMode::AddingContact => self
                .terminal
                .draw(|frame| Renderer::render_add_contact_modal(app, frame))?,
            AppMode::DeletingContact => self
                .terminal
                .draw(|frame| Renderer::render_delete_confirmation_modal(app, frame))?,
            _ => self
                .terminal
                .draw(|frame| Renderer::render_main_window(app, frame))?,
        };

        crossterm::execute!(io::stderr(), app.get_cursor_style())?;
        Ok(())
    }

    fn reset() -> AppResult<()> {
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stderr(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }

    pub fn exit(&mut self) -> AppResult<()> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
