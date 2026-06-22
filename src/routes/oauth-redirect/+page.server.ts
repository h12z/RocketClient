import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import { CLIENT_ID, CLIENT_SECRET, REDIRECT_URI } from '$env/static/private';

export const load: PageServerLoad = async ({ cookies, url }) => {

  const code = url.searchParams.get("code");
  const state = url.searchParams.get("state");
  const error = url.searchParams.get("error");
  const errorDescription = url.searchParams.get("error_description");

  const savedState = cookies.get("state");
  cookies.delete("state", { path: "/" });

  if (error) {
    return {
      status: 400,
      error: errorDescription || error,
    }
  }

  if (!state || state !== savedState) {
    return {
      status: 400,
      error: "Invalid state parameter. Possible CSRF attack.",
    }
  }

  if(!code) {
    return {
      status: 400,
      error: "Missing authorization code.",
    }
  }

  const verifier = cookies.get("pkce_verifier");

  const fetchUrl = "https://login.live.com/oauth20_token.srf";
  const searchParams = new URLSearchParams();
  searchParams.set("client_id", CLIENT_ID);
  searchParams.set("code", code);
  searchParams.set("client_secret", CLIENT_SECRET);
  searchParams.set("grant_type", "authorization_code");
  searchParams.set("redirect_uri", REDIRECT_URI);
  searchParams.set("scope", "XboxLive.signin offline_access");

  if (verifier) {
    searchParams.set("code_verifier", verifier);
  }

  const response = await fetch(fetchUrl, {
    method: "POST",
    headers: {
      "Content-Type": "application/x-www-form-urlencoded"
    },
    body: searchParams
  });

  const json = await response.json();

  if(!response.ok) {
    return {
      status: 500,
      error: json.error_description || "Failed to exchange code for tokens.",
    }
  }

  cookies.delete("pkce_verifier", { path: "/" });

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

  throw redirect(303, "/play")

}
