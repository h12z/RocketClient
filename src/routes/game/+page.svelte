<script lang="ts">
  import type { PageProps } from "./$types";
  import { onMount } from "svelte";
  import { fade } from "svelte/transition";
  import { GameLoader } from "$lib/game/loader";

  let { data }: PageProps = $props();
  const { profile, serverAddress } = data;

  let isConnecting = $state(true);
  let canvas: HTMLCanvasElement;
  let engine: any = null;

  onMount(async () => {
    const gl = canvas.getContext("webgl2");
    if (!gl) {
      console.error("WebGL 2 not supported");
      return;
    }

    const loader = new GameLoader(gl);

    try {
      engine = await loader.load("/game.wasm");

      const websocket = new WebSocket(data.serverAddress);
      
      websocket.onmessage = (event) => {
        const data: string = event.data;

        const encoder = new TextEncoder();
        const encodedString = encoder.encode(data);

        const pointer = engine.alloc(encodedString.length);
        
        const wasmMemoryBuffer = new Uint8Array(engine.memory.buffer);

        engine.on_message()
      }
      

      engine.init(window.innerWidth, window.innerHeight);
      isConnecting = false;

      let startTime = performance.now();
      function loop(now: number) {
        if (!engine) return;
        const elapsed = (now - startTime) / 1000;
        engine.update(elapsed);
        requestAnimationFrame(loop);
      }
      requestAnimationFrame(loop);

      const handleResize = () => {
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        engine.resize(canvas.width, canvas.height);
      };
      window.addEventListener("resize", handleResize);
      handleResize();

      return () => {
        window.removeEventListener("resize", handleResize);
        engine = null;
      };
    } catch (e) {
      console.error("Failed to load game engine", e);
    }
  });
</script>

<div
  class="fixed inset-0 z-50 flex flex-col items-center justify-center bg-zinc-950 text-white selection:bg-amber-500/30"
>
  <canvas
    bind:this={canvas}
    class="absolute inset-0 h-full w-full bg-black transition-opacity duration-1000"
    class:opacity-0={isConnecting}
    class:opacity-100={!isConnecting}
  ></canvas>

  {#if isConnecting}
    <div
      in:fade={{ duration: 200 }}
      out:fade={{ duration: 500 }}
      class="relative z-10 flex flex-col items-center"
    >
      <div
        class="mb-8 h-12 w-12 rounded-xl bg-amber-500 flex items-center justify-center shadow-[0_0_30px_rgba(245,158,11,0.2)]"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="2.5"
          stroke-linecap="round"
          stroke-linejoin="round"
          class="text-zinc-950 animate-pulse"
          ><path
            d="M4.5 16.5c-1.5 1.26-2 5-2 5s3.74-.5 5-2c.71-.84.7-2.13-.09-2.91a2.18 2.18 0 0 0-2.91-.09z"
          /><path
            d="m12 15-3-3a22 22 0 0 1 2-3.95A12.88 12.88 0 0 1 22 2c0 2.72-.78 7.5-6 11a22.35 22.35 0 0 1-4 2z"
          /><path d="M9 12H4s.55-3.03 2-4c1.62-1.08 5 0 5 0" /><path
            d="M12 15v5s3.03-.55 4-2c1.08-1.62 0-5 0-5"
          /></svg
        >
      </div>
      <h2 class="text-xl font-bold tracking-tight mb-1">Starting Engine</h2>
      <p class="text-zinc-500 text-xs font-mono uppercase tracking-widest">
        WASM // WEBGL2
      </p>
    </div>
  {/if}

  <div class="absolute top-4 left-4 z-10">
    <a
      href="/play"
      class="rounded-lg bg-zinc-900/50 px-4 py-2 text-xs font-bold text-zinc-100 backdrop-blur-md transition-all hover:bg-zinc-800"
    >
      Disconnect
    </a>
  </div>
</div>
