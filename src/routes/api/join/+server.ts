import { error, json, type RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ cookies, request }) => {
  const accessToken = cookies.get("accessToken");
  if (!accessToken) {
    error(401, "Unauthorized");
  }

  const { selectedProfile, serverId } = await request.json();

  if (!selectedProfile || !serverId) {
    error(400, "Missing selectedProfile or serverId");
  }

  // Mojang Session Server Join Endpoint
  // This is used by the client to prove it has a valid session to the Minecraft server
  const response = await fetch("https://sessionserver.mojang.com/session/minecraft/join", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify({
      accessToken,
      selectedProfile,
      serverId,
    }),
  });

  if (!response.ok) {
    // Mojang often returns empty body on failure with a status code, or a JSON error
    const errorText = await response.text();
    try {
        const errorJson = JSON.parse(errorText);
        error(response.status, errorJson.errorMessage || "Failed to join server");
    } catch {
        error(response.status, errorText || "Failed to join server");
    }
  }

  // 204 No Content is success for this endpoint
  return json({ success: true });
};
