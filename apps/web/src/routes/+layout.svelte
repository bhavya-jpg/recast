<script lang="ts">
	import "../app.css";
	import { ModeWatcher } from "mode-watcher";
	import { Toaster } from "svelte-sonner";
	let { children } = $props();
</script>

<ModeWatcher />
<Toaster position="top-center" />

<div class="relative flex min-h-screen flex-col overflow-x-hidden">
	{@render children()}
</div>

<style>
	:global(html) {
		scroll-behavior: smooth;
	}
</style>
