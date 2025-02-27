use dotenv::dotenv;
use crate::auth::get_profile;
use crate::minecraft_server::{write_unsigned_short, write_var_int, MinecraftServer, Packet};

mod minecraft_server;
mod auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    dotenv().ok();

    auth::open_browser();
    let auth_token = auth::start_server().await.to_string();
    let xbox_token = auth::authenticate_xbox(auth_token.as_str()).await?.Token;
    let xsts_token = auth::authenticate_xsts(xbox_token.as_str()).await?;
    let minecraft_token = auth::authenticate_minecraft(xsts_token.as_str()).await?;
    let profile = get_profile(minecraft_token.as_str()).await?;

    println!("username: {}, uuid: {}", profile.name, profile.id);

    Ok(())

    /*let mut minecraft_server = MinecraftServer::new("192.168.178.135:25565".parse().unwrap());

    let mut handshake_request_buffer = Vec::new();
    write_var_int(&mut handshake_request_buffer, 47);
    let server_address = "192.168.178.135";
    write_var_int(&mut handshake_request_buffer, server_address.len() as i32);
    handshake_request_buffer.extend_from_slice(server_address.as_bytes());
    write_unsigned_short(&mut handshake_request_buffer, 25565);
    write_var_int(&mut handshake_request_buffer, 2);
    minecraft_server.send_packet(&Packet { id: 0x00, data: handshake_request_buffer });

    let mut login_start_buffer = Vec::new();*/


}
