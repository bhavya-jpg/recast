<script lang="ts">
  import { Button } from "$components/ui/button";
  import {
    Camera,
    CameraOff,
    Mic,
    MicOff,
    Pencil,
    Plus,
    Trash2,
    Volume2,
    VolumeOff,
  } from "@lucide/svelte";
  import { onMount } from "svelte";

  interface RecordingProfile {
    id: string;
    name: string;
    systemAudio: boolean;
    microphone: boolean;
    camera: boolean;
    isDefault: boolean;
  }

  const STORAGE_KEY = "recast-recording-profiles";

  let profiles = $state<RecordingProfile[]>([]);
  let editingId = $state<string | null>(null);
  let editingName = $state("");

  onMount(() => {
    const stored = localStorage.getItem(STORAGE_KEY);
    if (stored) {
      try {
        profiles = JSON.parse(stored);
      } catch {
        profiles = [];
      }
    }
    if (profiles.length === 0) {
      profiles = [
        {
          id: crypto.randomUUID(),
          name: "Screen Only",
          systemAudio: true,
          microphone: false,
          camera: false,
          isDefault: true,
        },
        {
          id: crypto.randomUUID(),
          name: "Presentation",
          systemAudio: true,
          microphone: true,
          camera: true,
          isDefault: false,
        },
        {
          id: crypto.randomUUID(),
          name: "Tutorial",
          systemAudio: true,
          microphone: true,
          camera: false,
          isDefault: false,
        },
      ];
      save();
    }
  });

  function save() {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(profiles));
  }

  function addProfile() {
    const profile: RecordingProfile = {
      id: crypto.randomUUID(),
      name: `Profile ${profiles.length + 1}`,
      systemAudio: true,
      microphone: false,
      camera: false,
      isDefault: false,
    };
    profiles = [...profiles, profile];
    save();
    startEditing(profile.id, profile.name);
  }

  function deleteProfile(id: string) {
    const wasDefault = profiles.find((p) => p.id === id)?.isDefault;
    profiles = profiles.filter((p) => p.id !== id);
    if (wasDefault && profiles.length > 0) {
      profiles[0].isDefault = true;
    }
    save();
  }

  function setDefault(id: string) {
    profiles = profiles.map((p) => ({ ...p, isDefault: p.id === id }));
    save();
  }

  function toggleOption(id: string, option: "systemAudio" | "microphone" | "camera") {
    profiles = profiles.map((p) =>
      p.id === id ? { ...p, [option]: !p[option] } : p,
    );
    save();
  }

  function startEditing(id: string, name: string) {
    editingId = id;
    editingName = name;
  }

  function finishEditing() {
    if (editingId && editingName.trim()) {
      profiles = profiles.map((p) =>
        p.id === editingId ? { ...p, name: editingName.trim() } : p,
      );
      save();
    }
    editingId = null;
    editingName = "";
  }
</script>

<div
  class="flex-1 flex flex-col p-8 w-full max-w-3xl mx-auto animate-in fade-in slide-in-from-bottom-4 duration-500"
>
  <div class="mb-8 flex items-center justify-between">
    <div>
      <h2 class="text-2xl font-bold tracking-tight text-foreground">
        Recording Profiles
      </h2>
      <p class="text-sm text-muted-foreground mt-1">
        Save different recording configurations for quick access.
      </p>
    </div>
    <Button onclick={addProfile} variant="outline" class="gap-1.5">
      <Plus size={14} />
      New Profile
    </Button>
  </div>

  <div class="space-y-3">
    {#each profiles as profile (profile.id)}
      <div
        class="group rounded-xl border bg-card p-4 transition-all duration-200 hover:shadow-sm
          {profile.isDefault
          ? 'border-primary/30 ring-1 ring-primary/10'
          : 'border-border'}"
      >
        <div class="flex items-center justify-between gap-4">
          <!-- Name -->
          <div class="flex items-center gap-3 min-w-0 flex-1">
            {#if editingId === profile.id}
              <input
                bind:value={editingName}
                onblur={finishEditing}
                onkeydown={(e) => e.key === "Enter" && finishEditing()}
                class="text-sm font-semibold bg-transparent border-b border-primary outline-none px-0 py-0.5 w-48"
                autofocus
              />
            {:else}
              <button
                onclick={() => startEditing(profile.id, profile.name)}
                class="text-sm font-semibold text-foreground hover:text-primary transition-colors truncate text-left"
              >
                {profile.name}
              </button>
            {/if}

            {#if profile.isDefault}
              <span
                class="shrink-0 rounded-full bg-primary/10 px-2 py-0.5 text-[10px] font-medium text-primary"
              >
                Default
              </span>
            {/if}
          </div>

          <!-- Toggle options -->
          <div class="flex items-center gap-1">
            <button
              onclick={() => toggleOption(profile.id, "systemAudio")}
              class="size-8 rounded-lg flex items-center justify-center transition-colors
                {profile.systemAudio
                ? 'bg-blue-500/10 text-blue-400'
                : 'bg-muted text-muted-foreground/40 hover:text-muted-foreground'}"
              title={profile.systemAudio ? "System audio: on" : "System audio: off"}
            >
              {#if profile.systemAudio}
                <Volume2 size={14} />
              {:else}
                <VolumeOff size={14} />
              {/if}
            </button>

            <button
              onclick={() => toggleOption(profile.id, "microphone")}
              class="size-8 rounded-lg flex items-center justify-center transition-colors
                {profile.microphone
                ? 'bg-emerald-500/10 text-emerald-400'
                : 'bg-muted text-muted-foreground/40 hover:text-muted-foreground'}"
              title={profile.microphone ? "Microphone: on" : "Microphone: off"}
            >
              {#if profile.microphone}
                <Mic size={14} />
              {:else}
                <MicOff size={14} />
              {/if}
            </button>

            <button
              onclick={() => toggleOption(profile.id, "camera")}
              class="size-8 rounded-lg flex items-center justify-center transition-colors
                {profile.camera
                ? 'bg-violet-500/10 text-violet-400'
                : 'bg-muted text-muted-foreground/40 hover:text-muted-foreground'}"
              title={profile.camera ? "Camera: on" : "Camera: off"}
            >
              {#if profile.camera}
                <Camera size={14} />
              {:else}
                <CameraOff size={14} />
              {/if}
            </button>
          </div>

          <!-- Actions -->
          <div class="flex items-center gap-1 opacity-0 group-hover:opacity-100 transition-opacity">
            {#if !profile.isDefault}
              <button
                onclick={() => setDefault(profile.id)}
                class="rounded-md px-2.5 py-1 text-[10px] font-medium text-muted-foreground hover:bg-muted hover:text-foreground transition-colors"
              >
                Set default
              </button>
            {/if}
            <button
              onclick={() => startEditing(profile.id, profile.name)}
              class="size-7 rounded-md flex items-center justify-center text-muted-foreground/50 hover:text-foreground hover:bg-muted transition-colors"
              title="Rename"
            >
              <Pencil size={12} />
            </button>
            {#if profiles.length > 1}
              <button
                onclick={() => deleteProfile(profile.id)}
                class="size-7 rounded-md flex items-center justify-center text-muted-foreground/50 hover:text-destructive hover:bg-destructive/10 transition-colors"
                title="Delete"
              >
                <Trash2 size={12} />
              </button>
            {/if}
          </div>
        </div>
      </div>
    {/each}
  </div>

  {#if profiles.length === 0}
    <div
      class="flex flex-col items-center justify-center gap-4 rounded-2xl border border-dashed border-border bg-card/50 py-16"
    >
      <p class="text-sm text-muted-foreground">No profiles yet.</p>
      <Button onclick={addProfile} variant="outline" class="gap-1.5">
        <Plus size={14} />
        Create your first profile
      </Button>
    </div>
  {/if}
</div>
