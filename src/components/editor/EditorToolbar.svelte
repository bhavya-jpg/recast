<script lang="ts">
  import { Button } from "$components/ui/button";
  import * as DropdownMenu from "$components/ui/dropdown-menu";
  import * as Tooltip from "$components/ui/tooltip";
  import { Separator } from "$components/ui/separator";
  import type { BackgroundType, EditorStore } from "$lib/stores/editor-store.svelte";
  import {
    ArrowLeft,
    ChevronDown,
    Download,
    LoaderCircle,
    Redo2,
    Sparkles,
    Trash2,
    Undo2,
  } from "@lucide/svelte";

  interface Props {
    store: EditorStore;
    filename?: string;
    onback?: () => void;
    onexport?: () => void;
  }

  let { store, filename = "Recording", onback, onexport }: Props = $props();
  let showPresetsMenu = $state(false);

  const presets: {
    label: string;
    bg: BackgroundType;
    value?: string;
    padding: number;
    blur: number;
    layout?: "auto" | "crop";
  }[] = [
    { label: "Studio", bg: "gradient", value: "linear-gradient(135deg, #0f172a 0%, #1d4ed8 100%)", padding: 36, blur: 18, layout: "auto" },
    { label: "Focus", bg: "color", value: "#0b1120", padding: 24, blur: 0, layout: "auto" },
    { label: "Spotlight", bg: "wallpaper", value: "/wallpapers/wallpaper7.png", padding: 56, blur: 36, layout: "auto" },
    { label: "Edge to Edge", bg: "color", value: "#020617", padding: 0, blur: 0, layout: "crop" },
  ];

  function applyPreset(preset: (typeof presets)[0]) {
    store.pushUndoState();
    store.setBackground({ type: preset.bg, value: preset.value ?? store.backgroundValue });
    store.padding = preset.padding;
    store.backgroundBlur = preset.blur;
    if (preset.layout) store.layoutMode = preset.layout;
    showPresetsMenu = false;
  }
</script>

<div class="flex items-center justify-between w-full h-full px-2" data-tauri-drag-region>
  <div class="flex items-center gap-1 min-w-0">
    <Tooltip.Root>
      <Tooltip.Trigger>
        <Button variant="ghost" size="icon-sm" onclick={() => onback?.()} aria-label="Back">
          <ArrowLeft size={16} />
        </Button>
      </Tooltip.Trigger>
      <Tooltip.Content>Back to recordings</Tooltip.Content>
    </Tooltip.Root>

    <Tooltip.Root>
      <Tooltip.Trigger>
        <Button variant="ghost" size="icon-sm" class="text-muted-foreground/40 hover:text-destructive" aria-label="Delete">
          <Trash2 size={14} />
        </Button>
      </Tooltip.Trigger>
      <Tooltip.Content>Delete recording</Tooltip.Content>
    </Tooltip.Root>

    <Separator orientation="vertical" class="mx-1 h-4" />

    <span class="truncate text-xs font-medium text-muted-foreground max-w-44" title={filename}>
      {filename}
    </span>
  </div>

  <div class="flex items-center gap-1.5" data-tauri-drag-region>
    <DropdownMenu.Root bind:open={showPresetsMenu}>
      <DropdownMenu.Trigger>
        <Button variant="ghost" size="sm" class="gap-1.5 text-xs text-muted-foreground">
          <Sparkles size={13} />
          Presets
          <ChevronDown size={11} class="transition-transform duration-200 {showPresetsMenu ? 'rotate-180' : ''}" />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content preventScroll={false}>
        {#each presets as preset}
          <DropdownMenu.Item onclick={() => applyPreset(preset)}>
            {preset.label}
          </DropdownMenu.Item>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>

    <div class="flex items-center">
      <Tooltip.Root>
        <Tooltip.Trigger>
          <Button variant="ghost" size="icon-sm" onclick={() => store.undo()} disabled={!store.canUndo} aria-label="Undo">
            <Undo2 size={14} />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>Undo (Ctrl+Z)</Tooltip.Content>
      </Tooltip.Root>

      <Tooltip.Root>
        <Tooltip.Trigger>
          <Button variant="ghost" size="icon-sm" onclick={() => store.redo()} disabled={!store.canRedo} aria-label="Redo">
            <Redo2 size={14} />
          </Button>
        </Tooltip.Trigger>
        <Tooltip.Content>Redo (Ctrl+Shift+Z)</Tooltip.Content>
      </Tooltip.Root>
    </div>
  </div>

  <div class="flex items-center gap-2 mr-1">
    <Button
      onclick={() => onexport?.()}
      disabled={store.isExporting}
      size="sm"
      class="gap-1.5"
    >
      {#if store.isExporting}
        <LoaderCircle size={14} class="animate-spin" />
        Exporting...
      {:else}
        <Download size={13} />
        Export
      {/if}
    </Button>
  </div>
</div>
