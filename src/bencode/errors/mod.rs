use std::error::Error;
use std::fmt;
use std::num::ParseIntError;
use std::str::Utf8Error;

#[derive(Debug)]
pub enum BencodeErrorKind {
    IoError(std::io::Error),
    UnexpectedEnd,
    InvalidInteger(ParseIntError),
    InvalidUtf8(Utf8Error),
    InvalidDictionaryKey,
    UnexpectedToken(u8),
    IncompleteSequence,
}

#[derive(Debug)]
pub struct BencodeError {
    kind: BencodeErrorKind,
    position: usize,
}

impl BencodeError {
    pub fn new(kind: BencodeErrorKind, position: usize) -> Self {
        BencodeError { kind, position }
    }
}

impl fmt::Display for BencodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match &self.kind {
                BencodeErrorKind::IoError(e) => format!("I/O error: {}", e),
                BencodeErrorKind::UnexpectedEnd => "Unexpected end of input".to_string(),
                BencodeErrorKind::InvalidInteger(e) => format!("Invalid integer: {}", e),
                BencodeErrorKind::InvalidUtf8(e) => format!("Invalid UTF-8: {}", e),
                BencodeErrorKind::InvalidDictionaryKey =>
                    "Invalid dictionary key: expected byte string".to_string(),
                BencodeErrorKind::UnexpectedToken(b) => format!(
                    "Unexpected token: 0x{:02X} at position {}",
                    b, self.position
                ),
                BencodeErrorKind::IncompleteSequence => "Incomplete sequence".to_string(),
            }
        )
    }
}

impl Error for BencodeError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match &self.kind {
            BencodeErrorKind::IoError(e) => Some(e),
            BencodeErrorKind::InvalidInteger(e) => Some(e),
            BencodeErrorKind::InvalidUtf8(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for BencodeError {
    fn from(error: std::io::Error) -> Self {
        BencodeError {
            kind: BencodeErrorKind::IoError(error),
            position: 0,
        }
    }
}
