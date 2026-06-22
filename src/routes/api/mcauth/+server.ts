import { error, type RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ cookies }) => {
  if(cookies.get("accessToken")) {
    return new Response();
  }

  const msAccessToken = cookies.get("msAccessToken");
  if(!msAccessToken) {
    error(400, "missing access token");
  }

  const xblResponse = await fetch("https://user.auth.xboxlive.com/user/authenticate", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json",
    },
    body: JSON.stringify({
      "Properties": {
        "AuthMethod": "RPS",
        "SiteName": "user.auth.xboxlive.com",
        "RpsTicket": "d=" + msAccessToken,
      },
      "RelyingParty": "http://auth.xboxlive.com",
      "TokenType": "JWT"
    }),
  });
  const xblJson = await xblResponse.json();

  const xblToken = xblJson.Token;
  const uhs = xblJson.DisplayClaims.xui[0].uhs;

  const xstsResponse = await fetch("https://xsts.auth.xboxlive.com/xsts/authorize", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json",
    },
    body: JSON.stringify({
      "Properties": {
        "SandboxId": "RETAIL",
        "UserTokens": [
          xblToken
        ]
      },
      "RelyingParty": "rp://api.minecraftservices.com/",
      "TokenType": "JWT"
    }),
  });
  const xstsJson = await xstsResponse.json();

  const xstsToken = xstsJson.Token;
  
  const minecraftResponse = await fetch("https://api.minecraftservices.com/authentication/login_with_xbox", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Accept": "application/json",
    },
    body: JSON.stringify({
      "identityToken": "XBL3.0 x=" + uhs + ";" + xstsToken,
    }),
  });
  const minecraftJson = await minecraftResponse.json();

  cookies.set("accessToken", minecraftJson.access_token, {
    path: "/",
    httpOnly: true,
    secure: true,
    sameSite: "lax",
    maxAge: minecraftJson.expires_in
  });

  return new Response();
}