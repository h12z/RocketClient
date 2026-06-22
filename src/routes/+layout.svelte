<script lang="ts">
	import './layout.css';
	import favicon from '$lib/assets/RCLogo.png';
	import { toastManager } from '$lib/toasts.svelte';
	import { fade, fly } from 'svelte/transition';

	let { children } = $props();
</script>

<svelte:head>
	<title>RocketClient</title>
	<link rel="icon" href={favicon} />
</svelte:head>

<div class="min-h-screen bg-zinc-950 text-zinc-100 selection:bg-amber-500/30">
	<nav class="border-b border-zinc-800 bg-zinc-900/50 backdrop-blur-md">
		<div class="mx-auto flex max-w-7xl items-center justify-between px-4 py-4 sm:px-6 lg:px-8">
			<a href="/" class="flex items-center gap-2">
				<img src={favicon} alt="Logo" class="h-8 w-8 rounded-md" />
				<span class="text-xl font-bold tracking-tight">
					Rocket<span class="text-amber-400">Client</span>
				</span>
			</a>
		</div>
	</nav>

	<main class="mx-auto max-w-7xl px-4 py-8 sm:px-6 lg:px-8">
		{@render children()}
	</main>
</div>

<!-- Toast Container -->
<div class="fixed bottom-4 right-4 z-50 flex flex-col gap-2">
	{#each toastManager.toasts as toast (toast.id)}
		<div
			in:fly={{ y: 20, duration: 300 }}
			out:fade={{ duration: 200 }}
			class="flex items-center gap-3 rounded-lg border border-zinc-800 bg-zinc-900 px-4 py-3 shadow-xl"
		>
			<div
				class="h-2 w-2 rounded-full"
				class:bg-amber-400={toast.type === 'success'}
				class:bg-red-500={toast.type === 'error'}
				class:bg-blue-400={toast.type === 'info'}
			></div>
			<p class="text-sm font-medium">{toast.message}</p>
			<button
				onclick={() => toastManager.remove(toast.id)}
				class="ml-2 text-zinc-500 hover:text-zinc-300"
			>
				&times;
			</button>
		</div>
	{/each}
</div>
