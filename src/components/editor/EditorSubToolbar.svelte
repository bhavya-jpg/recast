<script lang="ts">
  import { Button } from "$components/ui/button";
  import * as DropdownMenu from "$components/ui/dropdown-menu";
  import type { EditorStore, ExportFormat, ExportQuality } from "$lib/stores/editor-store.svelte";
  import { ChevronDown, Crop, LayoutGrid } from "@lucide/svelte";

  interface Props {
    store: EditorStore;
  }

  let { store }: Props = $props();
  let showFormatMenu = $state(false);
  let showQualityMenu = $state(false);

  const formats: { value: ExportFormat; label: string; desc: string }[] = [
    { value: "mp4", label: "MP4", desc: "Best quality, universal" },
    { value: "webm", label: "WebM", desc: "Web-optimized, smaller" },
    { value: "gif", label: "GIF", desc: "Animated, shareable" },
  ];

  const qualities: { value: ExportQuality; label: string; desc: string }[] = [
    { value: "small", label: "Small", desc: "Up to 720p" },
    { value: "hd", label: "HD", desc: "Up to 1080p" },
    { value: "4k", label: "4K", desc: "Up to 2160p" },
    { value: "source", label: "Source", desc: "Original resolution" },
  ];
</script>

<div class="h-9 flex items-center justify-between px-3 border-b border-border bg-card/50 backdrop-blur-sm shrink-0">
  <div class="flex items-center gap-0.5 rounded-md bg-muted p-0.5">
    <Button
      variant={store.layoutMode === "auto" ? "secondary" : "ghost"}
      size="sm"
      class="h-6 gap-1 text-[11px]"
      onclick={() => (store.layoutMode = "auto")}
    >
      <LayoutGrid size={12} />
      Auto
    </Button>
    <Button
      variant={store.layoutMode === "crop" ? "secondary" : "ghost"}
      size="sm"
      class="h-6 gap-1 text-[11px]"
      onclick={() => (store.layoutMode = "crop")}
    >
      <Crop size={12} />
      Crop
    </Button>
  </div>

  <div class="flex items-center gap-1.5">
    <DropdownMenu.Root bind:open={showFormatMenu}>
      <DropdownMenu.Trigger>
        <Button variant="outline" size="sm" class="h-6 gap-1 text-[11px]">
          {store.exportFormat.toUpperCase()}
          <ChevronDown size={10} />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" class="w-48" preventScroll={false}>
        {#each formats as fmt}
          <DropdownMenu.Item
            onclick={() => { store.exportFormat = fmt.value; showFormatMenu = false; }}
          >
            <div>
              <div class="text-xs font-medium">{fmt.label}</div>
              <div class="text-[10px] text-muted-foreground">{fmt.desc}</div>
            </div>
          </DropdownMenu.Item>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>

    <DropdownMenu.Root bind:open={showQualityMenu}>
      <DropdownMenu.Trigger>
        <Button variant="outline" size="sm" class="h-6 gap-1 text-[11px]">
          {store.exportQuality.toUpperCase()}
          <ChevronDown size={10} />
        </Button>
      </DropdownMenu.Trigger>
      <DropdownMenu.Content align="end" class="w-48" preventScroll={false}>
        {#each qualities as q}
          <DropdownMenu.Item
            onclick={() => { store.exportQuality = q.value; showQualityMenu = false; }}
          >
            <div>
              <div class="text-xs font-medium">{q.label}</div>
              <div class="text-[10px] text-muted-foreground">{q.desc}</div>
            </div>
          </DropdownMenu.Item>
        {/each}
      </DropdownMenu.Content>
    </DropdownMenu.Root>
  </div>
</div>
