<script lang="ts">
	import { onMount } from "svelte";
	import type { Component } from "svelte";

	let { json, fill = false }: { json: unknown; fill?: boolean } = $props();

	let Editor = $state<Component<Record<string, unknown>> | null>(null);

	onMount(async () => {
		const mod = await import("svelte-jsoneditor");
		Editor = mod.JSONEditor;
	});
</script>

<div
	class={[
		"jse-wrap overflow-hidden rounded-lg border select-text",
		{ "min-h-0 flex-1": fill, fill },
	]}
>
	{#if Editor}
		{@const C = Editor}
		<C
			content={{ json }}
			readOnly={true}
			mode="tree"
			mainMenuBar={false}
			navigationBar={false}
			statusBar={false}
		/>
	{:else}
		<p class="p-3 text-xs text-muted-foreground">Loading viewer...</p>
	{/if}
</div>

<style>
	.jse-wrap {
		--jse-theme-color: var(--muted);
		font-size: 0.8125rem;
	}
	.jse-wrap :global(.jse-main) {
		min-height: 8rem;
		max-height: 32rem;
	}
	.jse-wrap.fill :global(.jse-main) {
		min-height: 0;
		max-height: none;
	}
</style>
