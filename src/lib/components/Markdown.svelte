<script lang="ts">
	import { marked } from "marked";

	let { text }: { text: string } = $props();

	const html = $derived(marked(text, { async: false }) as string);
</script>

<div class="md">
	<!-- eslint-disable-next-line svelte/no-at-html-tags -->
	{@html html}
</div>

<style>
	.md :global(p) {
		margin-bottom: 0.5rem;
		font-size: 0.875rem;
		line-height: 1.6;
	}
	.md :global(p:last-child) {
		margin-bottom: 0;
	}
	.md :global(a) {
		color: hsl(217 91% 60%);
		text-decoration: underline;
		text-underline-offset: 2px;
	}
	.md :global(code),
	.md :global(pre) {
		font-family: var(--font-mono);
		font-size: 0.8125rem;
		background: var(--muted);
	}
	.md :global(code) {
		padding: 0.1em 0.35em;
		border-radius: 0.25rem;
		overflow-wrap: anywhere;
	}
	.md :global(pre) {
		padding: 0.75rem 1rem;
		border-radius: 0.375rem;
		overflow-x: auto;
		margin-bottom: 0.75rem;
	}
	.md :global(pre code) {
		background: none;
		padding: 0;
	}
	.md :global(ul),
	.md :global(ol) {
		margin-bottom: 0.5rem;
		padding-left: 1.5rem;
		font-size: 0.875rem;
	}
	.md :global(ul) {
		list-style-type: disc;
	}
	.md :global(ol) {
		list-style-type: decimal;
	}
	.md :global(li) {
		margin-bottom: 0.15rem;
	}
	.md :global(h1),
	.md :global(h2),
	.md :global(h3),
	.md :global(h4) {
		font-weight: 600;
		margin-bottom: 0.4rem;
	}
	.md :global(blockquote) {
		border-left: 3px solid var(--border);
		padding-left: 0.75rem;
		color: var(--muted-foreground);
		font-style: italic;
		margin-bottom: 0.5rem;
	}
	.md :global(hr) {
		border: none;
		border-top: 1px solid var(--border);
		margin: 0.75rem 0;
	}
</style>
