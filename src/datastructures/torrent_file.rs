
use serde::Deserialize;
use serde_bytes::ByteBuf;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TorrentFile {
    pub info: Info,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Info {
    pub name: String,
    #[serde(rename = "piece length")]
    pub piece_length: i64,
    pub pieces: ByteBuf,
    pub length: Option<i64>,
    pub files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct File {
    pub length: i64,
}
