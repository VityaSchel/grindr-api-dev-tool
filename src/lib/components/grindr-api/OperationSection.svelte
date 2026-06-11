<script lang="ts">
	import { grindrApiHref } from "$lib/links";
	import type { Operation } from "$lib/openapi";
	import { getParamGroupsForTag } from "$lib/openapi";
	import { METHOD_COLORS } from "$lib/methods";
	import Markdown from "$lib/components/Markdown.svelte";
	import ParamTable from "./ParamTable.svelte";
	import ResponsesTable from "./ResponsesTable.svelte";
	import ErrorsTable from "./ErrorsTable.svelte";
	import SchemaLink from "./SchemaLink.svelte";
	import IdempotentBadge from "$lib/components/IdempotentBadge.svelte";
	import DeprecatedBadge from "$lib/components/DeprecatedBadge.svelte";
	import AuthRequiredBadge from "$lib/components/AuthRequiredBadge.svelte";
	import LegacyBadge from "$lib/components/LegacyBadge.svelte";
	import WIPBadge from "$lib/components/WIPBadge.svelte";

	// `currentPage` = this operation belongs to the page you're already on (the endpoint
	// page itself), so its path is shown as plain text rather than a self-link. On tag
	// pages the operations are *other* endpoints, so their paths link to those pages.
	let {
		op,
		path,
		currentPage = false,
	}: { op: Operation; path: string; currentPage?: boolean } = $props();

	// Link to this operation's own per-method endpoint tab (used on tag pages, where
	// the same path may appear once per HTTP method).
	const pathHref = $derived(
		grindrApiHref(path.replace(/^\//, ""), { method: op.method }),
	);

	/** A `x-see-also` value is either a ready `/grindr-api/...` href or a raw endpoint path. */
	function seeAlsoHref(ref: string): string {
		if (ref.startsWith("/grindr-api/")) return ref;
		return grindrApiHref(ref.replace(/^\//, ""));
	}

	const queryGroupLinks = $derived(
		(op["x-query-groups"] ?? []).map((groupName) => {
			const allGroups = (op.tags ?? []).flatMap((tag) =>
				getParamGroupsForTag(tag).map((g) => ({ ...g, tag })),
			);
			const found = allGroups.find((g) => g.name === groupName);
			return {
				name: groupName,
				tag: found?.tag ?? op.tags[0],
			};
		}),
	);

	const hasErrors = $derived(
		op["x-errors"] && Object.keys(op["x-errors"]).length > 0,
	);
</script>

<section>
	<div class="mb-1 flex flex-wrap items-center gap-2">
		<h2 class="text-base font-semibold">
			{op.summary ?? op.operationId ?? "Unnamed"}
		</h2>

		<div class="flex flex-wrap items-center gap-2">
			{#if op["x-wip"]}
				<WIPBadge />
			{/if}
			{#if op["x-legacy"]}
				<LegacyBadge />
			{/if}
			{#if op["x-paid"]}
				<span
					class="rounded bg-purple-100 px-2 py-0.5 text-xs font-medium text-purple-800 dark:bg-purple-900/30 dark:text-purple-400"
				>
					Paid
				</span>
			{/if}
			{#if op["x-idempotent"]}
				<IdempotentBadge />
			{/if}
			{#if op.deprecated}
				<DeprecatedBadge />
			{/if}
			{#if op.security?.length}
				<AuthRequiredBadge />
			{/if}
		</div>
	</div>

	<div class="mb-3 flex flex-wrap items-center gap-2">
		<span
			class="rounded px-2.5 py-1 font-mono text-sm font-bold"
			style="color: {METHOD_COLORS[
				op.method
			]}; background-color: color-mix(in oklch, {METHOD_COLORS[
				op.method
			]} 15%, transparent);"
		>
			{op.method.toUpperCase()}
		</span>
		{#if currentPage}
			<code class="font-mono text-sm text-muted-foreground">{path}</code>
		{:else}
			<a
				href={pathHref}
				class="font-mono text-sm text-blue-500 hover:underline dark:text-blue-400"
				title="Open this endpoint"
			>
				{path}
			</a>
		{/if}
	</div>

	{#if op["x-wip-note"]}
		<div
			class="mb-4 rounded-md border-l-4 border-l-yellow-400 bg-yellow-50 px-4 py-3 dark:bg-yellow-900/10"
		>
			<Markdown text={op["x-wip-note"]} />
		</div>
	{/if}

	{#if op.description}
		<div class="mb-4 text-muted-foreground">
			<Markdown text={op.description} />
		</div>
	{/if}

	{#if op["x-idempotent"]}
		<p class="mb-3 text-sm text-muted-foreground">
			Repeated requests are completed without errors.
		</p>
	{/if}
	{#if op["x-paid"]}
		<p class="mb-3 text-sm text-muted-foreground">Paid feature.</p>
	{/if}

	{#if op["x-notes"]?.length}
		<div
			class="mb-4 rounded-md border-l-4 border-l-yellow-400 bg-yellow-50 px-4 py-3 dark:bg-yellow-900/10"
		>
			<p
				class="mb-1 text-xs font-semibold tracking-wide text-yellow-800 uppercase dark:text-yellow-400"
			>
				Notes
			</p>
			<ul
				class="list-none space-y-1 text-sm text-yellow-900 dark:text-yellow-300"
			>
				{#each op["x-notes"] as note, ni (ni)}
					<li><Markdown text={note} /></li>
				{/each}
			</ul>
		</div>
	{/if}

	{#if op["x-see-also"]?.length}
		<div class="mb-4 text-sm text-muted-foreground">
			<span class="font-medium">See also: </span>
			{#each op["x-see-also"] as ref, j (ref)}
				<a
					href={seeAlsoHref(ref)}
					class="font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
					>{ref.replace(/^\/grindr-api\//, "")}</a
				>{#if j < (op["x-see-also"]?.length ?? 0) - 1}<span>, </span>{/if}
			{/each}
		</div>
	{/if}

	{#if queryGroupLinks.length}
		<div class="mb-4 text-sm text-muted-foreground">
			<span class="font-medium">Query parameter groups: </span>
			{#each queryGroupLinks as g, gi (g.name)}
				{#if gi > 0}<span>, </span>{/if}
				<a
					href={grindrApiHref(g.tag, { anchor: g.name })}
					class="font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
				>
					{g.name}
				</a>
			{/each}
		</div>
	{/if}

	{#if op.parameters.length}
		<div class="mb-5">
			<h3 class="mb-2 text-sm font-semibold">
				Parameters
				<span class="font-normal text-muted-foreground">
					({op.parameters.length})
				</span>
			</h3>
			<ParamTable parameters={op.parameters} />
		</div>
	{/if}

	{#if op.requestBody}
		<div class="mb-5">
			<h3 class="mb-2 text-sm font-semibold">
				Request Body
				{#if op.requestBody.required}
					<span class="ml-2 text-xs font-normal text-destructive">required</span
					>
				{/if}
			</h3>
			{#if op.requestBody.description}
				<div class="mb-2 text-sm text-muted-foreground">
					<Markdown text={op.requestBody.description} />
				</div>
			{/if}
			{#each Object.entries(op.requestBody.content) as [mime, mediaType] (mime)}
				<div
					class="flex items-center gap-3 rounded-md border px-3 py-2 font-mono text-xs"
				>
					<span class="text-muted-foreground">{mime}</span>
					{#if mediaType.schema}
						<SchemaLink schema={mediaType.schema} />
					{/if}
				</div>
			{/each}
		</div>
	{/if}

	<div class="mb-5">
		<h3 class="mb-2 text-sm font-semibold">Responses</h3>
		<ResponsesTable responses={op.responses} />
	</div>

	{#if hasErrors}
		<div class="mb-5">
			<h3 class="mb-2 text-sm font-semibold">API Errors</h3>
			<ErrorsTable errors={op["x-errors"]!} />
		</div>
	{/if}
</section>
