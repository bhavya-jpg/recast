<script lang="ts">
	import { cn } from "@recast/utils";
	let { children, class: className = "", id = "" } = $props();
</script>

<section {id} class={cn("section-padding", className)}>
	{@render children()}
</section>
