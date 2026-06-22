import type { PageServerLoad } from './$types';
import { CLIENT_ID, REDIRECT_URI } from '$env/static/private';

export const load: PageServerLoad = async ({ cookies }) => {
  const refreshToken = cookies.get("msRefreshToken");

  function generateCodeVerifier() {
    const array = new Uint32Array(56);
    crypto.getRandomValues(array);
    return Array.from(array, (dec) => ("0" + dec.toString(16)).substr(-2)).join(
      "",
    );
  }

  function base64UrlEncode(arrayBuffer: ArrayBuffer) {
    const bytes = new Uint8Array(arrayBuffer);
    let binary = "";
    for (let i = 0; i < bytes.byteLength; i++) {
      binary += String.fromCharCode(bytes[i]);
    }

    return btoa(binary)
      .replace(/\+/g, "-")
      .replace(/\//g, "_")
      .replace(/=+$/, "");
  }

  async function generateCodeChallenge(verifier: string) {
    const encoder = new TextEncoder();
    const data = encoder.encode(verifier);

    const hash = await crypto.subtle.digest("SHA-256", data);

    return base64UrlEncode(hash);
  }

  if(refreshToken == undefined) {
    const verifier = generateCodeVerifier();
    const challenge = await generateCodeChallenge(verifier);

    console.log("Code Verifier (Keep this safe for the /token call):");
    console.log(verifier);
    
    console.log("\nCode Challenge (Send this to Microsoft's /authorize call):");
    console.log(challenge);
    
    cookies.set("pkce_verifier", verifier, {
      path: "/",
      maxAge: 3600,
      sameSite: "lax",
      secure: true,
      httpOnly: true,
    });

    const state = crypto.randomUUID();

    cookies.set("state", state, {
      path: "/",
      maxAge: 3600,
      sameSite: "lax",
      secure: true,
      httpOnly: true,
    });

    const url = new URL("https://login.live.com/oauth20_authorize.srf");
    url.searchParams.set("client_id", CLIENT_ID);
    url.searchParams.set("prompt", "select_account");
    url.searchParams.set("response_type", "code");
    url.searchParams.set("scope", "XboxLive.signin offline_access");
    url.searchParams.set("redirect_uri", REDIRECT_URI);
    url.searchParams.set("code_challenge", challenge);
    url.searchParams.set("code_challenge_method", "S256");
    url.searchParams.set("state", state);
    
    return {
      loggedIn: false,
      url: url.toString(),
    }
  }

  return {
    loggedIn: true,
    url: "",
	};
};