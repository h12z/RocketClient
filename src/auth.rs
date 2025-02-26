use std::string::ToString;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::fs;

const CLIENT_ID: &str = "9c578555-8b03-4448-9ba6-d033496a4212";
pub const CLIENT_SECRET: String = "".to_string();
const REDIRECT_URI: &str = "http://localhost:8080";
const TOKEN_FILE: &str = "token.json";

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

async fn get_microsoft_token(client_id: &str, client_secret: &str, redirect_uri: &str, auth_code: &str) -> Result<AuthResponse, reqwest::Error> {
    let client = Client::new();

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("redirect_uri", redirect_uri),
        ("code", auth_code),
        ("grant_type", "authorization_code"),
    ];

    let response = client.post("https://login.live.com/oauth20_token.srf")
        .form(&params)
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    Ok(response)
}

async fn refresh_microsoft_token(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<AuthResponse, reqwest::Error> {
    let client = Client::new();

    let params = [
        ("client_id", client_id),
        ("client_secret", client_secret),
        ("refresh_token", refresh_token),
        ("grant_type", "refresh_token"),
    ];

    let response = client.post("https://login.live.com/oauth20_token.srf")
        .form(&params)
        .send()
        .await?
        .json::<AuthResponse>()
        .await?;

    Ok(response)
}

fn save_refresh_token(refresh_token: &str) {
    let _ = fs::write(TOKEN_FILE, refresh_token);
}

fn load_refresh_token() -> Option<String> {
    fs::read_to_string(TOKEN_FILE).ok()
}

fn open_browser() {
    let auth_url = format!(
        "https://login.live.com/oauth20_authorize.srf?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access",
        CLIENT_ID, REDIRECT_URI
    );
    open::that(auth_url).expect("Failed to open browser");
}

#[derive(Serialize)]
struct XboxAuthRequest {
    properties: XboxProperties,
    relying_party: String,
    token_type: String,
}

#[derive(Serialize)]
struct XboxProperties {
    auth_method: String,
    site_name: String,
    rps_ticket: String,
}

#[derive(Deserialize)]
struct XboxAuthResponse {
    token: String,
}

async fn authenticate_xbox(access_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let auth_request = XboxAuthRequest {
        properties: XboxProperties {
            auth_method: "RPS".to_string(),
            site_name: "user.auth.xboxlive.com".to_string(),
            rps_ticket: format!("d={}", access_token),
        },
        relying_party: "http://auth.xboxlive.com".to_string(),
        token_type: "JWT".to_string(),
    };

    let response = client.post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&auth_request)
        .send()
        .await?
        .json::<XboxAuthResponse>()
        .await?;

    Ok(response.token)
}

#[derive(Serialize)]
struct MinecraftAuthRequest {
    identity_token: String,
}

#[derive(Deserialize)]
struct MinecraftAuthResponse {
    access_token: String,
}

async fn authenticate_minecraft(xbox_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let auth_request = MinecraftAuthRequest {
        identity_token: format!("XBL3.0 x={};{}", "userhash", xbox_token),
    };

    let response = client.post("https://api.minecraftservices.com/authentication/login_with_xbox")
        .json(&auth_request)
        .send()
        .await?
        .json::<MinecraftAuthResponse>()
        .await?;

    Ok(response.access_token)
}