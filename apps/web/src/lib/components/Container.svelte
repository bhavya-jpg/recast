<script lang="ts">
	import { cn } from "@recast/utils";
	let { children, class: className = "" } = $props();
</script>

<div class={cn("container", className)}>
	{@render children()}
</div>
