<script lang="ts">
	import type { EditorStore } from "$lib/stores/editor-store.svelte";
	import { fade } from "svelte/transition";

	interface Props {
		store: EditorStore;
		previewSrc: string;
		fallbackSrc: string;
		isRendering: boolean;
	}

	let { store, previewSrc, fallbackSrc, isRendering }: Props = $props();

	const activeSrc = $derived(previewSrc || fallbackSrc);
	const isFallback = $derived(!previewSrc && !!fallbackSrc);
</script>

<div class="relative flex-1 overflow-hidden rounded-[28px] border border-border/60 bg-[radial-gradient(circle_at_top,rgba(59,130,246,0.12),transparent_42%),linear-gradient(180deg,rgba(10,14,23,0.96),rgba(5,7,12,0.98))] shadow-[0_18px_60px_rgba(0,0,0,0.35)]">
	{#if activeSrc}
		<img
			in:fade={{ duration: 180 }}
			src={activeSrc}
			alt="Rendered preview"
			class="h-full w-full object-contain"
			draggable="false"
		/>
	{:else}
		<div class="flex h-full w-full items-center justify-center">
			<div class="rounded-2xl border border-white/10 bg-white/5 px-4 py-3 text-sm text-white/70 backdrop-blur-sm">
				Preview is being prepared
			</div>
		</div>
	{/if}

	<div class="pointer-events-none absolute inset-x-0 top-0 flex items-start justify-between p-4">
		<div class="flex items-center gap-2">
			<div class="rounded-md bg-black/45 px-2.5 py-1 text-[10px] font-medium text-white/80 backdrop-blur-sm">
				{store.metadata ? `${store.metadata.width}x${store.metadata.height}` : "--"}
			</div>
			{#if isRendering}
				<div class="rounded-md bg-black/45 px-2.5 py-1 text-[10px] font-medium text-white/80 backdrop-blur-sm">
					Rendering
				</div>
			{/if}
		</div>

		{#if isFallback}
			<div class="rounded-md bg-amber-500/15 px-2.5 py-1 text-[10px] font-medium text-amber-100 backdrop-blur-sm">
				Thumbnail fallback
			</div>
		{/if}
	</div>

	<div class="pointer-events-none absolute inset-x-0 bottom-0 bg-gradient-to-t from-black/65 via-black/20 to-transparent p-5">
		<div class="flex items-end justify-between gap-4">
			<div>
				<p class="text-sm font-semibold text-white/95">Preview</p>
				<p class="mt-1 text-xs text-white/65">
					{#if isFallback}
						Showing a generated frame while live preview catches up.
					{:else}
						Rust-rendered frame preview for the current timeline state.
					{/if}
				</p>
			</div>
			<div class="rounded-full border border-white/10 bg-white/8 px-3 py-1 text-[11px] font-medium text-white/75 backdrop-blur-sm">
				{store.currentTime.toFixed(2)}s
			</div>
		</div>
	</div>
</div>
