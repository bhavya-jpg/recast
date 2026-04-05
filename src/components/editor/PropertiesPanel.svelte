<script lang="ts">
	import type { EditorStore } from "$lib/stores/editor-store.svelte";
	import { ImageIcon, MousePointer } from "@lucide/svelte";
	import BackgroundPicker from "./BackgroundPicker.svelte";
	import CursorPanel from "./CursorPanel.svelte";

	interface Props {
		store: EditorStore;
	}

	type PanelTab = "background" | "cursor";

	type PanelDefinition = {
		id: PanelTab;
		label: string;
		hint: string;
		icon: typeof ImageIcon;
	};

	const tabs: PanelDefinition[] = [
		{
			id: "background",
			label: "Background",
			hint: "Canvas background, blur, and frame spacing.",
			icon: ImageIcon,
		},
		{
			id: "cursor",
			label: "Cursor",
			hint: "Pointer visibility and emphasis controls for playback.",
			icon: MousePointer,
		},
	];

	let { store }: Props = $props();

	let activeTab = $state<PanelTab>("background");

	const activePanel = $derived(
		tabs.find((tab) => tab.id === activeTab) ?? tabs[0],
	);

	function formatDuration(seconds: number | undefined) {
		if (!seconds || seconds <= 0) return "--:--";
		const totalSeconds = Math.round(seconds);
		const minutes = Math.floor(totalSeconds / 60);
		const remainderSeconds = totalSeconds % 60;
		return `${minutes}:${remainderSeconds.toString().padStart(2, "0")}`;
	}

	function formatResolution() {
		if (!store.metadata?.width || !store.metadata?.height) return "Unknown";
		return `${store.metadata.width} x ${store.metadata.height}`;
	}

	function formatFrameRate() {
		if (!store.metadata?.fps) return "--";
		return `${Math.round(store.metadata.fps)} fps`;
	}
</script>

<div
	class="flex h-full flex-col border-l border-border/80 bg-linear-to-b from-card via-card/95 to-background/95 backdrop-blur-sm"
>
	<div class="shrink-0 border-b border-border/70 px-4 pb-3 pt-4">

		<div class="mt-3 flex items-center gap-2">
			<div class="rounded-full border border-border/70 bg-background/80 px-2.5 py-1 text-[11px] font-medium text-foreground">
					{formatDuration(store.metadata?.duration)}
			</div>
			<div class="rounded-full border border-border/70 bg-background/80 px-2.5 py-1 text-[11px] font-medium text-foreground">
				{formatResolution()}
			</div>
			<div class="rounded-full border border-border/70 bg-background/80 px-2.5 py-1 text-[11px] font-medium text-foreground">
				{formatFrameRate()}
			</div>
		</div>

		<div class="mt-3 grid grid-cols-2 gap-2">
			{#each tabs as tab}
				{@const Icon = tab.icon}
				<button
					type="button"
					onclick={() => (activeTab = tab.id)}
					aria-pressed={activeTab === tab.id}
					title={tab.hint}
					class="rounded-2xl border px-2 py-2 text-left transition-all duration-200 {activeTab ===
					tab.id
						? 'border-primary/60 bg-primary/10 shadow-[0_10px_24px_rgba(59,130,246,0.1)]'
						: 'border-border/70 bg-background/70 hover:border-border hover:bg-background'}"
				>
					<div class="flex items-center justify-between gap-2">
						<span
							class="flex h-8 w-8 items-center justify-center rounded-xl border border-border/70 bg-muted/70 text-muted-foreground {activeTab ===
							tab.id
								? 'border-primary/20 bg-primary/10 text-primary'
								: ''}"
						>
							<Icon size={14} />
						</span>
					</div>

					<p class="mt-2 text-[11px] font-semibold text-foreground">{tab.label}</p>
				</button>
			{/each}
		</div>
	</div>

	<div class="custom-scrollbar flex-1 overflow-y-auto px-4 py-4">
		{#if activeTab === "background"}
			<BackgroundPicker {store} />
		{:else}
			<CursorPanel {store} />
		{/if}
	</div>
</div>

<style>
	.custom-scrollbar::-webkit-scrollbar {
		width: 6px;
	}

	.custom-scrollbar::-webkit-scrollbar-track {
		background: transparent;
	}

	.custom-scrollbar::-webkit-scrollbar-thumb {
		background: rgba(120, 120, 128, 0.35);
		border-radius: 999px;
	}

	.custom-scrollbar {
		scrollbar-width: thin;
		scrollbar-color: rgba(120, 120, 128, 0.35) transparent;
	}
</style>
