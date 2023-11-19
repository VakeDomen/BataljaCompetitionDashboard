use std::path::Path;
use std::{fmt, io};
use std::error::Error;

use zip::result::ZipError;

pub enum MatchMakerError {
    DatabaseError(diesel::result::Error),
    IOError(io::Error),
    InvalidPath(Box<Path>),
    TimeoutError,
    GameProcessFailed,
    ZippingError(ZipError),
}

// Implement std::fmt::Display for MatchMakerError
impl fmt::Display for MatchMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchMakerError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            MatchMakerError::IOError(err) => write!(f, "IO Error: {}", err),
            MatchMakerError::InvalidPath(path) => write!(f, "Invalid path {:#?}", path),
            MatchMakerError::TimeoutError => writeln!(f, "GameTimeout Error"),
            MatchMakerError::GameProcessFailed => writeln!(f, "GameProcessFailed Error"),
            MatchMakerError::ZippingError(err) => writeln!(f, "ZippingError: {}", err),
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
            MatchMakerError::GameProcessFailed => writeln!(f, "MatchMakerError::GameProcessFailed"),
            MatchMakerError::ZippingError(err) => writeln!(f, "MatchMakerError::ZippingError: {:?}", err),
        }
    }
}

// Implement std::error::Error for MatchMakerError
impl Error for MatchMakerError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MatchMakerError::DatabaseError(err) => Some(err),
            MatchMakerError::IOError(err) => Some(err),
            MatchMakerError::InvalidPath(_) => None,
            MatchMakerError::TimeoutError => None,
            MatchMakerError::GameProcessFailed => None,
            MatchMakerError::ZippingError(err) => Some(err),
        }
    }
}
