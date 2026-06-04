<script lang="ts">
	import { marked } from "marked";
	import { openUrl } from "@tauri-apps/plugin-opener";

	let { text }: { text: string } = $props();

	const CALLOUT =
		/<blockquote>\s*<p>\s*\[!(NOTE|TIP|IMPORTANT|WARNING|CAUTION)\]\s*(.*?)<\/p>([\s\S]*?)<\/blockquote>/gi;

	function renderCallouts(html: string): string {
		return html.replace(
			CALLOUT,
			(_m, type: string, rest: string, body: string) => {
				const t = type.toLowerCase();
				const head = rest.trim() ? `<p>${rest}</p>` : "";
				return `<div class="callout callout-${t}"><div class="callout-title">${type}</div>${head}${body}</div>`;
			},
		);
	}

	const html = $derived(
		renderCallouts(
			marked((text ?? "").trim(), {
				async: false,
				gfm: true,
				breaks: true,
			}) as string,
		),
	);

	function onClick(e: MouseEvent) {
		const anchor = (e.target as HTMLElement | null)?.closest("a");
		const href = anchor?.getAttribute("href");
		if (href && /^https?:\/\//i.test(href)) {
			e.preventDefault();
			openUrl(href).catch((err) => console.error("failed to open url", err));
		}
	}
</script>

<div class="md" onclick={onClick} role="presentation">
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
		cursor: pointer;
		overflow-wrap: anywhere;
	}
	.md :global(code),
	.md :global(pre) {
		font-family: var(--font-mono);
		font-size: 0.8125rem;
		/* Tint of the surrounding text colour so it stays legible on both the
		   light page background and the dark tooltip background. */
		background: color-mix(in oklab, currentColor 15%, transparent);
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

	/* GitHub-style callouts */
	.md :global(.callout) {
		border-left: 3px solid var(--border);
		border-radius: 0.375rem;
		background: color-mix(in oklch, var(--muted) 60%, transparent);
		padding: 0.5rem 0.75rem;
		margin-bottom: 0.5rem;
		font-size: 0.8125rem;
	}
	.md :global(.callout p) {
		margin-bottom: 0.25rem;
		font-size: 0.8125rem;
	}
	.md :global(.callout p:last-child) {
		margin-bottom: 0;
	}
	.md :global(.callout-title) {
		font-weight: 600;
		font-size: 0.7rem;
		letter-spacing: 0.04em;
		text-transform: uppercase;
		margin-bottom: 0.25rem;
	}
	.md :global(.callout-warning),
	.md :global(.callout-caution) {
		border-left-color: var(--destructive);
	}
	.md :global(.callout-warning .callout-title),
	.md :global(.callout-caution .callout-title) {
		color: var(--destructive);
	}
	.md :global(.callout-note),
	.md :global(.callout-important),
	.md :global(.callout-tip) {
		border-left-color: hsl(217 91% 60%);
	}
	.md :global(.callout-note .callout-title),
	.md :global(.callout-important .callout-title),
	.md :global(.callout-tip .callout-title) {
		color: hsl(217 91% 60%);
	}
</style>
