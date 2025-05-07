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
