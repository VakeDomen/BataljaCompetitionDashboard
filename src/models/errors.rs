use std::path::Path;
use std::{fmt, io};
use std::error::Error;

pub enum MatchMakerError {
    DatabaseError(diesel::result::Error),
    IOError(io::Error),
    InvalidPath(Box<Path>),
    TimeoutError,
    GameThreadJoinError,
    InvalidOutput,
    GameProcessFailed,
}

// Implement std::fmt::Display for MatchMakerError
impl fmt::Display for MatchMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchMakerError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            MatchMakerError::IOError(err) => write!(f, "IO Error: {}", err),
            MatchMakerError::InvalidPath(path) => write!(f, "Invalid path {:#?}", path),
            MatchMakerError::TimeoutError => writeln!(f, "GameTimeout Error"),
            MatchMakerError::GameThreadJoinError => writeln!(f, "GameThreadJoin Error"),
            MatchMakerError::InvalidOutput => writeln!(f, "InvalidOutput Error"),
            MatchMakerError::GameProcessFailed => writeln!(f, "GameProcessFailed Error"),
        }
    }
}

// Implement std::fmt::Debug for MatchMakerError
impl fmt::Debug for MatchMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchMakerError::DatabaseError(err) => write!(f, "MatchMakerError::DatabaseError: {:?}", err),
            MatchMakerError::IOError(err) => write!(f, "MatchMakerError::IOError: {:?}", err),
            MatchMakerError::InvalidPath(path) => write!(f, "MatchMakerError::InvalidPath: {:?}", path),
            MatchMakerError::TimeoutError => writeln!(f, "MatchMakerError::TimeoutError"),
            MatchMakerError::GameThreadJoinError => writeln!(f, "MatchMakerError::GameThreadJoinError"),
            MatchMakerError::InvalidOutput => writeln!(f, "MatchMakerError::InvalidOutput"),
            MatchMakerError::GameProcessFailed => writeln!(f, "MatchMakerError::GameProcessFailed"),
        }
    }
}

// Implement std::error::Error for MatchMakerError
impl Error for MatchMakerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MatchMakerError::DatabaseError(err) => Some(err),
            MatchMakerError::IOError(err) => Some(err),
            MatchMakerError::InvalidPath(path) => None,
            MatchMakerError::TimeoutError => None,
            MatchMakerError::GameThreadJoinError => None,
            MatchMakerError::InvalidOutput => None,
            MatchMakerError::GameProcessFailed => None,
        }
    }
}
