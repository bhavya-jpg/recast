<script lang="ts">
	import type { EditorStore } from "$lib/stores/editor-store.svelte";

	interface Props {
		store: EditorStore;
		previewSrc: string;
		isRendering: boolean;
	}

	let { store, previewSrc, isRendering }: Props = $props();
</script>

<div class="relative flex-1 overflow-hidden rounded-xl border border-border/60 bg-black/30">
	{#if previewSrc}
		<img
			src={previewSrc}
			alt="Rendered preview"
			class="h-full w-full object-contain"
			draggable="false"
		/>
	{:else}
		<div class="flex h-full w-full items-center justify-center text-sm text-muted-foreground">
			Preview unavailable
		</div>
	{/if}

	<div class="absolute left-3 top-3 flex items-center gap-2 pointer-events-none">
		<div class="rounded-md bg-black/50 px-2 py-1 text-[10px] font-medium text-white/80 backdrop-blur-sm">
			{store.metadata
				? `${store.metadata.width}×${store.metadata.height}`
				: "—"}
		</div>
		{#if isRendering}
			<div class="rounded-md bg-black/50 px-2 py-1 text-[10px] font-medium text-white/80 backdrop-blur-sm">
				Rendering
			</div>
		{/if}
	</div>
</div>
