<script lang="ts">
	import { Card } from "@recast/ui";
	import { Layers, MousePointer2, Shield, Zap } from "lucide-svelte";
	import { Container, Section } from ".";

	const features = [
		{
			title: "Blazing Fast",
			description:
				"Native performance with GPU acceleration. Edit 4K video without breaking a sweat.",
			icon: Zap,
		},
		{
			title: "Minimal UI",
			description:
				"Interface that gets out of your way. Only the tools you need, exactly when you need them.",
			icon: MousePointer2,
		},
		{
			title: "Layered Precision",
			description:
				"Infinite tracks, sub-pixel positioning, and advanced masking for complex workflows.",
			icon: Layers,
		},
		{
			title: "Cloud Sync",
			description:
				"Your projects are always with you. seamless transition between desktop and the web.",
			icon: Shield,
		},
	];
</script>

<Section id="features" class="bg-muted/30">
	<Container>
		<div class="text-center mb-20">
			<h2 class="text-3xl md:text-5xl font-bold mb-4">
				Everything you need, <br />nothing you don't.
			</h2>
			<p class="text-muted-foreground text-lg max-w-xl mx-auto">
				Built for speed and simplicity. No clutter, just pure
				creativity.
			</p>
		</div>

		<div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8">
			{#each features as feature}
				<Card
					class="p-8 group hover:border-primary/50 transition-all duration-300 bg-card/50 backdrop-blur-sm"
				>
					<div
						class="size-12 rounded-xl bg-primary/10 flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300"
					>
						{#if feature.icon}
							{@const Icon = feature.icon}
							<Icon class="size-6 text-primary" />
						{/if}
					</div>
					<h3 class="text-xl font-bold mb-2">{feature.title}</h3>
					<p class="text-muted-foreground leading-relaxed">
						{feature.description}
					</p>
				</Card>
			{/each}
		</div>
	</Container>
</Section>
