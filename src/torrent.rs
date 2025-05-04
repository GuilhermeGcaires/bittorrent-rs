use serde::{Deserialize, Serialize};
use serde_bytes::ByteBuf;
use sha1::{Digest, Sha1};

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Info {
    pub length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
    pub pieces: ByteBuf,
}

impl Torrent {
    pub fn info_hash(&self) -> [u8; 20] {
        let info_encoded =
            serde_bencode::to_bytes(&self.info).expect("info section encoding error");
        let mut hasher = Sha1::new();
        hasher.update(&info_encoded);
        hasher.finalize().try_into().expect("Error on hasher array")
    }
}
