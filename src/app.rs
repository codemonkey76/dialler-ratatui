pub mod config;
pub mod line_buffer;

use crate::app::config::{AppResult, Config};
use crossterm::cursor::SetCursorStyle;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use rusqlite::Connection;
use std::sync::{Arc, Mutex};
use std::{default, error};
use tracing::info;

use crate::app::line_buffer::LineBuffer;
use crate::contact::{Contact, ContactForUpdate};
use crate::db::Db;
use crate::event::Event;

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

        Ok(Self {
            conn,
            ..Default::default()
        })
    }

    pub fn tick(&mut self) -> AppResult<()> {
        if self.state.filter.updated {
            self.state.filter.updated = false;
            self.get_contacts()?
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.running = false;
    }

    pub fn get_contacts(&mut self) -> AppResult<()> {
        let db = Db::new(self.conn.clone());
        self.state.contacts = db.list(self.state.filter.to_string())?;

        Ok(())
    }

    pub fn insert_contact(&self) {
        info!("Running insert_contact");
        let db = Db::new(self.conn.clone());

        let c = ContactForUpdate {
            first_name: Some("Shane".into()),
            last_name: Some("Poppleton".into()),
            company_name: Some("Alpha IT Centre".into()),
            phone_number: "0400 588 588".into(),
        };

        let _ = db.insert(c);
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
            (_, KeyCode::Esc) => self.quit(),
            (KeyModifiers::CONTROL, KeyCode::Char(c)) => match c {
                'q' => self.quit(),
                'c' => {
                    info!("Received Ctrl-c");
                    self.insert_contact();
                }
                _ => {}
            },
            (_, KeyCode::Insert) => {
                self.type_mode = match self.type_mode {
                    TypingMode::Insert => TypingMode::Overwrite,
                    TypingMode::Overwrite => TypingMode::Insert,
                };
            }
            _ => match self.mode {
                AppMode::Filtering => self.state.filter.handle_key_events(key_event, type_mode)?,
                AppMode::Editing => {}
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
    pub filter: LineBuffer,
    pub contacts: Vec<Contact>,
}

#[derive(Debug)]
pub enum AppMode {
    Filtering,
    Editing,
}
