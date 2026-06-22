<script lang="ts">
  import type { PageProps } from "./$types";
  import { toastManager } from "$lib/toasts.svelte";
  import { invalidateAll } from "$app/navigation";

  let { data }: PageProps = $props();
  const profile = data.profile;
  const activeSkin = profile.skins.find((s: any) => s.state === 'ACTIVE');

  let variant = $state("classic");
  let selectedFile = $state<File | null>(null);
  let isUploading = $state(false);

  async function handleUpload() {
    if (!selectedFile) return;

    isUploading = true;
    const formData = new FormData();
    formData.append("file", selectedFile);
    formData.append("variant", variant);

    try {
      const response = await fetch("/api/profile/skin", {
        method: "POST",
        body: formData,
      });

      if (response.ok) {
        toastManager.add("Skin uploaded successfully!", "success");
        await invalidateAll();
      } else {
        const error = await response.json();
        toastManager.add(error.message || "Failed to upload skin", "error");
      }
    } catch (e) {
      toastManager.add("An unexpected error occurred", "error");
    } finally {
      isUploading = false;
      selectedFile = null;
    }
  }

  async function handleReset() {
    if (!confirm("Are you sure you want to reset your skin to default?")) return;

    try {
      const response = await fetch("/api/profile/skin", {
        method: "DELETE",
      });

      if (response.ok) {
        toastManager.add("Skin reset successfully!", "success");
        await invalidateAll();
      } else {
        toastManager.add("Failed to reset skin", "error");
      }
    } catch (e) {
      toastManager.add("An unexpected error occurred", "error");
    }
  }

  function handleFileChange(e: Event) {
    const target = e.target as HTMLInputElement;
    if (target.files && target.files.length > 0) {
      selectedFile = target.files[0];
    }
  }
</script>

<div class="mx-auto max-w-6xl">
  <div class="mb-8">
    <h1 class="text-3xl font-bold text-zinc-100">Profile Settings</h1>
    <p class="text-zinc-500">Manage your Minecraft appearance and profile details.</p>
  </div>

  <div class="grid grid-cols-1 gap-8 lg:grid-cols-3">
    <!-- Skin Preview & Upload -->
    <div class="lg:col-span-2 space-y-6">
      <div class="rounded-3xl border border-zinc-800 bg-zinc-900 p-8">
        <h3 class="mb-6 text-xl font-bold text-zinc-100">Minecraft Skin</h3>
        
        <div class="flex flex-col gap-8 md:flex-row">
          <!-- Preview -->
          <div class="flex flex-col items-center gap-4">
            <div class="relative h-64 w-64 overflow-hidden rounded-2xl bg-zinc-950 border border-zinc-800 shadow-inner group">
              {#if activeSkin}
                <!-- This is a very basic 2D preview, ideally we'd want a 3D renderer here later -->
                <img 
                  src={activeSkin.url} 
                  alt="Skin Preview" 
                  class="h-full w-auto object-cover scale-[1] translate-y-[20%] [image-rendering:pixelated]" 
                />
              {:else}
                <div class="flex h-full items-center justify-center text-zinc-700">
                  <svg xmlns="http://www.w3.org/2000/svg" width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2"/><circle cx="12" cy="7" r="4"/></svg>
                </div>
              {/if}
            </div>
            <span class="text-xs font-bold uppercase tracking-widest text-zinc-500">Current Skin</span>
          </div>

          <!-- Upload Form -->
          <div class="flex-1 space-y-6">
            <div class="space-y-2">
              <label class="block text-xs font-bold uppercase tracking-widest text-zinc-500">Model Variant</label>
              <div class="flex gap-2">
                <button 
                  onclick={() => variant = 'classic'}
                  class="flex-1 rounded-xl border py-2 text-sm font-bold transition-all {variant === 'classic' ? 'border-amber-400 bg-amber-400/10 text-amber-400' : 'border-zinc-800 bg-zinc-950/50 text-zinc-500 hover:border-zinc-700 cursor-pointer'}"
                >
                  Classic (Steve)
                </button>
                <button 
                  onclick={() => variant = 'slim'}
                  class="flex-1 rounded-xl border py-2 text-sm font-bold transition-all {variant === 'slim' ? 'border-amber-400 bg-amber-400/10 text-amber-400' : 'border-zinc-800 bg-zinc-950/50 text-zinc-500 hover:border-zinc-700 cursor-pointer'}"
                >
                  Slim (Alex)
                </button>
              </div>
            </div>

            <div class="space-y-2">
              <label class="block text-xs font-bold uppercase tracking-widest text-zinc-500">Skin File</label>
              <div class="relative group">
                <input 
                  type="file" 
                  accept="image/png"
                  onchange={handleFileChange}
                  class="absolute inset-0 z-10 h-full w-full cursor-pointer opacity-0"
                />
                <div class="flex items-center gap-3 rounded-xl border border-zinc-800 bg-zinc-950/50 p-4 text-sm text-zinc-400 transition-all group-hover:border-amber-400/30 group-hover:bg-zinc-900">
                  <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4"/><polyline points="17 8 12 3 7 8"/><line x1="12" y1="3" x2="12" y2="15"/></svg>
                  <span>{selectedFile ? selectedFile.name : 'Choose a .png skin file...'}</span>
                </div>
              </div>
            </div>

            <div class="flex gap-3 pt-4">
              <button 
                onclick={handleUpload}
                disabled={!selectedFile || isUploading}
                class="flex-1 rounded-xl bg-amber-500 py-3 font-bold text-zinc-950 transition-all hover:bg-amber-400 active:scale-95 disabled:opacity-50 disabled:hover:bg-amber-500"
              >
                {isUploading ? 'Uploading...' : 'Upload Skin'}
              </button>
              <button 
                onclick={handleReset}
                class="rounded-xl border border-zinc-800 bg-zinc-800/50 px-6 font-bold text-zinc-400 transition-all hover:bg-zinc-800 hover:text-red-400 active:scale-95 cursor-pointer"
              >
                Reset
              </button>
            </div>
          </div>
        </div>
      </div>

      <div class="rounded-3xl border border-zinc-800 bg-zinc-900 p-8">
        <h3 class="mb-4 text-xl font-bold text-zinc-100">Account Information</h3>
        <div class="space-y-4">
          <div class="flex items-center justify-between rounded-xl bg-zinc-950/50 p-4">
            <div>
              <p class="text-[10px] font-bold uppercase tracking-widest text-zinc-500">Username</p>
              <p class="font-mono text-zinc-100">{profile.name}</p>
            </div>
          </div>
          <div class="flex items-center justify-between rounded-xl bg-zinc-950/50 p-4">
            <div>
              <p class="text-[10px] font-bold uppercase tracking-widest text-zinc-500">UUID</p>
              <p class="font-mono text-sm text-zinc-400">{profile.id}</p>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Sidebar / Additional Info -->
    <div class="space-y-6">
        <div class="rounded-3xl border border-zinc-800 bg-amber-500/5 p-6 border-amber-500/20">
            <h4 class="mb-2 font-bold text-amber-400">Skin Guidelines</h4>
            <ul class="space-y-2 text-xs text-zinc-400 leading-relaxed">
                <li>• Format must be a 64x64 or 64x32 .png file.</li>
                <li>• Use "Slim" if your skin has 3-pixel wide arms.</li>
                <li>• Use "Classic" if your skin has 4-pixel wide arms.</li>
                <li>• Skins must comply with Mojang's EULA.</li>
            </ul>
        </div>

        <div class="rounded-3xl border border-zinc-800 bg-zinc-900 p-6">
            <h4 class="mb-4 font-bold text-zinc-100">Support</h4>
            <p class="mb-4 text-sm text-zinc-500">Need help with your account or having trouble uploading?</p>
            <a href="https://help.minecraft.net" target="_blank" class="block w-full rounded-xl border border-zinc-800 bg-zinc-950/50 py-3 text-center text-sm font-bold text-zinc-300 transition-all hover:bg-zinc-800 hover:text-amber-400">
                Official Mojang Help
            </a>
        </div>
    </div>
  </div>
</div>
