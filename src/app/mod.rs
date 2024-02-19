use crate::config::Config;
use crate::data_layer::contact::{Contact, ContactForUpdate};
use crate::dialog::modal::{DialogResult, Modal};
use crate::error::AppResult;

use crate::data_layer::db::Db;
use crate::event::Event;
use crate::line_buffer::LineBuffer;
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use tracing::info;

#[derive(Default, Debug, PartialEq, Clone)]
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
    conn: Arc<Mutex<Option<Connection>>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            type_mode: TypingMode::Insert,
            mode: AppMode::Filtering,
            state: AppState::default(),
            conn: Arc::default(),
        }
    }
}

impl App {
    pub fn new() -> AppResult<Self> {

        let conn = Arc::new(Mutex::new(Some(Config::create_db()?)));

        let modal = Modal::add_contact();
        let state = AppState {
            focus: AppFocus::Filter,
            selected_contact_index: 0,
            filter: Default::default(),
            contacts: vec![],
            config: Config::new()?,
            modal,
        };
        Ok(Self {
            conn,
            state,
            ..Default::default()
        })
    }

    pub fn tick(&mut self) -> AppResult<()> {
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_contacts(&mut self) -> AppResult<()> {
        let db = Db::new(self.conn.clone());
        self.state.contacts = db.list(self.state.filter.to_string())?;
        self.state.selected_contact_index = 0;

        Ok(())
    }

    pub fn add_contact(&mut self) {
        self.mode = AppMode::AddingContact;
    }

    pub fn insert_contact(&self) {
        info!("Running insert_contact");
        let db = Db::new(self.conn.clone());

        let c = ContactForUpdate {
            first_name: Some(self.state.modal.fields[0].to_string()),
            last_name: Some(self.state.modal.fields[1].to_string()),
            phone_number: self.state.modal.fields[2].to_string(),
            company_name: Some(self.state.modal.fields[3].to_string()),
        };

        let _ = db.insert(c);
    }

    pub fn delete_contact(&self) {
        let db = Db::new(self.conn.clone());
        let c = &self.state.contacts[self.state.selected_contact_index];

        let _ = db.delete(c.id);
    }

    pub fn confirm_delete_contact(&mut self) {
        self.mode = AppMode::DeletingContact;
    }
    pub fn call_selected_contact(&self) {
        let c = &self.state.contacts[self.state.selected_contact_index];
        info!("Calling \"{}\"", c.phone_number);
        let _ = std::process::Command::new(&self.state.config.dialler_program)
            .arg(c.phone_number.replace(' ', ""))
            .spawn();
    }
    pub fn handle_event(&mut self, event: Event) -> AppResult<()> {
        match event {
            Event::Tick => self.tick()?,
            Event::Key(key_event) => self.handle_key_event(key_event, self.type_mode.clone())?,
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event)?,
            Event::Resize(_, _) => {}
        }

        Ok(())
    }

    pub fn handle_mouse_event(&mut self, _: MouseEvent) -> AppResult<()> {
        Ok(())
    }

    pub fn handle_key_event(
        &mut self,
        key_event: KeyEvent,
        type_mode: TypingMode,
    ) -> AppResult<()> {
        info!("Handling key event {:?}", key_event);
        match (key_event.modifiers, key_event.code) {
            (_, KeyCode::Esc) if self.mode == AppMode::Filtering => self.quit(),
            (KeyModifiers::CONTROL, KeyCode::Char(c)) => match c {
                'q' => self.quit(),
                'a' => {
                    self.add_contact();
                }
                'c' => self.call_selected_contact(),
                'd' => {

                    if (self.state.contacts.get(self.state.selected_contact_index)).is_some() {
                        self.confirm_delete_contact();
                    } else {
                        self.state.selected_contact_index = 0;
                    }
                }
                _ => {}
            },
            (_, KeyCode::Insert) => {
                self.type_mode = match self.type_mode {
                    TypingMode::Insert => TypingMode::Overwrite,
                    TypingMode::Overwrite => TypingMode::Insert,
                };
            }
            (_, KeyCode::Down) => {
                // If there is more contacts go down
                if self.state.selected_contact_index < self.state.contacts.len() - 1 {
                    self.state.selected_contact_index += 1;
                }
            }
            (_, KeyCode::Up) => {
                if self.state.selected_contact_index > 0 {
                    self.state.selected_contact_index -= 1;
                }
            }
            _ => match self.mode {
                AppMode::Filtering => {
                    self.state.filter.handle_key_events(key_event, type_mode)?;
                    if self.state.filter.updated {
                        self.state.selected_contact_index = 0;
                        self.get_contacts()?
                    }
                }
                AppMode::AddingContact => {
                    match self.state.modal.handle_key_events(key_event, type_mode)? {
                        DialogResult::Ok => {
                            self.insert_contact();
                            self.get_contacts()?;
                            self.mode = AppMode::Filtering;
                        }
                        DialogResult::Cancel => self.mode = AppMode::Filtering,
                        _ => {}
                    }
                }
                AppMode::DeletingContact => match key_event.code {
                    KeyCode::Char('y') => {
                        info!("Deleting contact");
                        self.delete_contact();
                        self.get_contacts()?;
                        self.mode = AppMode::Filtering;
                    }
                    _ => self.mode = AppMode::Filtering,
                },
                _ => {}
            },
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
    pub focus: AppFocus,
    pub selected_contact_index: usize,
    pub filter: LineBuffer,
    pub contacts: Vec<Contact>,
    pub config: Config,
    pub modal: Modal,
}

#[derive(Debug, Default)]
pub enum AppFocus {
    #[default]
    Filter,
    Contacts,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppMode {
    Filtering,
    AddingContact,
    EditingContact,
    DeletingContact,
}
