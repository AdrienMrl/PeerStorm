use std::io::{BufRead, BufReader, Read};
use std::str::FromStr;

use super::{BencodeError, BencodeErrorKind};

pub struct ParserContext<R: Read> {
    pub buffer: BufReader<R>,
    pub position: usize,
}

impl<R: Read> ParserContext<R> {
    pub fn new(reader: R) -> Self {
        ParserContext {
            buffer: BufReader::new(reader),
            position: 0,
        }
    }

    pub fn try_peek_byte(&mut self) -> Option<u8> {
        self.buffer
            .fill_buf()
            .ok()
            .and_then(|buf| buf.first().copied())
    }

    pub fn peek_byte(&mut self) -> Result<u8, BencodeError> {
        let buffer = self
            .buffer
            .fill_buf()
            .map_err(|e| BencodeError::new(super::BencodeErrorKind::IoError(e), self.position))?;
        buffer
            .first()
            .copied()
            .ok_or_else(|| BencodeError::new(super::BencodeErrorKind::UnexpectedEnd, self.position))
    }

    pub fn read_byte(&mut self) -> Result<u8, BencodeError> {
        let byte = self.peek_byte()?;
        self.position += 1;
        self.buffer.consume(1);
        Ok(byte)
    }

    pub fn read_until(&mut self, byte: u8) -> Result<Vec<u8>, BencodeError> {
        let mut buf = Vec::new();
        let read = self.buffer.read_until(byte, &mut buf)?;
        self.position += read;
        Ok(buf)
    }

    pub fn read_exact(&mut self, size: usize) -> Result<Vec<u8>, BencodeError> {
        let mut buf = vec![0; size];
        self.buffer.read_exact(&mut buf)?;
        self.position += size;
        Ok(buf)
    }

    pub fn parse_utf8<'a>(&self, bytes: &'a [u8]) -> Result<&'a str, BencodeError> {
        std::str::from_utf8(bytes)
            .map_err(|e| BencodeError::new(BencodeErrorKind::InvalidUtf8(e), self.position))
    }

    pub fn parse_i64(&self, bytes: &[u8]) -> Result<i64, BencodeError> {
        let string_number = self.parse_utf8(bytes)?;
        i64::from_str(string_number)
            .map_err(|e| BencodeError::new(BencodeErrorKind::InvalidInteger(e), self.position))
    }

    pub fn parse_usize(&self, bytes: &[u8]) -> Result<usize, BencodeError> {
        let string_number = self.parse_utf8(bytes)?;
        usize::from_str(string_number)
            .map_err(|e| BencodeError::new(BencodeErrorKind::InvalidInteger(e), self.position))
    }
}
