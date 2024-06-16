use std::collections::HashMap;

#[derive(Debug)]
struct InvalidFormatError {
    message: String,
}

impl std::fmt::Display for InvalidFormatError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Invalid Bencode: {}", self.message)
    }
}

impl std::error::Error for InvalidFormatError {}

#[derive(Debug)]
enum BencodeError {
    InvalidFormat(InvalidFormatError),
}

struct BencodeString {
    value: String,
}

trait ParsingSucess {
    type BecodeObjectType;
    fn get_parsed_object(&self) -> &Self::BecodeObjectType;
    fn get_bencode_size(&self) -> usize;
}

struct StringParsingSuccess {
    parsed_object: BencodeString,
    bencode_size: usize,
}

impl ParsingSucess for StringParsingSuccess {
    type BecodeObjectType = BencodeString;

    fn get_parsed_object(&self) -> &BencodeString {
        return &self.parsed_object;
    }

    fn get_bencode_size(&self) -> usize {
        return self.bencode_size;
    }
}

enum BencodeObject {
    Integer(i64),
    String(BencodeString),
    List(Vec<BencodeObject>),
    Dictionary(HashMap<String, BencodeObject>),
}

fn read_file(file_path: &str) -> Result<Vec<u8>, std::io::Error> {
    std::fs::read(file_path)
}

fn parse_string(bytes: &Vec<u8>) -> Result<StringParsingSuccess, BencodeError>
{
    let delimiter_position = bytes.iter()
        .position(|&byte| byte == b':')
        .ok_or_else(|| {
            BencodeError::InvalidFormat(InvalidFormatError {
                message: "Delimiter ':' not found".to_string(),
            })
        })?;
    let size_bytes = &bytes[..delimiter_position];
    let size_str = String::from_utf8(size_bytes.to_vec()).map_err(|e| {
        BencodeError::InvalidFormat(InvalidFormatError {
            message: format!("Invalid string size: {}", e),
        })
    })?;
    let size = size_str.parse::<usize>().map_err(|e| {
        BencodeError::InvalidFormat(InvalidFormatError {
            message: format!("Invalid string size: {}", e),
        })
    })?;
    let string_start_index = delimiter_position + 1;
    let string_bytes = &bytes[string_start_index..string_start_index + size];
    let string_value = String::from_utf8(string_bytes.to_vec())
        .map_err(|e| {
            BencodeError::InvalidFormat(InvalidFormatError {
                message: format!("Error bencode string to utf8: {}", e),
            })
        })?;
    Ok(StringParsingSuccess { parsed_object: BencodeString { value: string_value }, bencode_size: size_bytes.len() + 1 + size })
}

// fn parse_bencode(bytes: Vec<u8>) -> Result<Vec<Bencode>, InvalidBencodeError>
// {
//     let mut bencode: Vec<Bencode> = Vec::new();
//     for byte in bytes {
//         match byte {
//             b'i' => {},
//         }
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_reads_bytes() {
        let result = read_file("./assets/ubuntu_test.torrent");
        assert!(result.is_ok());
        let bytes = result.unwrap();
        assert!(!bytes.is_empty(), "File should not be empty");
    }

    #[test]
    fn it_parses_simple_bencode_string() {
        let bencode_bytes: Vec<u8> = vec![
            b'1', b'3', b':', b'h', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd',
            b' ', b'!',
        ];
        let result = parse_string(&bencode_bytes);
        match result {
            Ok(success) => {
                let parsed_value = &success.get_parsed_object().value;
                let bencode_size = success.get_bencode_size();
    
                // Print parsed value and bencode size for debug purposes
                println!("Parsed value: {}", parsed_value);
                println!("Bencode size: {}", bencode_size);
    
                // Use assert_eq for better error messages
                assert_eq!(parsed_value, "hello world !", "The parsed value does not match the expected value.");
                assert_eq!(bencode_size, 16, "The bencode size does not match the expected size.");
            },
            Err(e) => panic!("Parsing failed with error: {:?}", e)
        }
    }
}
