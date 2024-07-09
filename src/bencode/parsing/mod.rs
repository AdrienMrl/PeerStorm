use std::io::Read;

use parser_context::ParserContext;

use super::{errors::*, BencodeValue};

mod byte_string;
mod dictionary;
mod integer;
mod list;
mod parser_context;

pub fn parse_bencode<R: Read>(
    context: &mut ParserContext<R>,
) -> Result<Vec<BencodeValue>, BencodeError> {
    parse_bencode_to_end(context, None, false)
}

pub fn parse_bencode_to_end<R: Read>(
    context: &mut ParserContext<R>,
    end: Option<u8>,
    consume_end: bool,
) -> Result<Vec<BencodeValue>, BencodeError> {
    let mut bencode_values = Vec::new();
    loop {
        match context.try_peek_byte() {
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
            Some(b'd') => {
                let dictionary = dictionary::parse_dictionary(context)?;
                bencode_values.push(BencodeValue::Dictionary(dictionary));
            }
            None => break,
            other => {
                if other == end {
                    if consume_end {
                        context.read_byte()?;
                    }
                    break;
                }
                return Err(BencodeError::new(
                    BencodeErrorKind::UnexpectedToken(other.unwrap()),
                    context.position,
                ));
            }
        }
    }
    Ok(bencode_values)
}

pub fn parse_one_bencode<R: Read>(
    context: &mut ParserContext<R>,
) -> Result<BencodeValue, BencodeError> {
    match context.try_peek_byte() {
        Some(b'i') => integer::parse_integer(context).map(BencodeValue::Integer),
        Some(b'0'..=b'9') => byte_string::parse_string(context).map(BencodeValue::ByteString),
        Some(b'l') => list::parse_list(context).map(BencodeValue::List),
        Some(b'd') => dictionary::parse_dictionary(context).map(BencodeValue::Dictionary),
        None => Err(BencodeError::new(
            BencodeErrorKind::UnexpectedEnd,
            context.position,
        )),
        other => Err(BencodeError::new(
            BencodeErrorKind::UnexpectedToken(other.unwrap()),
            context.position,
        ))?,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn it_parses_a_mix_of_string_and_integer() {
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
        match parse_bencode(&mut context) {
            Ok(result) => {
                assert_eq!(result, expected);
            }
            Err(e) => {
                panic!("Parsing failed with error: {}", e);
            }
        }
    }

    #[test]
    fn it_parses_a_mix_of_all_4_bencode_types() -> Result<(), BencodeError> {
        let bencode_bytes = vec![
            // First string
            b'3', b':', b'o', b'n', b'e', // "one"
            // Second integer
            b'i', b'4', b'2', b'e', // 42
            // Third list
            b'l', // Start of list
            b'3', b':', b'f', b'o', b'o', // "foo"
            b'i', b'4', b'2', b'e', // 42
            b'e', // End of list
            // Fourth dictionary
            b'd', // Start of dictionary
            b'3', b':', b'f', b'o', b'o', // Key: "foo"
            b'i', b'4', b'2', b'e', // Value: 42
            b'5', b':', b'h', b'e', b'l', b'l', b'o', // Key: "hello"
            b'l', // Start of list
            b'i', b'4', b'2', b'e', // Integer: 42
            b'e', // End of list
            b'e', // End of dictionary
        ];
        let mut cursor = std::io::Cursor::new(bencode_bytes);
        let mut context = ParserContext::new(&mut cursor);
        let expected = vec![
            BencodeValue::ByteString(b"one".to_vec()),
            BencodeValue::Integer(42),
            BencodeValue::List(vec![
                BencodeValue::ByteString(b"foo".to_vec()),
                BencodeValue::Integer(42),
            ]),
            BencodeValue::Dictionary(vec![
                (b"foo".to_vec(), BencodeValue::Integer(42)),
                (
                    b"hello".to_vec(),
                    BencodeValue::List(vec![BencodeValue::Integer(42)]),
                ),
            ]),
        ];
        match parse_bencode(&mut context) {
            Ok(result) => {
                assert_eq!(result, expected);
                Ok(())
            }
            Err(e) => {
                panic!("Parsing failed with error: {}", e);
            }
        }
    }
}
