import { error, type RequestHandler } from "@sveltejs/kit";

export const GET: RequestHandler = async ({ cookies }) => {
  const accessToken = cookies.get("accessToken");
  if(!accessToken) {
    error(400, "missing access token");
  }

  const response = await fetch("https://api.minecraftservices.com/minecraft/profile", {
    method: "GET",
    headers: {
      "Authorization": "Bearer " + accessToken,
    },
  })
  const json = await response.json();

  if(!response.ok) {
    error(response.status, JSON.stringify(json));
  }

  return new Response(JSON.stringify(json));
}