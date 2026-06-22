import { error, type RequestHandler } from "@sveltejs/kit";
import { CLIENT_ID, CLIENT_SECRET, REDIRECT_URI } from '$env/static/private';

export const GET: RequestHandler = async ({ cookies }) => {
  if(cookies.get("msAccessToken")) {
    return new Response();
  }

  const msRefreshToken = cookies.get("msRefreshToken");
  if(!msRefreshToken) {
    error(400, "missing access token");
  }
  
  const fetchUrl = "https://login.live.com/oauth20_token.srf";
  const searchParams = new URLSearchParams();
  searchParams.set("client_id", CLIENT_ID);
  searchParams.set("refresh_token", msRefreshToken);
  searchParams.set("client_secret", CLIENT_SECRET);
  searchParams.set("grant_type", "refresh_token");
  searchParams.set("redirect_uri", REDIRECT_URI);
  searchParams.set("scope", "XboxLive.signin offline_access");

  const response = await fetch(fetchUrl, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded"
    },
    body: searchParams
  });
  const json = await response.json();

  if(!response.ok) {
    error(400, json.error_message);
  }

  const accessToken = json.access_token;
  const refreshToken = json.refresh_token;

  cookies.set("msAccessToken", accessToken, {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    maxAge: json.expires_in
  });

  cookies.set("msRefreshToken", refreshToken, {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    maxAge: 30 * 24 * 60 * 60
  });

  return new Response();
}