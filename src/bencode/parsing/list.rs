use std::io::Read;

use super::{BencodeError, BencodeErrorKind, BencodeValue, ParserContext};

pub fn parse_list<R: Read>(
    context: &mut ParserContext<R>,
) -> Result<Vec<BencodeValue>, BencodeError> {
    context.expect_and_consume_byte(b'l')?;
    let bencode_seq = super::parse_bencode_to_end(context, Some(b'e'), true)?;
    Ok(bencode_seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{io::Cursor, result};

    #[test]
    fn it_parses_simple_bencode_list() {
        let bencode_bytes: Vec<u8> = vec![
            b'l', // Start of list
            b'5', b':', b'h', b'e', b'l', b'l', b'o', // String: "hello"
            b'i', b'4', b'2', b'e', // Integer: 42
            b'3', b':', b'f', b'o', b'o', // String: "foo"
            b'i', b'-', b'1', b'0', b'e', // Integer: -10
            b'0', b':', // Empty string
            b'i', b'0', b'e', // Integer: 0
            b'e', // End of list
        ];
        let mut cursor = Cursor::new(bencode_bytes);
        let mut context = ParserContext::new(&mut cursor);

        let expected_list = vec![
            BencodeValue::ByteString(b"hello".to_vec()),
            BencodeValue::Integer(42),
            BencodeValue::ByteString(b"foo".to_vec()),
            BencodeValue::Integer(-10),
            BencodeValue::ByteString(b"".to_vec()),
            BencodeValue::Integer(0),
        ];

        match parse_list(&mut context) {
            Ok(result) => {
                assert_eq!(result, expected_list);
            }
            Err(e) => {
                panic!("Parsing failed with error: {}", e);
            }
        }
    }
}
