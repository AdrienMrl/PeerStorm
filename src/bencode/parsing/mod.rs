use std::io::Read;

use parser_context::ParserContext;

use std::str::FromStr;

use super::{errors::*, BencodeValue};

mod byte_string;
mod integer;
mod list;
mod parser_context;

pub fn parse_bencode<R: Read>(
    context: &mut ParserContext<R>,
) -> Result<Vec<BencodeValue>, BencodeError> {
    parse_bencode_to_end(context, None, true)
}

pub fn parse_bencode_to_end<R: Read>(
    context: &mut ParserContext<R>,
    end: Option<u8>,
    consume_end: bool,
) -> Result<Vec<BencodeValue>, BencodeError> {
    let mut bencode_values = Vec::new();
    loop {
        let peek = context.try_peek_byte();
        if peek == end {
            if consume_end && end.is_some() {
                context.read_byte()?;
            }
            break;
        }
        match peek {
            Some(b'i') => {
                let integer = integer::parse_integer(context)?;
                bencode_values.push(BencodeValue::Integer(integer));
            }
            Some(b'0'..=b'9') => {
                let bytes_string = byte_string::parse_string(context)?;
                bencode_values.push(BencodeValue::ByteString(bytes_string));
            }
            Some(b'l') => {
                let list = list::parse_list(context)?;
                bencode_values.push(BencodeValue::List(list));
            }
            _ => {
                return Err(BencodeError::new(
                    BencodeErrorKind::UnexpectedToken(context.peek_byte()?),
                    context.position,
                ));
            }
        }
    }
    Ok(bencode_values)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_parses_a_mix_of_string_and_integer() -> Result<(), BencodeError> {
        let bencode_bytes: Vec<u8> = vec![
            // First string and integer
            b'3', b':', b'o', b'n', b'e', // "one"
            b'i', b'-', b'1', b'2', b'3', b'e', // -123
            // Second string and integer
            b'3', b':', b't', b'w', b'o', // "two"
            b'i', b'4', b'5', b'6', b'7', b'e', // 4567
            // Third string and integer
            b'5', b':', b't', b'h', b'r', b'e', b'e', // "three"
            b'i', b'-', b'8', b'9', b'0', b'e', // -890
            // Fourth string and integer
            b'4', b':', b'f', b'o', b'u', b'r', // "four"
            b'i', b'1', b'2', b'3', b'4', b'5', b'e', // 12345
            // Fifth string and integer
            b'4', b':', b'f', b'i', b'v', b'e', // "five"
            b'i', b'-', b'6', b'7', b'8', b'9', b'e', // -6789
        ];
        let mut cursor = std::io::Cursor::new(bencode_bytes);
        let mut context = ParserContext::new(&mut cursor);
        let result = parse_bencode(&mut context)?;
        let expected = vec![
            BencodeValue::ByteString(b"one".to_vec()),
            BencodeValue::Integer(-123),
            BencodeValue::ByteString(b"two".to_vec()),
            BencodeValue::Integer(4567),
            BencodeValue::ByteString(b"three".to_vec()),
            BencodeValue::Integer(-890),
            BencodeValue::ByteString(b"four".to_vec()),
            BencodeValue::Integer(12345),
            BencodeValue::ByteString(b"five".to_vec()),
            BencodeValue::Integer(-6789),
        ];
        assert_eq!(result, expected);
        Ok(())
    }
}
