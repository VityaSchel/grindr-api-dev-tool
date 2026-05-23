<script lang="ts">
	import { resolve } from "$app/paths";
	import type { Operation } from "$lib/openapi";
	import { getParamGroupsForTag } from "$lib/openapi";
	import Markdown from "$lib/components/Markdown.svelte";
	import ParamTable from "./ParamTable.svelte";
	import ResponsesTable from "./ResponsesTable.svelte";
	import ErrorsTable from "./ErrorsTable.svelte";
	import SchemaLink from "./SchemaLink.svelte";

	const METHOD_COLORS: Record<string, string> = {
		get: "oklch(0.7 0.1702 146.12)",
		post: "oklch(0.7 0.1258 72)",
		put: "oklch(0.7 0.1455 253.06)",
		delete: "oklch(0.7 0.185 31.76)",
		patch: "oklch(0.7 0.1949 316.59)",
		head: "oklch(0.65 0.01 220)",
		options: "oklch(0.65 0.01 220)",
	};

	let { op, path }: { op: Operation; path: string } = $props();

	// Map x-query-groups entries to their render-on-tag page so we can link there
	const queryGroupLinks = $derived(
		(op["x-query-groups"] ?? []).map((groupName) => {
			// Find which tag this group renders on
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
				<span
					class="rounded bg-yellow-100 px-2 py-0.5 text-xs font-medium text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400"
				>
					WIP
				</span>
			{/if}
			{#if op["x-legacy"]}
				<span
					class="rounded bg-muted px-2 py-0.5 text-xs font-medium text-muted-foreground"
				>
					Legacy
				</span>
			{/if}
			{#if op["x-paid"]}
				<span
					class="rounded bg-purple-100 px-2 py-0.5 text-xs font-medium text-purple-800 dark:bg-purple-900/30 dark:text-purple-400"
				>
					Paid
				</span>
			{/if}
			{#if op["x-idempotent"]}
				<span
					class="rounded bg-blue-100 px-2 py-0.5 text-xs font-medium text-blue-800 dark:bg-blue-900/30 dark:text-blue-400"
				>
					Idempotent
				</span>
			{/if}
			{#if op.deprecated}
				<span
					class="rounded bg-red-100 px-2 py-0.5 text-xs font-medium text-red-800 dark:bg-red-900/30 dark:text-red-400"
				>
					Deprecated
				</span>
			{/if}
			{#if op.security?.length}
				<span
					class="rounded border border-border px-2 py-0.5 text-xs font-medium text-muted-foreground"
				>
					🔒 Auth required
				</span>
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
		<code class="font-mono text-sm text-muted-foreground">{path}</code>
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
				<span class="font-mono text-xs">{ref}</span
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
					href={resolve(`/rest/${g.tag}`) + `#${g.name}`}
					class="font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
					>{g.name}</a
				>
			{/each}
		</div>
	{/if}

	{#if op.parameters.length}
		<div class="mb-5">
			<h3 class="mb-2 text-sm font-semibold">
				Parameters <span class="font-normal text-muted-foreground"
					>({op.parameters.length})</span
				>
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
