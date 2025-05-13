use std::{net::TcpStream, ptr::null};

use super::version::ProtocolVersion;

pub struct Connection {
    pub stream: TcpStream,
    version: ProtocolVersion,
    compression_threshold: i32,
    shared_key: Vec<u8>,
    public_server_key: Vec<u8>,
}

impl Connection {
    pub async fn connect(addr: &str, version: ProtocolVersion) -> std::io::Result<Connection> {
        let stream = TcpStream::connect(addr)?;
        Ok(Connection {
            stream,
            version,
            compression_threshold: -1,
            shared_key: Vec::new(),
            public_server_key: Vec::new()
        })
    }
    pub fn send_packet
}