use rusqlite::{Connection, Error as RusqError};
use std::env;
use std::error::Error as StdErr;
use std::fmt;

const ENV_DB_PATH: &'static str = "CH3F_DB_PATH";

#[derive(Debug)]
pub enum Error {
    Connection { err: RusqError, db_path: String },
    NoDbPath,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::NoDbPath => write!(f, "No env var for database specified ({})", ENV_DB_PATH),
            Error::Connection { err, db_path } => {
                write!(f, "Failed to connect to database at '{}'. {}", db_path, err)
            }
        }
    }
}

impl StdErr for Error {}

pub fn establish_connection() -> Result<Connection, Error> {
    let db_path = match env::var(ENV_DB_PATH) {
        Ok(db_path) => Ok(db_path),
        Err(_) => Err(Error::NoDbPath),
    }?;
    match Connection::open(&db_path) {
        Ok(conn) => Ok(conn),
        Err(err) => Err(Error::Connection { err, db_path }),
    }
}
