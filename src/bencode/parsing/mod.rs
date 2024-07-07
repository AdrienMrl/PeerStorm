use std::io::Read;

use parser_context::ParserContext;

use std::str::FromStr;

use super::{errors::*, BencodeValue};

mod integer;
mod parser_context;
mod string;

pub fn parse_bencode(bytes: &Vec<u8>) -> Result<Vec<BencodeValue>, BencodeError> {
    todo!()
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
        let success = parse_bencode(&bencode_bytes)?;
        assert_eq!(success.len(), 10);
        Ok(())
    }
}
