use datastructures::torrent_file::TorrentFile;
use serde_bencode::de;

mod datastructures;


fn main() {
    let torrent_data = std::fs::read("./assets/ubuntu_test.torrent").unwrap();
    let torrent: TorrentFile = de::from_bytes(&torrent_data).unwrap();

    println!("{}", torrent.info.name);
}
