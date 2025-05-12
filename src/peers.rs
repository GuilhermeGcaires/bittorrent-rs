use std::{convert::TryFrom, fmt, net::Ipv4Addr};

#[derive(Debug)]
pub struct Peer {
    pub ip: Ipv4Addr,
    pub port: u16,
}

#[derive(Debug)]
pub struct Peers(pub Vec<Peer>);

#[derive(Debug, thiserror::Error)]
pub enum PeerParseError {
    #[error("Invalid peer data length (must be multiples of 6 bytes)")]
    InvalidLength,
}

impl TryFrom<&[u8]> for Peers {
    type Error = PeerParseError;

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        let peers = bytes
            .chunks_exact(6)
            .map(|chunk| {
                let ip = Ipv4Addr::new(chunk[0], chunk[1], chunk[2], chunk[3]);
                let port = u16::from_be_bytes([chunk[4], chunk[5]]);
                Peer { ip, port }
            })
            .collect::<Vec<_>>();

        Ok(Peers(peers))
    }
}

impl fmt::Display for Peer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.ip, self.port)
    }
}

pub struct Handshake {
    length: u8,
    magic_string: [u8; 19],
    reserved_bytes: [u8; 8],
    info_hash: [u8; 20],
    pub peer_id: [u8; 20],
}

impl Handshake {
    pub fn new(info_hash: [u8; 20], peer_id: [u8; 20]) -> Self {
        Self {
            length: 19,
            magic_string: *b"BitTorrent protocol",
            reserved_bytes: [0; 8],
            info_hash,
            peer_id,
        }
    }

    pub fn encode(&self) -> [u8; 68] {
        let mut buffer = [0u8; 68];

        buffer[0] = self.length;
        buffer[1..20].copy_from_slice(&self.magic_string);
        buffer[20..28].copy_from_slice(&self.reserved_bytes);
        buffer[28..48].copy_from_slice(&self.info_hash);
        buffer[48..68].copy_from_slice(&self.peer_id);

        buffer
    }
}
