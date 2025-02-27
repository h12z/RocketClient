use std::string::ToString;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use reqwest::{Client, Response};
use serde::{Deserialize, Serialize};
use tokio::fs;
use warp::{Error, Filter};

const CLIENT_ID: &str = "9c578555-8b03-4448-9ba6-d033496a4212";
const REDIRECT_URI: &str = "http://localhost:8080";
const TOKEN_FILE: &str = "token.json";

pub async fn start_server() -> Result<String, Error>   {

    let token_storage = Arc::new(Mutex::new(None));
    let token_storage_clone = Arc::clone(&token_storage);

    let server = warp::path::end()
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(move |params: std::collections::HashMap<String, String>| {
            if let Some(code) = params.get("code") {
                let code = code.clone();
                let token_storage = Arc::clone(&token_storage_clone);
                tokio::spawn(async move {
                    if let Ok(auth_response) = get_microsoft_token(CLIENT_ID, dotenv::var("CLIENT_SECRET").unwrap().as_str(), REDIRECT_URI, &*code).await {
                        *token_storage.lock().unwrap() = Some(auth_response.refresh_token.clone());
                        save_refresh_token(&auth_response.refresh_token);
                        println!("Authenticated! Refresh token saved.");
                    }
                });
                "Authentication successful! You can close this tab."
            } else {
                "Error: No auth code received."
            }
        });

    let (_addr, server) = warp::serve(server).bind_with_graceful_shutdown(([127, 0, 0, 1], 8080), async {
        tokio::time::sleep(Duration::from_secs(60)).await;
    });

    Ok(refresh_microsoft_token(CLIENT_ID, dotenv::var("CLIENT_SECRET").unwrap().as_str(), load_refresh_token().await.unwrap().as_str()).await.unwrap())

}

pub fn open_browser() {
    let auth_url = format!(
        "https://login.live.com/oauth20_authorize.srf?client_id={}&response_type=code&redirect_uri={}&scope=XboxLive.signin%20offline_access",
        CLIENT_ID, REDIRECT_URI
    );
    open::that(auth_url).expect("Failed to open browser");
}

#[derive(Deserialize)]
struct AuthResponse {
    access_token: String,
    refresh_token: String,
    expires_in: u64,
}

pub async fn get_microsoft_token(client_id: &str, client_secret: &str, redirect_uri: &str, auth_code: &str) -> Result<AuthResponse, reqwest::Error> {
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

pub async fn refresh_microsoft_token(client_id: &str, client_secret: &str, refresh_token: &str) -> Result<String, reqwest::Error> {
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
        .await?
        .access_token;

    Ok(response)
}

pub fn save_refresh_token(refresh_token: &str) {
    let _ = fs::write(TOKEN_FILE, refresh_token);
}

pub async fn load_refresh_token() -> Option<String> {
    fs::read_to_string(TOKEN_FILE).await.ok()
}

#[derive(Serialize)]
struct XboxAuthRequest {
    Properties: XboxProperties,
    RelyingParty: String,
    TokenType: String,
}

#[derive(Serialize)]
struct XboxProperties {
    AuthMethod: String,
    SiteName: String,
    RpsTicket: String,
}

#[derive(Deserialize)]
pub struct XboxAuthResponse {
    pub Token: String,
}

pub async fn authenticate_xbox(access_token: &str) -> Result<XboxAuthResponse, reqwest::Error> {
    let client = Client::new();

    let auth_request = XboxAuthRequest {
        Properties: XboxProperties {
            AuthMethod: "RPS".to_string(),
            SiteName: "user.auth.xboxlive.com".to_string(),
            RpsTicket: format!("d={}", access_token),
        },
        RelyingParty: "http://auth.xboxlive.com".to_string(),
        TokenType: "JWT".to_string(),
    };

    client.post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&auth_request)
        .send()
        .await?
        .json::<XboxAuthResponse>()
        .await
}

#[derive(Serialize)]
struct XSTSRequest {
    Properties: Properties,
    RelyingParty: String,
    TokenType: String,
}

#[derive(Serialize)]
struct Properties {
    SandboxId: String,
    UserTokens: Vec<String>,
}

#[derive(Deserialize)]
struct XSTSResponse {
    Token: String,
}

pub async fn authenticate_xsts(xbox_token: &str) -> Result<String, reqwest::Error> {
    let client = Client::new();

    let auth_request = XSTSRequest {
        Properties: Properties {
            SandboxId: "RETAIL".to_string(),
            UserTokens: Vec::from([xbox_token.to_string()]),
        },
        RelyingParty: "rp://api.minecraftservices.com/".to_string(),
        TokenType: "JWT".to_string(),
    };

    let response = client.post("https://user.auth.xboxlive.com/user/authenticate")
        .json(&auth_request)
        .send()
        .await?
        .json::<XSTSResponse>()
        .await?;

    Ok(response.Token)
}

#[derive(Serialize)]
struct MinecraftAuthRequest {
    identity_token: String,
}

#[derive(Deserialize)]
struct MinecraftAuthResponse {
    access_token: String,
}

pub async fn authenticate_minecraft(xbox_token: &str) -> Result<String, reqwest::Error> {
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

#[derive(Deserialize)]
pub struct MinecraftProfileResponse {
    pub id: String,
    pub name: String,
}

pub async fn get_profile(auth_token: &str) -> Result<MinecraftProfileResponse, reqwest::Error> {
    let client = Client::new();

    let response = client.get("https://api.minecraftservices.com/minecraft/profile")
        .header("Authorization", format!("Bearer {}", auth_token))
        .send()
        .await?
        .json::<MinecraftProfileResponse>()
        .await?;

    Ok(response)
}