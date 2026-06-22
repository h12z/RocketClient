<script lang="ts">
  let { skinUrl, size = "h-32 w-32", overlayScale = "scale-[1.1]" } = $props<{
    skinUrl: string;
    size?: string;
    overlayScale?: string;
  }>();
</script>

<div class="relative {size} overflow-hidden rounded-xl border-4 border-zinc-900 bg-zinc-800 shadow-lg">
  {#if skinUrl}
    <!-- Base Layer (Head) -->
    <div 
      class="absolute inset-0"
      style="
        background-image: url({skinUrl});
        background-size: 800%;
        background-position: 14.285% 14.285%;
        image-rendering: pixelated;
      "
    ></div>
    <!-- Overlay Layer (Hat) -->
    <div 
      class="absolute inset-0 {overlayScale}"
      style="
        background-image: url({skinUrl});
        background-size: 800%;
        background-position: 71.428% 14.285%;
        image-rendering: pixelated;
      "
    ></div>
  {/if}
</div>
