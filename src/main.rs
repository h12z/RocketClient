use std::collections::HashMap;

use protocol::{packets::{Boolean, ChatMessageClientbound, ClientboundPacket, Handshake, KeepAliveClientbound, KeepAliveServerbound, LoginStart, LoginSuccess, SetCompressionLogin, SetCompressionPlay, UnsignedShort, VarInt, VarString, IDS}, server::MinecraftSever};
use serde::{Deserialize, Serialize};

mod protocol;

fn main() {
    let has_skylight = false;
    let server_address = "192.168.178.128:25565".to_string();
    let mut minecraft_server = MinecraftSever::connect(server_address);
    minecraft_server.send_packet::<Handshake>(Handshake::new(VarInt(47), VarString("192.168.178.128".to_string()), UnsignedShort(25565), VarInt(2)));
    minecraft_server.set_state("login".to_string());
    minecraft_server.send_packet::<LoginStart>(LoginStart::new(VarString("h12z".to_string())));
    println!("Logged in as h12z");
    loop {
        let (id, data) = minecraft_server.read_and_parse_packet().unwrap();
        if minecraft_server.state == "login" {
            if id.parse() == IDS::SetCompressionLogin.parse() {
                let set_compression_packet = SetCompressionLogin::parse(&data).unwrap();
                minecraft_server.set_compression(set_compression_packet.threshold.0);
            }
            if id.parse() == IDS::LoginSuccess.parse() {
                minecraft_server.set_state("play".to_string());
            }
        }
        if minecraft_server.state == "play" {
            if id.parse() == IDS::KeepAliveClientbound.parse() {
                let keep_alive = KeepAliveClientbound::parse(&data).unwrap();
                minecraft_server.send_packet(KeepAliveServerbound::new(keep_alive.keep_alive_id));
            }
            if id.parse() == IDS::ChatMessageClientbound.parse() {
                let chat_message = ChatMessageClientbound::parse(&data).unwrap();
                
            }
        }
    }
}