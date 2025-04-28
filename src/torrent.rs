use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Torrent {
    pub announce: String,
    pub info: Info,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Info {
    pub length: usize,
    name: String,
    #[serde(rename = "piece length")]
    piece_length: usize,
}
