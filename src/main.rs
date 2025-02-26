use dotenv::dotenv;
use crate::minecraft_server::{write_unsigned_short, write_var_int, MinecraftServer, Packet};

mod minecraft_server;
mod auth;

fn main() {

    dotenv().ok();
    auth::CLIENT_SECRET = dotenv::var("CLIENT_SECRET").expect("CLIENT_SECRET must be set");

    let mut minecraft_server = MinecraftServer::new("192.168.178.135:25565".parse().unwrap());

    let mut handshake_request_buffer = Vec::new();
    write_var_int(&mut handshake_request_buffer, 47);
    let server_address = "192.168.178.135";
    write_var_int(&mut handshake_request_buffer, server_address.len() as i32);
    handshake_request_buffer.extend_from_slice(server_address.as_bytes());
    write_unsigned_short(&mut handshake_request_buffer, 25565);
    write_var_int(&mut handshake_request_buffer, 2);
    minecraft_server.send_packet(&Packet { id: 0x00, data: handshake_request_buffer });


}
