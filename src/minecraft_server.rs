use std::io::{Read, Write};
use std::net::{SocketAddr, TcpStream};
use aes::Aes128;
use cfb8::cipher::{AsyncStreamCipher, KeyIvInit};
use flate2::Compression;
use flate2::read::{GzDecoder, ZlibEncoder};
use num_bigint::BigInt;
use num_traits::Signed;
use rand::RngCore;
use reqwest::Client;
use serde::Serialize;
use sha1::{Digest, Sha1};

type AesCfb8Enc = cfb8::Encryptor<Aes128>;
type AesCfb8Dec = cfb8::Decryptor<Aes128>;

pub fn write_var_int(buffer: &mut Vec<u8>, mut value: i32) {
    const SEGMENT_BITS: i32 = 0b0111_1111;
    const CONTINUE_BIT: i32 = 0b1000_0000;
    loop {
        if((value & SEGMENT_BITS) == 0) {
            buffer.push(value as u8);
        }

        buffer.push(((value & SEGMENT_BITS) | CONTINUE_BIT) as u8);

        value = (value as u32 >> 7) as i32;
    }
}

pub fn read_var_int(buffer: &mut Vec<u8>) -> i32 {
    const SEGMENT_BITS: i32 = 0b0111_1111;
    const CONTINUE_BIT: i32 = 0b1000_0000;

    let mut value: i32 = 0;
    let mut position = 0;
    let mut current_byte: u8;
    loop {
        current_byte = buffer.first().unwrap().clone();
        buffer.remove(0);

        value |= ((current_byte & SEGMENT_BITS as u8) << position) as i32;

        if(current_byte & CONTINUE_BIT as u8 == 0) { break; }

        position += 7;

        if(position >= 32) { panic!("VarInt is too big") }
    }
    value
}

pub fn write_unsigned_short(buffer: &mut Vec<u8>, value: u16) {
    buffer.push((value >> 8) as u8);
    buffer.push((value & 0xFF) as u8);
}

pub fn read_unsigned_short(buffer: &[u8]) -> u16 {
    ((buffer[0] as u16) << 8) | (buffer[1] as u16)
}

pub struct Packet {
    pub(crate) id: i32,
    pub(crate) data: Vec<u8>,
}

#[derive(Serialize)]
pub struct JoinServerRequest {
    access_token: String,
    selected_profile: String,
    server_id: String,
}

pub struct MinecraftServer {
    tcp_stream: TcpStream,
    address: SocketAddr,
    pub compress_threshold: i32,
    pub state: String,
    pub encrypted: bool,
    pub encryption_key: Vec<u8>,
}

impl MinecraftServer {
    pub fn new(address: SocketAddr) -> MinecraftServer {
        let tcp_stream = TcpStream::connect(&address).unwrap();
        MinecraftServer {tcp_stream, address, compress_threshold: -1, state: "none".to_string(), encrypted: false, encryption_key: Vec::new() }
    }
    pub fn send_packet(&mut self, packet: &Packet) {
        let mut dataBuffer = Vec::new();
        write_var_int(&mut dataBuffer, packet.id);
        dataBuffer.extend_from_slice(packet.data.as_slice());
        let mut buffer = Vec::new();
        if(self.compress_threshold > 0 && dataBuffer.len() >= self.compress_threshold as usize) {
            let mut preCompressedBuffer = Vec::new();
            write_var_int(&mut preCompressedBuffer, dataBuffer.len() as i32);
            let mut compressor = ZlibEncoder::new(Vec::new(), Compression::default());
            compressor.write_all(&dataBuffer).unwrap();
            preCompressedBuffer.extend_from_slice(compressor.into_inner().as_slice());
            write_var_int(&mut buffer, preCompressedBuffer.len() as i32);
            if(self.encrypted) {
                self.encrypt(preCompressedBuffer.as_mut());
            }
            self.tcp_stream.write(preCompressedBuffer.as_slice()).unwrap();
            return;
        }
        write_var_int(&mut buffer, dataBuffer.len() as i32);
        buffer.extend_from_slice(dataBuffer.as_slice());
        if(self.encrypted) {
            self.encrypt(buffer.as_mut());
        }
        self.tcp_stream.write(buffer.as_slice()).unwrap();
    }
    pub fn receive_packet(&mut self) -> Packet {
        let mut buffer = Vec::new();
        self.tcp_stream.read_to_end(&mut buffer).unwrap();
        if(self.encrypted) {
            self.decrypt(buffer.as_mut());
        }
        let mut packet_length = read_var_int(&mut buffer);
        if(self.compress_threshold > 0) {
            read_var_int(&mut buffer);
            let mut decompressor = GzDecoder::new(buffer.as_slice());
            let mut decompressed_buffer = Vec::new();
            decompressor.read_to_end(&mut decompressed_buffer).unwrap();
            let mut packet_id = read_var_int(&mut decompressed_buffer);
            let mut data = Vec::new();
            data.extend_from_slice(&decompressed_buffer);
            return Packet { id: packet_id, data}
        }
        let mut id = read_var_int(&mut buffer);
        let mut data = Vec::new();
        data.extend_from_slice(&buffer);
        Packet {id, data}
    }

    pub fn generate_server_hash(server_id: String, shared_secret: Vec<u8>, public_key: Vec<u8>) -> String {
        let mut sha1 = Sha1::new();
        sha1.update(server_id.as_bytes());
        sha1.update(shared_secret.as_slice());
        sha1.update(public_key.as_slice());
        let hash_result = sha1.finalize();
        let bigint = BigInt::from_signed_bytes_be(&hash_result);
        let mut hash_hex = bigint.to_str_radix(16);
        if bigint.is_negative() {
            hash_hex.insert(0, '-');
        }
        hash_hex
    }

    pub fn create_shared_secret() -> Vec<u8> {
        let mut data = [0u8; 16];
        rand::rng().fill_bytes(data.as_mut());
        data.to_vec()
    }

    pub fn encrypt(&mut self, data: &mut Vec<u8>) {
        AesCfb8Enc::new_from_slices(self.encryption_key.as_slice(), self.encryption_key.as_slice()).unwrap().encrypt(data.as_slice().as_mut());
    }
    pub fn decrypt(&mut self, data: &mut Vec<u8>) {
        AesCfb8Dec::new_from_slices(self.encryption_key.as_slice(), self.encryption_key.as_slice()).unwrap().decrypt(data.as_slice().as_mut());
    }

    async fn join_minecraft_server(auth_token: &str, uuid: &str, server_hash: &str) -> Result<(), reqwest::Error> {
        let client = Client::new();

        let request_body = JoinServerRequest {
            access_token: auth_token.to_string(),
            selected_profile: uuid.to_string(),
            server_id: server_hash.to_string(),
        };

        client.post("https://sessionserver.mojang.com/session/minecraft/join")
            .json(&request_body)
            .send()
            .await?;

        Ok(())
    }

}