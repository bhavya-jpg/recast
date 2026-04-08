<script lang="ts">
	import type { EditorStore } from "$lib/stores/editor-store.svelte";
	import { Spinner } from "$components/ui/spinner";
	import { Badge } from "$components/ui/badge";
	import { fade } from "svelte/transition";

	interface Props {
		store: EditorStore;
		videoEl: HTMLVideoElement | null;
		previewSrc: string;
		fallbackSrc: string;
		isRendering: boolean;
		hasEffects: boolean;
	}

	let { store, videoEl, previewSrc, fallbackSrc, isRendering, hasEffects }: Props = $props();

	const showRenderedPreview = $derived(hasEffects && !!previewSrc);

	let containerEl: HTMLDivElement | null = $state(null);

	$effect(() => {
		if (!containerEl || !videoEl) return;
		if (!showRenderedPreview && videoEl.parentElement !== containerEl) {
			videoEl.className = "max-h-full max-w-full object-contain rounded-lg";
			videoEl.style.opacity = "1";
			videoEl.style.position = "static";
			videoEl.style.width = "";
			videoEl.style.height = "";
			containerEl.appendChild(videoEl);
		}
	});
</script>

<div
	bind:this={containerEl}
	class="relative flex h-full w-full max-w-280 items-center justify-center overflow-hidden rounded-2xl border border-border bg-muted/30"
>
	{#if showRenderedPreview}
		<img
			in:fade={{ duration: 150 }}
			src={previewSrc}
			alt="Preview"
			class="max-h-full max-w-full object-contain"
			draggable="false"
		/>
	{:else if !videoEl && fallbackSrc}
		<img
			src={fallbackSrc}
			alt="Preview"
			class="max-h-full max-w-full object-contain"
			draggable="false"
		/>
	{:else if !videoEl}
		<div class="flex items-center gap-2 text-sm text-muted-foreground">
			<Spinner class="size-4" />
			<span>Loading preview</span>
		</div>
	{/if}

	{#if isRendering}
		<div class="pointer-events-none absolute right-3 top-3">
			<Badge variant="secondary" class="gap-1.5 text-[10px]">
				<Spinner class="size-3" />
				Rendering
			</Badge>
		</div>
	{/if}
</div>
