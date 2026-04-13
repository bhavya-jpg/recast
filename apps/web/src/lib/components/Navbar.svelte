<script lang="ts">
	import { Button } from "@recast/ui";
	import { Container } from ".";
	import { cn } from "@recast/utils";

	let { class: className = "" } = $props();

	let scrolled = $state(false);

	if (typeof window !== "undefined") {
		window.addEventListener("scroll", () => {
			scrolled = window.scrollY > 50;
		});
	}
</script>

<header
	class={cn(
		"fixed top-0 left-0 right-0 z-50 transition-all duration-300",
		scrolled ? "bg-background/80 backdrop-blur-md border-b border-border/50 py-3" : "py-6",
		className
	)}
>
	<Container class="flex items-center justify-between">
		<div class="flex items-center gap-2">
			<div class="size-8 rounded-lg bg-primary flex items-center justify-center">
				<div class="size-4 rounded-full border-2 border-primary-foreground"></div>
			</div>
			<span class="text-xl font-bold tracking-tight">Recast</span>
		</div>

		<nav class="hidden md:flex items-center gap-8 text-sm font-medium text-muted-foreground">
			<a href="#features" class="hover:text-foreground transition-colors">Features</a>
			<a href="#workflow" class="hover:text-foreground transition-colors">Workflow</a>
			<a href="#showcase" class="hover:text-foreground transition-colors">Showcase</a>
		</nav>

		<div class="flex items-center gap-4">
			<Button variant="ghost" size="sm" class="hidden sm:inline-flex">Log In</Button>
			<Button size="sm">Get Started</Button>
		</div>
	</Container>
</header>
