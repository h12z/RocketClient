<script lang="ts">
  import { onMount } from "svelte";
  import { serverStore } from "$lib/servers.svelte";
  import { goto } from "$app/navigation";

  let { server, onEdit } = $props<{
    server: {
      id: string;
      name: string;
      address: string;
    };
    onEdit: () => void;
  }>();

  let status = $state<any>(null);
  let isLoading = $state(true);

  async function fetchStatus() {
    isLoading = true;
    try {
      const res = await fetch(`/api/status?address=${server.address}`);
      status = await res.json();
    } catch (e) {
      console.error(e);
    } finally {
      isLoading = false;
    }
  }

  function handleConnect() {
    goto(`/game?address=${encodeURIComponent(server.address)}`);
  }

  onMount(() => {
    fetchStatus();
  });

  const cleanMotd = (text: string) => text.replace(/§[0-9a-fk-or]/g, "");
</script>

<div

  class="icon group relative overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900 transition-all hover:border-amber-400/50 hover:shadow-2xl"
>
  {#if !isLoading && status?.icon}
    <img
      src={status.icon}
      alt=""
      class="pixelated absolute inset-0 h-full w-full object-cover opacity-20 transition-opacity group-hover:opacity-30"
    />
  {/if}

  <div
    class="relative z-10 p-5 bg-linear-to-t from-zinc-950/80 via-zinc-900/40 to-transparent"
  >
    <div class="mb-4 flex items-start justify-between">
      <div class="flex items-center gap-4">
        <div>
          <h3 class="font-bold text-zinc-100 drop-shadow-md">{server.name}</h3>
          <p class="text-xs text-zinc-400 font-mono drop-shadow-sm">
            {server.address}
          </p>
        </div>
      </div>
      <div class="text-right drop-shadow-md">
        {#if isLoading}
          <div class="h-4 w-12 animate-pulse rounded bg-zinc-800"></div>
        {:else if status?.online}
          <span class="text-xs font-bold text-amber-400"
            >{status.playersOnline.toLocaleString()}</span
          >
          <span
            class="text-[10px] text-zinc-300 uppercase block font-bold tracking-tighter"
            >Online</span
          >
        {:else}
          <span class="text-xs font-bold text-red-400">Offline</span>
        {/if}
      </div>
    </div>

    <div class="min-h-15 text-sm text-zinc-300 line-clamp-3 drop-shadow-sm">
      {#if isLoading}
        <div class="space-y-2">
          <div class="h-3 w-full animate-pulse rounded bg-zinc-800"></div>
          <div class="h-3 w-2/3 animate-pulse rounded bg-zinc-800"></div>
        </div>
      {:else}
        {cleanMotd(status?.motd || "")}
      {/if}
    </div>

    <div class="mt-4 flex items-center justify-between">
      <button
        onclick={handleConnect}
        class="scale-90 rounded-xl bg-amber-500 px-6 py-2.5 text-sm font-bold text-zinc-950 transition-all hover:scale-105 active:scale-95 hover:cursor-pointer"
      >
        Play
      </button>
      <button
        aria-label="Edit"
        onclick={onEdit}
        class="flex h-10 w-10 scale-90 items-center justify-center rounded-xl border border-zinc-700 bg-zinc-900/80 text-zinc-300 transition-all hover:scale-105 active:scale-95 hover:cursor-pointer backdrop-blur-sm"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="18"
          height="18"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2"
          stroke-linecap="round"
          stroke-linejoin="round"
          ><path
            d="M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7"
          /><path
            d="M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z"
          /></svg
        >
      </button>
    </div>
  </div>
</div>

<style>
  /* Global utility class for handling modern/legacy browser pixel perfect scaling */
  :global(.pixelated) {
    image-rendering: -moz-crisp-edges;
    image-rendering: crisp-edges;
    image-rendering: pixelated;
  }
</style>
