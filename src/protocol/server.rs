use std::{collections::HashMap, io::{Read, Write}, net::TcpStream};

use byteorder::{NetworkEndian, ReadBytesExt};
use flate2::{bufread::{ZlibDecoder}, Compression};
use flate2::write::ZlibEncoder;

use super::packets::{read_varint_from_stream, Boolean, ServerboundPacket, VarInt, IDS};

pub struct MinecraftSever {
    tcp_stream: TcpStream,
    pub state: String,
    compression_threshold: i32
}
impl MinecraftSever {
    pub fn connect(address: String) -> MinecraftSever {
        let tcp_stream: TcpStream = TcpStream::connect(address).unwrap();
        Self { tcp_stream, state: "handshaking".to_string(), compression_threshold: -1 }
    }
    pub fn read_and_parse_packet(&mut self) -> Result<(IDS, Vec<u8>), String> {
        let packet_length = read_varint_from_stream(&mut self.tcp_stream).unwrap();
        let mut data_length = 0;
        if self.compression_threshold != -1 {
            data_length = read_varint_from_stream(&mut self.tcp_stream).unwrap();
            println!("{}", data_length);
        }
    
        let mut packet_data = vec![0u8; packet_length as usize];
        self.tcp_stream.read_exact(&mut packet_data)
            .map_err(|e| format!("Failed to read packet data: {}", e))?;

        if self.compression_threshold != -1 && data_length >= self.compression_threshold {
            let mut decompressed_packet_data: Vec<u8> = Vec::new();
            let mut decoder = ZlibDecoder::new(packet_data.as_slice());
            decoder.read_to_end(&mut decompressed_packet_data).unwrap();
            packet_data = decompressed_packet_data;
        }
    
        let (packet_id, offset) = VarInt::parse(&mut &packet_data[..]).unwrap();
    
        let payload = &packet_data[offset..];
    
        let id_enum: IDS;
        if self.state == "login" {
            id_enum = IDS::from_login_clientbound_id(packet_id.0).expect(format!("Unknown packet id: {}", packet_id.0).as_str());
        } else {
            id_enum = IDS::from_play_clientbound_id(packet_id.0).expect(format!("Unknown packet id: {}", packet_id.0).as_str());
        }
    
        Ok((id_enum, payload.to_vec()))
    }
    pub fn send_packet<T: ServerboundPacket>(&mut self, packet: T) {
        let mut packet_bytes = Vec::new();
        VarInt(packet.id().parse()).write(&mut packet_bytes);
        packet_bytes.extend_from_slice(packet.to_bytes().unwrap().as_slice());
        let packet_length = VarInt(packet_bytes.len() as i32);
        if self.compression_threshold != -1 && self.compression_threshold <= packet_bytes.len() as i32 {
            let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
            encoder.write_all(&packet_bytes).unwrap();
            packet_bytes = encoder.finish().unwrap();
        }
        let mut data: Vec<u8> = Vec::new();
        packet_length.write(&mut data);
        if self.compression_threshold != -1 && self.compression_threshold <= packet_bytes.len() as i32 {
            let mut compressed_data: Vec<u8> = Vec::new();
            VarInt((data.len() + packet_bytes.len()) as i32).write(&mut compressed_data);
            packet_length.write(&mut compressed_data);
            data = compressed_data;
        }
        data.extend_from_slice(packet_bytes.as_slice());
        let _ = self.tcp_stream.write(data.as_slice());
        let _ = self.tcp_stream.flush();
    }
    pub fn set_state(&mut self, state: String) {
        self.state = state;
    }
    pub fn set_compression(&mut self, level: i32) {
        self.compression_threshold = level;
    }
}