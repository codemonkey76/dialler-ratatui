use std::fmt::{Display, Formatter};
use derive_more::From;

#[derive(From, Debug)]
pub enum Error {
    ConfigError(String),

    #[from]
    IoError(std::io::Error),

    #[from]
    RusqlError(rusqlite::Error),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for Error {}

pub type AppResult<T> = Result<T, Error>;
