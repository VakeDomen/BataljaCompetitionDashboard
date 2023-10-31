use std::path::Path;
use std::{fmt, io};
use std::error::Error;

pub enum MatchMakerError {
    DatabaseError(diesel::result::Error),
    IOError(io::Error),
    InvalidPath(Box<Path>),
}

// Implement std::fmt::Display for MatchMakerError
impl fmt::Display for MatchMakerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MatchMakerError::DatabaseError(err) => write!(f, "Database Error: {}", err),
            MatchMakerError::IOError(err) => write!(f, "IO Error: {}", err),
            MatchMakerError::InvalidPath(path) => write!(f, "Invalid path {:#?}", path),
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
        }
    }
}
