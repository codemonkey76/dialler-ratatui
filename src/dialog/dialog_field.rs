use crate::app::TypingMode;
use crate::error::AppResult;
use crate::line_buffer::LineBuffer;
use crossterm::event::KeyEvent;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub struct DialogField {
    pub name: String,
    pub label: String,
    buffer: LineBuffer,
}

impl Display for DialogField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

impl DialogField {
    pub fn new(name: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            label: label.into(),
            buffer: Default::default(),
        }
    }

    pub fn reset(&mut self) {
        self.buffer.reset_cursor();
        self.buffer.clear();
    }

    pub fn handle_key_events(&mut self, event: KeyEvent, type_mode: TypingMode) -> AppResult<()> {
        self.buffer.handle_key_events(event, type_mode)?;

        Ok(())
    }

    pub fn set_value(&mut self, value: impl Into<String>) {
        self.buffer.set_value(value.into());
    }

    pub fn get_value(&self) -> &str {
        self.buffer.get_value()
    }

    pub fn get_cursor_pos(&self) -> u16 {
        self.buffer.get_cursor_position() as u16
    }
}
