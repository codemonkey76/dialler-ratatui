use crate::error::{AppResult, Error};
use directories::ProjectDirs;
use rusqlite::Connection;
use std::path::PathBuf;

pub struct Config;

impl Config {
    fn get_config_file(file: &str) -> AppResult<PathBuf> {
        Ok(ProjectDirs::from("com", "Shane Poppleton", "Dialler")
            .ok_or(Error::ConfigError(
                "No valid home directory path could be retrieved from the operating system."
                    .to_string(),
            ))?
            .config_dir()
            .join(file))
    }

    pub fn create_db() -> AppResult<Connection> {
        let db_file = Config::get_config_file("contacts.sqlite")?;
        let parent = db_file.parent().ok_or(Error::ConfigError(
            "Unable to find parent of config folder".to_string(),
        ))?;

        std::fs::create_dir_all(parent).map_err(|_| {
            Error::ConfigError("Unable to create parent folders of config folder".to_string())
        })?;
        let conn = Connection::open(db_file)?;

        // Check if table exists
        conn.execute(
            "create table if not exists contacts (
                id INTEGER PRIMARY KEY,
                first_name TEXT,
                last_name TEXT,
                phone_number TEXT NOT NULL UNIQUE,
                company_name TEXT
            )",
            [],
        )?;

        Ok(conn)
    }
}
