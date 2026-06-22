import { redirect } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ cookies }) => {
	cookies.delete('msAccessToken', { path: '/' });
	cookies.delete('msRefreshToken', { path: '/' });
	cookies.delete('accessToken', { path: '/' });
	throw redirect(302, '/');
};
