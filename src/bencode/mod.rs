pub mod errors;
pub mod parsing;

pub use errors::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum BencodeValue {
    Integer(i64),
    ByteString(Vec<u8>),
}
