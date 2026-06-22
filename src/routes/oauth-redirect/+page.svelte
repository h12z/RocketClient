<script lang="ts">
  import type { PageProps } from "./$types";
  import { onMount } from "svelte";
  import { toastManager } from "$lib/toasts.svelte";
  import { goto } from "$app/navigation";

  let { data }: PageProps = $props();

  onMount(() => {
    if (data.error) {
      toastManager.add(data.error, "error", 5000);
      //goto("/");
    }
  });
</script>

<div class="flex min-h-[60vh] flex-col items-center justify-center text-center">
  {#if !data.error}
    <div class="relative mb-8">
      <div class="h-16 w-16 animate-spin rounded-full border-4 border-zinc-800 border-t-amber-400"></div>
      <div class="absolute inset-0 flex items-center justify-center">
        <div class="h-2 w-2 rounded-full bg-amber-400 animate-ping"></div>
      </div>
    </div>
    
    <h2 class="text-2xl font-bold text-zinc-100">Authenticating...</h2>
    <p class="mt-2 text-zinc-400 text-sm">Finishing the connection with Microsoft.</p>
  {:else}
    <div class="rounded-full bg-red-500/10 p-4 text-red-500 mb-6">
      <svg xmlns="http://www.w3.org/2000/svg" width="32" height="32" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><line x1="12" y1="8" x2="12" y2="12"/><line x1="12" y1="16" x2="12.01" y2="16"/></svg>
    </div>
    <h2 class="text-2xl font-bold text-zinc-100">Authentication Failed</h2>
    <p class="mt-2 text-zinc-400 text-sm mb-8">{data.error}</p>
    <a href="/" class="text-amber-400 hover:underline">Return to Home</a>
  {/if}
</div>
