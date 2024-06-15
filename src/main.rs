use serde::Deserialize;
use serde_bencode::de;

#[derive(Debug, Deserialize)]
struct TorrentFile {
    info: Info,
    announce: String,
    #[serde(rename = "announce-list")]
    announce_list: Option<Vec<Vec<String>>>,
    #[serde(rename = "creation date")]
    creation_date: Option<i64>,
    comment: Option<String>,
    #[serde(rename = "created by")]
    created_by: Option<String>,
}

#[derive(Debug, Deserialize)]
struct Info {
    name: String,
    #[serde(rename = "piece length")]
    piece_length: i64,
    pieces: Vec<u8>,
    length: Option<i64>,
    files: Option<Vec<File>>,
}

#[derive(Debug, Deserialize)]
struct File {
    length: i64,
    path: Vec<String>,
}

fn main() {
    let torrent_data = std::fs::read("./assets/ubuntu_test.torrent").unwrap();
    let torrent: TorrentFile = de::from_bytes(&torrent_data).unwrap();

    println!("{:#?}", torrent);
}
