<script lang="ts">
	import { goto } from "$app/navigation";
	import EditorToolbar from "$components/editor/EditorToolbar.svelte";
	import PlaybackControls from "$components/editor/PlaybackControls.svelte";
	import PropertiesPanel from "$components/editor/PropertiesPanel.svelte";
	import Timeline from "$components/editor/Timeline.svelte";
	import VideoPreview from "$components/editor/VideoPreview.svelte";
	import type { EditorRenderState, VideoMetadata } from "$lib/stores/editor-store.svelte";
	import { createEditorStore } from "$lib/stores/editor-store.svelte";
	import { convertFileSrc, invoke } from "@tauri-apps/api/core";
	import { tick } from "svelte";

	interface Props {
		data: {
			filePath: string;
			filename: string;
		};
	}

	interface EditorDocument {
		projectPath: string;
		mediaPath: string;
		cursorPath?: string | null;
		editsPath?: string | null;
		metadata: VideoMetadata;
		renderState: EditorRenderState;
	}

	let { data }: Props = $props();

	const store = createEditorStore();

	let videoEl: HTMLVideoElement | null = $state(null);
	let videoSrc = $state("");
	let documentPath = $state("");
	let mediaPath = $state("");
	let previewSrc = $state("");
	let isLoading = $state(true);
	let isRenderingPreview = $state(false);
	let error = $state("");
	let loadedPath = $state("");
	let previewToken = 0;
	let lastPreviewKey = "";

	function handleTimeUpdate() {
		if (videoEl && store.isPlaying) {
			store.currentTime = videoEl.currentTime;
		}
	}

	function handleVideoEnded() {
		store.isPlaying = false;
	}

	function mergeVideoMetadata(next: Partial<VideoMetadata>) {
		store.metadata = {
			duration: next.duration ?? store.metadata?.duration ?? 0,
			width: next.width ?? store.metadata?.width ?? 0,
			height: next.height ?? store.metadata?.height ?? 0,
			fps: next.fps ?? store.metadata?.fps ?? 30,
			codec: next.codec ?? store.metadata?.codec ?? "unknown",
			sizeBytes: next.sizeBytes ?? store.metadata?.sizeBytes ?? 0,
		};
		if (store.trimEnd <= 0 && store.metadata.duration > 0) {
			store.loadRenderState({ trimEnd: store.metadata.duration });
		}
	}

	async function renderPreview(force = false) {
		if (!documentPath) return;
		const previewTime = store.isPlaying
			? Math.round(store.currentTime * 8) / 8
			: store.currentTime;
		const renderState = store.toRenderState();
		const previewKey = `${documentPath}|${previewTime.toFixed(3)}|${JSON.stringify(renderState)}`;
		if (!force && previewKey === lastPreviewKey) return;
		lastPreviewKey = previewKey;
		const token = ++previewToken;
		isRenderingPreview = true;

		try {
			const frame = await invoke<string>("render_preview_frame", {
				request: {
					inputPath: documentPath,
					time: previewTime,
					renderState,
				},
			});
			if (token === previewToken) {
				previewSrc = frame;
			}
		} catch (err) {
			console.error("Preview render failed", err);
		} finally {
			if (token === previewToken) {
				isRenderingPreview = false;
			}
		}
	}

	function handleVideoLoadedMetadata() {
		if (!videoEl) return;
		mergeVideoMetadata({
			duration: videoEl.duration,
			width: videoEl.videoWidth,
			height: videoEl.videoHeight,
		});
	}

	function handleVideoReady() {
		handleVideoLoadedMetadata();
		isLoading = false;
		void renderPreview(true);
	}

	function handleVideoError() {
		const code = videoEl?.error?.code;
		error = code
			? `Failed to load source media (media error ${code}).`
			: "Failed to load source media.";
		isLoading = false;
	}

	async function loadDocument() {
		error = "";
		isLoading = true;
		previewSrc = "";
		videoSrc = "";
		videoEl?.pause();
		store.metadata = null;
		store.reset();

		try {
			const document = await invoke<EditorDocument>("load_editor_document", {
				path: data.filePath,
			});

			documentPath = document.projectPath;
			mediaPath = document.mediaPath;
			store.videoPath = document.projectPath;
			store.metadata = document.metadata;
			store.loadRenderState(document.renderState);

			videoSrc = convertFileSrc(document.mediaPath);
			await tick();
			videoEl?.load();
		} catch (err) {
			console.error("Failed to load editor document", err);
			error = `Could not load project: ${err}`;
			isLoading = false;
		}
	}

	async function handleExport() {
		if (store.isExporting) return;
		store.isExporting = true;
		store.exportProgress = 0;

		try {
			const result = await invoke<string>("export_video", {
				request: {
					inputPath: documentPath || data.filePath,
					format: store.exportFormat,
					renderState: store.toRenderState(),
				},
			});
			console.log("Export complete:", result);
		} catch (err) {
			console.error("Export failed:", err);
			alert(`Export failed: ${err}`);
		} finally {
			store.isExporting = false;
			store.exportProgress = null;
		}
	}

	function handleBack() {
		goto("/");
	}

	function handleKeydown(e: KeyboardEvent) {
		if (e.defaultPrevented) return;
		if (
			e.target instanceof HTMLInputElement ||
			e.target instanceof HTMLTextAreaElement
		) {
			return;
		}

		switch (e.key) {
			case " ":
				e.preventDefault();
				if (!videoEl) return;
				if (store.isPlaying) {
					videoEl.pause();
					store.isPlaying = false;
				} else {
					videoEl.play();
					store.isPlaying = true;
				}
				break;
			case "ArrowLeft":
				if (videoEl && store.metadata) {
					const frameDur = 1 / (store.metadata.fps || 30);
					videoEl.currentTime = Math.max(0, videoEl.currentTime - frameDur);
					store.currentTime = videoEl.currentTime;
				}
				break;
			case "ArrowRight":
				if (videoEl && store.metadata) {
					const frameDur = 1 / (store.metadata.fps || 30);
					videoEl.currentTime = Math.min(
						store.metadata.duration,
						videoEl.currentTime + frameDur,
					);
					store.currentTime = videoEl.currentTime;
				}
				break;
			case "z":
				if (e.ctrlKey || e.metaKey) {
					e.preventDefault();
					if (e.shiftKey) {
						store.redo();
					} else {
						store.undo();
					}
				}
				break;
		}
	}

	$effect(() => {
		if (!data.filePath || data.filePath === loadedPath) return;
		loadedPath = data.filePath;
		void loadDocument();
	});

	$effect(() => {
		if (!documentPath || isLoading || error) return;
		void renderPreview();
	});

	$effect(() => {
		if (!videoEl) return;
		videoEl.muted = true;
	});
</script>

<svelte:window onkeydown={handleKeydown} />

<div class="flex min-h-screen w-full flex-col overflow-hidden bg-background text-foreground">
	<EditorToolbar
		{store}
		filename={data.filename}
		onback={handleBack}
		onexport={handleExport}
	/>

	<div class="flex flex-1 overflow-hidden">
		<div class="flex flex-1 flex-col overflow-hidden">
			<div class="flex flex-1 items-center justify-center p-6 pb-2">
				{#if isLoading}
					<div class="flex flex-col items-center gap-4 animate-in fade-in duration-500">
						<div class="h-8 w-8 animate-spin rounded-full border-2 border-primary border-t-transparent"></div>
						<p class="text-sm text-muted-foreground">Loading project…</p>
					</div>
				{:else if error}
					<div class="flex flex-col items-center gap-4 text-center animate-in fade-in duration-500">
						<div class="flex h-16 w-16 items-center justify-center rounded-2xl bg-destructive/10 text-destructive">
							<span class="text-2xl">⚠</span>
						</div>
						<p class="max-w-sm text-sm text-muted-foreground">{error}</p>
						<button onclick={handleBack} class="text-sm text-primary hover:underline">
							← Back to recordings
						</button>
					</div>
				{:else}
					<VideoPreview
						{store}
						previewSrc={previewSrc}
						isRendering={isRenderingPreview}
					/>
				{/if}
			</div>

			<PlaybackControls {store} {videoEl} />
			<Timeline {store} {videoEl} />
		</div>

		<div class="w-96 shrink-0">
			<PropertiesPanel {store} />
		</div>
	</div>

	{#if videoSrc}
		<!-- svelte-ignore a11y_media_has_caption -->
		<video
			bind:this={videoEl}
			src={videoSrc}
			ontimeupdate={handleTimeUpdate}
			onended={handleVideoEnded}
			onloadedmetadata={handleVideoLoadedMetadata}
			onloadeddata={handleVideoReady}
			oncanplay={handleVideoReady}
			onerror={handleVideoError}
			class="pointer-events-none absolute -z-10 opacity-0"
			playsinline
			preload="auto"
		></video>
	{/if}

	{#if store.isExporting}
		<div class="fixed inset-0 z-50 flex items-center justify-center bg-background/80 backdrop-blur-sm animate-in fade-in duration-200">
			<div class="flex flex-col items-center gap-4 rounded-2xl border border-border bg-card p-8 shadow-2xl animate-in zoom-in-95 duration-300">
				<div class="h-10 w-10 animate-spin rounded-full border-3 border-primary border-t-transparent"></div>
				<div class="text-center">
					<p class="text-sm font-semibold text-foreground">Exporting video…</p>
					<p class="mt-1 text-xs text-muted-foreground">
						{store.exportFormat.toUpperCase()} • {store.exportProgress !== null
							? `${Math.round(store.exportProgress)}%`
							: "Preparing…"}
					</p>
				</div>
				{#if store.exportProgress !== null}
					<div class="h-1.5 w-48 overflow-hidden rounded-full bg-muted">
						<div
							class="h-full rounded-full bg-linear-to-r from-primary to-blue-400 transition-[width] duration-300"
							style="width: {store.exportProgress}%"
						></div>
					</div>
				{/if}
			</div>
		</div>
	{/if}
</div>
