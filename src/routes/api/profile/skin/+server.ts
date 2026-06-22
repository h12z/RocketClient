import { error, json, type RequestHandler } from "@sveltejs/kit";

export const POST: RequestHandler = async ({ cookies, request }) => {
  const accessToken = cookies.get("accessToken");
  if (!accessToken) {
    error(401, "Unauthorized");
  }

  const formData = await request.formData();
  const file = formData.get("file");
  const variant = formData.get("variant") || "classic";

  if (!file || !(file instanceof File)) {
    error(400, "Missing skin file");
  }

  const mojangFormData = new FormData();
  mojangFormData.append("variant", variant);
  mojangFormData.append("file", file);

  const response = await fetch("https://api.minecraftservices.com/minecraft/profile/skins", {
    method: "POST",
    headers: {
      "Authorization": `Bearer ${accessToken}`,
    },
    body: mojangFormData,
  });

  if (!response.ok) {
    const errorData = await response.json();
    error(response.status, errorData.errorMessage || "Failed to upload skin");
  }

  return json({ success: true });
};

export const DELETE: RequestHandler = async ({ cookies }) => {
  const accessToken = cookies.get("accessToken");
  if (!accessToken) {
    error(401, "Unauthorized");
  }

  const response = await fetch("https://api.minecraftservices.com/minecraft/profile/skins/active", {
    method: "DELETE",
    headers: {
      "Authorization": `Bearer ${accessToken}`,
    },
  });

  if (!response.ok) {
    error(response.status, "Failed to reset skin");
  }

  return json({ success: true });
};
