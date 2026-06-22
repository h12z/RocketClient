<script lang="ts">
  import type { PageProps } from "./$types";
  import PlayerHead from "$lib/components/PlayerHead.svelte";
  import ServerCard from "$lib/components/ServerCard.svelte";
  import { serverStore, type Server } from "$lib/servers.svelte";
  import { fly, fade, scale } from "svelte/transition";

  let { data }: PageProps = $props();
  const profile = data.profile;
  const activeSkin = profile.skins.find((s: any) => s.state === "ACTIVE");
  const skinUrl = activeSkin ? activeSkin.url : "";

  let isProfileOpen = $state(false);

  // Modal State
  let isModalOpen = $state(false);
  let editingServer = $state<Server | null>(null);
  let formName = $state("");
  let formAddress = $state("");

  function openAddModal() {
    editingServer = null;
    formName = "";
    formAddress = "";
    isModalOpen = true;
  }

  function openEditModal(server: Server) {
    editingServer = server;
    formName = server.name;
    formAddress = server.address;
    isModalOpen = true;
  }

  function handleSubmit() {
    if (editingServer) {
      serverStore.update(editingServer.id, formName, formAddress);
    } else {
      serverStore.add(formName, formAddress);
    }
    isModalOpen = false;
  }

  function handleDelete() {
    if (editingServer) {
      serverStore.remove(editingServer.id);
      isModalOpen = false;
    }
  }

  // Mock Friend List
  const friends = [];
</script>

<div class="relative min-h-[80vh]">

  <!-- Server Grid -->
  <div
    class="grid grid-cols-1 gap-6 md:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4"
  >
    {#each serverStore.servers as server (server.id)}
      <ServerCard {server} onEdit={() => openEditModal(server)} />
    {/each}

    <!-- Add Server Button -->
    <button
      onclick={openAddModal}
      class="flex h-full min-h-40 flex-col items-center justify-center gap-3 rounded-2xl border-2 border-dashed border-zinc-800 transition-all hover:border-amber-400/50 hover:bg-amber-400/5 cursor-pointer"
    >
      <div
        class="flex h-12 w-12 items-center justify-center rounded-full bg-zinc-900 text-zinc-500"
      >
        <svg
          xmlns="http://www.w3.org/2000/svg"
          width="24"
          height="24"
          viewBox="0 0 24 24"
          fill="none"
          stroke="currentColor"
          stroke-width="3"
          stroke-linecap="round"
          stroke-linejoin="round"
          ><line x1="12" y1="5" x2="12" y2="19" /><line
            x1="5"
            y1="12"
            x2="19"
            y2="12"
          /></svg
        >
      </div>
      <span class="text-sm font-bold text-zinc-500 uppercase tracking-widest"
        >Add Server</span
      >
    </button>
  </div>

  <!-- Floating Profile -->
  <div class="fixed bottom-8 right-8 z-40">
    {#if isProfileOpen}
      <div
        in:fly={{ y: 20, duration: 200 }}
        out:fade={{ duration: 150 }}
        class="absolute bottom-full right-0 mb-4 w-64 overflow-hidden rounded-2xl border border-zinc-800 bg-zinc-900 shadow-2xl"
      >
        <div class="border-b border-zinc-800 p-4">
          <div class="flex items-center gap-3">
            <PlayerHead {skinUrl} size="h-12 w-12" />
            <div>
              <h4 class="font-bold text-zinc-100">{profile.name}</h4>
            </div>
          </div>
        </div>
        <div class="p-2">
          <a
            href="/settings"
            class="flex w-full items-center gap-3 rounded-xl px-3 py-2 text-sm text-zinc-400 transition-colors hover:bg-zinc-800 hover:text-zinc-100"
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
                d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"
              /><circle cx="12" cy="12" r="3" /></svg
            >
            <span>Settings</span>
          </a>
          <a
            href="/logout"
            class="flex w-full items-center gap-3 rounded-xl px-3 py-2 text-sm text-red-400 transition-colors hover:bg-red-500/10 hover:text-red-300"
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
              ><path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /><polyline
                points="16 17 21 12 16 7"
              /><line x1="21" y1="12" x2="9" y2="12" /></svg
            >
            <span>Sign Out</span>
          </a>
        </div>
      </div>
    {/if}

    <button
      onclick={() => (isProfileOpen = !isProfileOpen)}
      class="flex h-14 w-14 items-center justify-center rounded-2xl bg-amber-500 text-zinc-950 shadow-xl transition-transform hover:scale-110 active:scale-95 cursor-pointer"
    >
      <PlayerHead {skinUrl} size="h-12 w-12" overlayScale="scale-[1]" />
    </button>
  </div>

  <!-- Add/Edit Server Modal -->
  {#if isModalOpen}
    <div
      class="fixed inset-0 z-50 flex items-center justify-center bg-zinc-950/60 backdrop-blur-sm"
      transition:fade={{ duration: 200 }}
    >
      <div
        class="w-full max-w-md rounded-3xl border border-zinc-800 bg-zinc-900 p-8 shadow-2xl"
        transition:scale={{ duration: 200, start: 0.95 }}
      >
        <h3 class="mb-6 text-2xl font-bold text-zinc-100">
          {editingServer ? "Edit Server" : "Add New Server"}
        </h3>

        <div class="space-y-4">
          <div>
            <label
              for="name"
              class="mb-1.5 block text-xs font-bold uppercase tracking-widest text-zinc-500"
              >Server Name</label
            >
            <input
              id="name"
              type="text"
              bind:value={formName}
              placeholder="My Awesome Server"
              class="w-full rounded-xl border border-zinc-800 bg-zinc-950/50 p-3 text-zinc-100 transition-all focus:border-amber-400/50 focus:outline-none focus:ring-4 focus:ring-amber-400/5"
            />
          </div>
          <div>
            <label
              for="address"
              class="mb-1.5 block text-xs font-bold uppercase tracking-widest text-zinc-500"
              >Server Address</label
            >
            <input
              id="address"
              type="text"
              bind:value={formAddress}
              onsubmit={handleSubmit}
              placeholder="mc.example.com"
              class="w-full rounded-xl border border-zinc-800 bg-zinc-900/50 p-3 text-zinc-100 transition-all focus:border-amber-400/50 focus:outline-none focus:ring-4 focus:ring-amber-400/5"
            />
          </div>
        </div>

        <div class="mt-8 flex gap-3">
          <button
            onclick={handleSubmit}
            class="flex-1 rounded-xl bg-amber-500 py-3 font-bold text-zinc-950 transition-all hover:bg-amber-400 active:scale-95 cursor-pointer"
          >
            {editingServer ? "Save Changes" : "Add Server"}
          </button>
          {#if editingServer}
            <button
              aria-label="Delete"
              onclick={handleDelete}
              class="rounded-xl border border-red-500/20 bg-red-500/10 px-4 font-bold text-red-500 transition-all hover:bg-red-500 hover:text-white active:scale-95 cursor-pointer"
            >
              <svg
                xmlns="http://www.w3.org/2000/svg"
                width="20"
                height="20"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
                stroke-linecap="round"
                stroke-linejoin="round"
                ><polyline points="3 6 5 6 21 6" /><path
                  d="M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                /><line x1="10" y1="11" x2="10" y2="17" /><line
                  x1="14"
                  y1="11"
                  x2="14"
                  y2="17"
                /></svg
              >
            </button>
          {/if}
          <button
            onclick={() => (isModalOpen = false)}
            class="rounded-xl border border-zinc-800 bg-zinc-800/50 px-6 font-bold text-zinc-400 transition-all hover:bg-zinc-800 hover:text-zinc-100 active:scale-95 cursor-pointer"
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>
