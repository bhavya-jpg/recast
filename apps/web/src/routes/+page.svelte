<script lang="ts">
	import { Navbar, Hero, FeatureGrid, Section, Container } from "$lib/components";
	import { Button } from "@recast/ui";
</script>

<svelte:head>
	<title>Recast — Video editing, refined.</title>
	<meta name="description" content="Recast is a fast, minimal, and intentional video editor for the next generation of storytellers." />
</svelte:head>

<Navbar />

<main>
	<Hero />
	<FeatureGrid />

	<Section id="workflow">
		<Container>
			<div class="flex flex-col lg:flex-row items-center gap-20">
				<div class="flex-1">
					<span class="text-primary font-bold text-sm uppercase tracking-widest mb-4 block">Workflow</span>
					<h2 class="text-4xl md:text-6xl font-bold mb-8 tracking-tight">Focus on the <br />story, not the tools.</h2>
					<div class="space-y-8">
						<div class="flex gap-4">
							<div class="size-8 rounded-full bg-primary/20 text-primary flex items-center justify-center shrink-0 mt-1 font-bold">1</div>
							<div>
								<h3 class="text-xl font-bold mb-2">Instant Import</h3>
								<p class="text-muted-foreground">Drag and drop anything. Recast handles every format natively without proxying.</p>
							</div>
						</div>
						<div class="flex gap-4">
							<div class="size-8 rounded-full bg-primary/20 text-primary flex items-center justify-center shrink-0 mt-1 font-bold">2</div>
							<div>
								<h3 class="text-xl font-bold mb-2">Smart Timeline</h3>
								<p class="text-muted-foreground">Magnetic by default, manual when you need it. Reordering clips has never been smoother.</p>
							</div>
						</div>
						<div class="flex gap-4">
							<div class="size-8 rounded-full bg-primary/20 text-primary flex items-center justify-center shrink-0 mt-1 font-bold">3</div>
							<div>
								<h3 class="text-xl font-bold mb-2">Export & Share</h3>
								<p class="text-muted-foreground">Optimized presets for every platform. One click and you're live.</p>
							</div>
						</div>
					</div>
				</div>

				<div class="flex-1 w-full aspect-square bg-muted/50 rounded-3xl border border-border/50 relative overflow-hidden group">
					<div class="absolute inset-0 bg-gradient-to-br from-primary/5 to-secondary/5"></div>
					<div class="absolute inset-10 border border-border/50 rounded-2xl bg-card shadow-2xl p-6 flex flex-col gap-4">
						<div class="h-4 w-1/3 bg-muted rounded"></div>
						<div class="h-32 w-full bg-primary/10 rounded-lg flex items-center justify-center">
							<div class="size-8 rounded-full border-2 border-primary border-t-transparent animate-spin"></div>
						</div>
						<div class="space-y-2 mt-auto">
							<div class="h-3 w-full bg-muted rounded"></div>
							<div class="h-3 w-2/3 bg-muted rounded"></div>
						</div>
					</div>
				</div>
			</div>
		</Container>
	</Section>

	<Section id="cta" class="bg-primary text-primary-foreground transform skew-y-1">
		<Container class="transform -skew-y-1 py-12">
			<div class="flex flex-col md:flex-row items-center justify-between gap-12">
				<div class="max-w-2xl">
					<h2 class="text-4xl md:text-6xl font-bold mb-4">Ready to recast your <br />creation process?</h2>
					<p class="text-primary-foreground/80 text-lg md:text-xl">
						Join thousands of creators who switched to the most intentional editor on the market.
					</p>
				</div>
				<Button size="lg" variant="secondary" class="px-12 py-8 text-xl font-bold shadow-2xl">
					Get Started Now
				</Button>
			</div>
		</Container>
	</Section>
</main>

<footer class="py-12 border-t border-border/50">
	<Container class="flex flex-col md:flex-row items-center justify-between gap-8">
		<div class="flex items-center gap-2">
			<div class="size-6 rounded-lg bg-primary flex items-center justify-center">
				<div class="size-3 rounded-full border border-primary-foreground"></div>
			</div>
			<span class="font-bold">Recast</span>
		</div>
		<p class="text-sm text-muted-foreground">
			© 2026 Recast Technologies. All rights reserved.
		</p>
		<div class="flex items-center gap-6 text-sm text-muted-foreground">
			<a href="/features" class="hover:text-foreground transition-colors">Features</a>
			<a href="/workflow" class="hover:text-foreground transition-colors">Workflow</a>
			<a href="/showcase" class="hover:text-foreground transition-colors">Showcase</a>
		</div>
	</Container>
</footer>
