import { redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ cookies, fetch, url }) => {
  const accessToken = cookies.get('accessToken');
  const address = url.searchParams.get('address');

  if (!address) {
    throw redirect(302, '/play');
  }

  if (!accessToken) {
    const refreshRes = await fetch('/api/refreshToken');
    if (refreshRes.ok) {
      await fetch('/api/mcauth');
      const finalToken = cookies.get('accessToken');
      if (finalToken) {
        const profileRes = await fetch('/api/profile');
        if (profileRes.ok) {
          return {
            profile: await profileRes.json(),
            serverAddress: address
          };
        }
      }
    }
    throw redirect(302, '/');
  }

  const profileRes = await fetch('/api/profile');

  if (!profileRes.ok) {
    await fetch('/api/refreshToken');
    await fetch('/api/mcauth');
    const retryProfileRes = await fetch('/api/profile');

    if (retryProfileRes.ok) {
      return {
        profile: await retryProfileRes.json(),
        serverAddress: address
      };
    }
    throw redirect(302, '/');
  }

  return {
    profile: await profileRes.json(),
    serverAddress: address
  };
};
