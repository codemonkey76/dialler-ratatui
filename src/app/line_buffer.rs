use std::fmt;
use std::fmt::{Display, Formatter};
use crossterm::event::{KeyCode, KeyEvent};
use crate::app::AppResult;

#[derive(Debug, Default)]
pub struct LineBuffer {
    buffer: String,
    max_buffer: usize,
    display_buffer: usize,
    cursor_position: usize,
}

impl LineBuffer {
    pub fn handle_key_events(&mut self, event: KeyEvent) -> AppResult<()> {
        match event.code {
            KeyCode::Char(c) => {
                self.enter_char(c);
            }
            KeyCode::Enter => {
                self.reset_cursor();
                self.buffer.clear();
            }
            KeyCode::Backspace => {
                self.backspace_char();
            }
            KeyCode::Delete => {
                self.delete_char();
            }
            KeyCode::Left => {
                self.move_cursor_left();
            }
            KeyCode::Right => {
                self.move_cursor_right()
            }
            KeyCode::Home => {
                self.reset_cursor();
            }
            KeyCode::End => {
                self.move_cursor_to_end();
            }
            _ => {}
        }

        Ok(())
    }

    pub fn get_cursor_position(&self) -> usize {
        self.cursor_position
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.buffer.insert(self.cursor_position, new_char);

        self.move_cursor_right();
    }

    fn backspace_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.buffer.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.buffer.chars().skip(current_index);
            self.buffer = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn delete_char(&mut self) {
        let is_not_cursor_rightmost = self.cursor_position != self.buffer.len();
        if is_not_cursor_rightmost {
            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index;
            let before_char_to_delete = self.buffer.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.buffer.chars().skip(current_index + 1);
            self.buffer = before_char_to_delete.chain(after_char_to_delete).collect();
        }
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }

    fn move_cursor_to_end(&mut self) {
        self.cursor_position = self.buffer.len();
    }

    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.buffer.len())
    }
}

impl Display for LineBuffer {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.buffer)
    }
}

/*






    fn clamp_cursor(&mut self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
 */