<script lang="ts">
  import AppSidebar from "$components/layout/app-sidebar.svelte";
  import CustomTitlebar from "$components/layout/custom-titlebar.svelte";
  import { Button } from "$components/ui/button";
  import { Separator } from "$components/ui/separator";
  import * as Sidebar from "$components/ui/sidebar";
  import { config } from "$constants/app";
  import { launchRecordingPanel } from "$lib/ipc";
  import Radio from "@tabler/icons-svelte/icons/radio";
  let { children } = $props();
</script>

<Sidebar.Provider class="h-full min-h-full fixed inset-0">
  <AppSidebar />
  <Sidebar.Inset class="@container/layout">
    <CustomTitlebar
      class="shrink-0 items-center gap-2 border-b border-border px-2"
    >
      <Sidebar.Trigger />
      <Separator
        orientation="vertical"
        class="me-2 data-[orientation=vertical]:h-4"
      />
      <div class="flex items-center gap-2 px-3 h-full" data-tauri-drag-region>
        <span class="text-sm font-semibold text-foreground"
          >{config.appName}</span
        >
      </div>
      <Button onclick={launchRecordingPanel} variant="default_soft" class="group relative max-w-xs ml-auto h-8">
        <Radio size={16} class="animate-pulse" />
        Launch Panel
      </Button>
    </CustomTitlebar>
    <main class="flex-1 overflow-y-auto no-scrollbar">
      {@render children()}
    </main>
  </Sidebar.Inset>
</Sidebar.Provider>
