<script lang="ts">
  import type { PageProps } from "./$types";

  let { data }: PageProps = $props();
</script>

<div class="flex min-h-[70vh] flex-col items-center justify-center text-center">
  <div class="relative mb-8">
    <div class="absolute -inset-4 rounded-full bg-amber-500/20 blur-2xl"></div>
    <h1 class="relative text-5xl font-extrabold tracking-tight sm:text-7xl">
      Next-Gen <span class="text-amber-400">Minecraft</span> Launcher
    </h1>
  </div>
  
  <p class="mb-12 max-w-2xl text-lg text-zinc-400 sm:text-xl">
    RocketClient is the fastest, most customizable Minecraft client designed for performance and beauty. Join the future of play.
  </p>

  <div class="flex flex-col gap-4 sm:flex-row">
    {#if data.loggedIn}
      <a 
        href="/play" 
        class="group relative flex items-center justify-center gap-2 overflow-hidden rounded-xl bg-amber-500 px-8 py-4 text-lg font-bold text-zinc-950 transition-all hover:scale-105 active:scale-95"
      >
        <span>Launch Game</span>
        <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round" class="transition-transform group-hover:translate-x-1"><path d="m9 18 6-6-6-6"/></svg>
      </a>
    {:else}
      <a 
        href={data.url} 
        class="flex items-center justify-center gap-3 rounded-xl border border-zinc-800 bg-zinc-900 px-8 py-4 text-lg font-semibold transition-all hover:border-amber-400/50 hover:bg-zinc-800 active:scale-95"
      >
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 23 23"><path fill="#f3f3f3" d="M0 0h11v11H0zM12 0h11v11H12zM0 12h11v11H0zM12 12h11v11H12z"/></svg>
        <span>Connect with Microsoft</span>
      </a>
    {/if}
    
    <a 
      href="https://github.com" 
      target="_blank"
      class="flex items-center justify-center gap-2 rounded-xl border border-zinc-800 px-8 py-4 text-lg font-semibold transition-all hover:bg-zinc-900"
    >
      <span>View on GitHub</span>
    </a>
  </div>
</div>
