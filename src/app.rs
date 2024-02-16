pub mod line_buffer;

use std::error;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use crate::app::line_buffer::LineBuffer;
use crate::event::Event;

pub type AppResult<T> = Result<T, Box<dyn error::Error>>;

#[derive(Default, Debug, PartialEq)]
pub enum TypingMode {
    #[default]
    Insert,
    Overwrite,
}

#[derive(Debug)]
pub struct App {
    pub running: bool,
    type_mode: TypingMode,
    pub mode: AppMode,
    pub state: AppState,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            type_mode: TypingMode::Insert,
            mode: AppMode::Filtering,
            state: AppState::default()
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) {}

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn handle_event(&mut self, event: Event) -> AppResult<()> {
        match event {
            Event::Tick =>self.tick(),
            Event::Key(key_event) => self.handle_key_event(key_event)?,
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event)?,
            Event::Resize(_, _) => {}
        }

        Ok(())
    }

    pub fn handle_mouse_event(&mut self, _: MouseEvent) -> AppResult<()> {
        Ok(())
    }

    pub fn handle_key_event(&mut self, key_event: KeyEvent) -> AppResult<()> {
        match (key_event.modifiers, key_event.code) {
            (KeyModifiers::CONTROL, KeyCode::Char(c)) => {
                match c {
                    'c' | 'q' => self.quit(),
                    _ => {}
                }
            }
            (_, KeyCode::Insert) => {
                self.type_mode = match self.type_mode {
                    TypingMode::Insert => TypingMode::Overwrite,
                    TypingMode::Overwrite => TypingMode::Insert,
                };
            }
            _ => {
                match self.mode {
                    AppMode::Filtering => self.state.filter.handle_key_events(key_event)?,
                    AppMode::Editing => {}
                }
            }
        }
        Ok(())

    }

    pub fn get_cursor_style(&self) -> SetCursorStyle {
        if self.type_mode == TypingMode::Insert {
            SetCursorStyle::BlinkingBlock
        } else {
            SetCursorStyle::BlinkingUnderScore
        }
    }
}

#[derive(Default, Debug)]
pub struct AppState {
    pub filter: LineBuffer,
}


#[derive(Debug)]
pub enum AppMode {
    Filtering,
    Editing
}

