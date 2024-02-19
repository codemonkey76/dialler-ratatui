use crate::app::TypingMode;
use crate::dialog::dialog_field::DialogField;

use crate::error::AppResult;
use crossterm::event::{KeyCode, KeyEvent};
use tracing::info;

#[derive(Default, Debug)]
pub struct Modal {
    pub focused_index: usize,
    pub fields: Vec<DialogField>,
}

impl Modal {
    pub fn get_max_label(&self) -> u16 {
        self.fields
            .iter()
            .map(|field| field.label.len())
            .max()
            .unwrap_or(0) as u16
    }
    pub fn add_contact() -> Self {
        let mut modal = Modal::default();
        modal.fields.push(DialogField::new("first", "First Name"));
        modal.fields.push(DialogField::new("last", "Last Name"));
        modal.fields.push(DialogField::new("phone", "Phone Number"));
        modal.fields.push(DialogField::new("company", "Company"));

        modal
    }

    pub fn handle_key_events(
        &mut self,
        event: KeyEvent,
        type_mode: TypingMode,
    ) -> AppResult<DialogResult> {
        let mut result = DialogResult::None;

        match event.code {
            KeyCode::Tab => {
                self.focus_next();
            }
            KeyCode::BackTab => {
                self.focus_previous();
            }
            KeyCode::Enter => {
                info!("Setting Dialog OK");
                result = DialogResult::Ok;
            }
            KeyCode::Esc => result = DialogResult::Cancel,
            _ => {
                self.fields[self.focused_index].handle_key_events(event, type_mode)?;
            }
        }

        Ok(result)
    }

    fn focus_previous(&mut self) {
        if self.focused_index == 0 {
            self.focused_index = self.fields.len() - 1
        } else {
            self.focused_index -= 1;
        }
    }
    fn focus_next(&mut self) {
        if self.focused_index == self.fields.len() - 1 {
            self.focused_index = 0;
        } else {
            self.focused_index += 1;
        }
    }
}

pub enum DialogResult {
    Ok,
    Cancel,
    None,
}
