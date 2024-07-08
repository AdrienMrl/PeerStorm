use std::io::Read;

use std::str::FromStr;

use super::{BencodeError, BencodeErrorKind, ParserContext};

pub fn parse_integer<R: Read>(context: &mut ParserContext<R>) -> Result<i64, BencodeError> {
    let start_position = context.position;

    let first_byte = context.read_byte()?;
    if first_byte != b'i' {
        return Err(BencodeError::new(
            BencodeErrorKind::UnexpectedToken(first_byte),
            start_position,
        ));
    }

    let mut ascii_number = context.read_until(b'e')?;

    if ascii_number.last() != Some(&b'e') {
        return Err(BencodeError::new(
            BencodeErrorKind::UnexpectedEnd,
            context.position,
        ));
    }

    ascii_number.pop();

    let string_number = std::str::from_utf8(&ascii_number)
        .map_err(|e| BencodeError::new(BencodeErrorKind::InvalidUtf8(e), start_position + 1))?;

    let number = i64::from_str(string_number)
        .map_err(|e| BencodeError::new(BencodeErrorKind::InvalidInteger(e), start_position + 1))?;

    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_parses_simple_bencode_number() -> Result<(), BencodeError> {
        let bencode_bytes: Vec<u8> = vec![b'i', b'4', b'2', b'5', b'1', b'e'];
        let mut cursor = std::io::Cursor::new(bencode_bytes);
        let mut context: ParserContext<&mut std::io::Cursor<Vec<u8>>> =
            ParserContext::new(&mut cursor);
        let result = parse_integer(&mut context)?;
        assert_eq!(result, 4251);
        Ok(())
    }
}
