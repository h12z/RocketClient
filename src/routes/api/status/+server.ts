import { json, error } from '@sveltejs/kit';
import type { RequestHandler } from './$types';

export const GET: RequestHandler = async ({ url }) => {
	const address = url.searchParams.get('address');

	if (!address) {
		error(400, 'Missing server address');
	}

	try {
		// We use mcsrvstat.us as a proxy to get Minecraft server data without needing a full TCP ping implementation in Node/Bun
		const response = await fetch(`https://api.mcsrvstat.us/3/${address}`);
		const data = await response.json();

		if (!data.online) {
			return json({
				online: false,
				name: address,
				ip: address,
				playersOnline: 0,
				maxPlayers: 0,
				motd: 'Server is offline',
				version: 'Unknown',
				icon: ''
			});
		}

		return json({
			online: true,
			name: data.hostname || address,
			ip: address,
			playersOnline: data.players?.online || 0,
			maxPlayers: data.players?.max || 0,
			motd: data.motd?.clean?.join('\n') || '',
			version: data.version || '1.8-1.21',
			icon: data.icon || `https://api.mcsrvstat.us/icon/${address}`
		});
	} catch (e) {
		console.error(`Failed to fetch status for ${address}:`, e);
		return json({
			online: false,
			name: address,
			ip: address,
			playersOnline: 0,
			maxPlayers: 0,
			motd: 'Failed to fetch status',
			version: 'Unknown',
			icon: ''
		});
	}
};
