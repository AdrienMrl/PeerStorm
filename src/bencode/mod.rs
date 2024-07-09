pub mod errors;
pub mod parsing;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BencodeValue {
    Integer(i64),
    ByteString(Vec<u8>),
    List(Vec<BencodeValue>),
    Dictionary(Vec<(Vec<u8>, BencodeValue)>),
}
