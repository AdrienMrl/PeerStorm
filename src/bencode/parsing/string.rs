use std::io::{BufRead, Read};

use crate::bencode::BencodeErrorKind;

use super::{BencodeError, BencodeValue, ParserContext};

pub fn parse_string<R: Read>(context: &mut ParserContext<R>) -> Result<BencodeValue, BencodeError> {
    let mut size_part = context.read_until(b':')?;
    size_part.pop();
    let size = context.parse_usize(&size_part)?;
    let bytes_string = context.read_exact(size as usize)?;
    Ok(BencodeValue::ByteString(bytes_string))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn it_parses_simple_bencode_string() {
        let bencode_bytes: Vec<u8> = vec![
            b'1', b'3', b':', b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd',
            b' ', b'!',
        ];
        let mut cursor = Cursor::new(bencode_bytes);
        let mut context = ParserContext::new(&mut cursor);

        match parse_string(&mut context) {
            Ok(result) => {
                assert_eq!(result, BencodeValue::ByteString(b"hello world !".to_vec()));
            }
            Err(e) => {
                panic!("Parsing failed with error: {}", e);
            }
        }
    }
}
