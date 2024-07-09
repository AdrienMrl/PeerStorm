use std::io::Read;

use super::{BencodeError, BencodeErrorKind, BencodeValue, ParserContext};

pub fn parse_dictionary<R: Read>(
    context: &mut ParserContext<R>,
) -> Result<Vec<(Vec<u8>, BencodeValue)>, BencodeError> {
    context.expect_and_consume_byte(b'd')?;
    let mut bencode_kv_pairs: Vec<(Vec<u8>, BencodeValue)> = Vec::new();
    loop {
        match context.try_peek_byte() {
            Some(b'e') => {
                context.read_byte()?;
                break;
            }
            Some(_) => {
                let key = super::byte_string::parse_string(context)?;
                let value = super::parse_one_bencode(context)?;
                bencode_kv_pairs.push((key, value));
            }
            None => {
                return Err(BencodeError::new(
                    BencodeErrorKind::UnexpectedEnd,
                    context.position,
                ));
            }
        }
    }
    Ok(bencode_kv_pairs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_parses_simple_bencode_list() {
        let bencode_bytes: Vec<u8> = vec![
            b'd', // Start of dictionary
            b'3', b':', b'f', b'o', b'o', // Key: "foo"
            b'i', b'4', b'2', b'e', // Value: 42
            b'5', b':', b'h', b'e', b'l', b'l', b'o', // Key: "hello"
            b'l', // Start of list
            b'i', b'4', b'2', b'e', // Integer: 42
            b'e', // End of list
            b'e', // End of dictionary
        ];
        let mut cursor = Cursor::new(bencode_bytes);
        let mut context = ParserContext::new(&mut cursor);

        let expected_dictionary = vec![
            (b"foo".to_vec(), BencodeValue::Integer(42)),
            (
                b"hello".to_vec(),
                BencodeValue::List(vec![BencodeValue::Integer(42)]),
            ),
        ];

        match parse_dictionary(&mut context) {
            Ok(result) => {
                assert_eq!(result, expected_dictionary);
            }
            Err(e) => {
                panic!("Parsing failed with error: {}", e);
            }
        }
    }
}
