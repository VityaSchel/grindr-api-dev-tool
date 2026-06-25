<script lang="ts">
	import { grindrApiHref } from "$lib/links";
	import {
		getOperations,
		getOperationsForTag,
		getTag,
		getSchemasForTag,
		getParamGroupsForTag,
		resolveGroupParams,
	} from "$lib/openapi";
	import Markdown from "$lib/components/Markdown.svelte";
	import { Separator } from "$lib/components/ui/separator";
	import RequestBuilder from "$lib/components/grindr-api/RequestBuilder.svelte";
	import OperationSection from "$lib/components/grindr-api/OperationSection.svelte";
	import SchemaSection from "$lib/components/grindr-api/SchemaSection.svelte";
	import ParamTable from "$lib/components/grindr-api/ParamTable.svelte";
	import WIPBadge from "$lib/components/WIPBadge.svelte";

	// The `[...path]` rest value, e.g. "v3/cascade" — owned by the tab, not the URL,
	// so the view stays mounted (and its request state alive) across tab switches.
	// `method` scopes the page to a single HTTP method (one tab per method); it's
	// `null` for tag pages.
	let {
		path: tagName,
		method = null,
	}: { path: string; method?: string | null } = $props();

	const pathKey = $derived("/" + tagName);
	const operations = $derived(getOperations(pathKey));
	// The operation this tab is scoped to (falls back to the first if the method query
	// is missing or unknown).
	const op = $derived(
		operations.find((o) => o.method === method) ?? operations[0],
	);
	const tagDef = $derived(getTag(tagName));
	const tagSchemas = $derived(getSchemasForTag(tagName));
	const tagOperations = $derived(getOperationsForTag(tagName));
	const paramGroups = $derived(getParamGroupsForTag(tagName));

	const isEndpointPage = $derived(operations.length > 0);
	const isTagPage = $derived(
		!isEndpointPage &&
			(tagDef !== undefined ||
				tagSchemas.length > 0 ||
				tagOperations.length > 0),
	);

	const uniqueTags = $derived(op?.tags ?? []);
</script>

{#if isEndpointPage}
	<div class="max-w-4xl">
		<div class="flex flex-wrap gap-1.5 px-6 pt-6">
			{#each uniqueTags as tag (tag)}
				<a
					href={grindrApiHref(tag)}
					class="rounded-full bg-muted px-2 py-0.5 font-mono text-xs text-muted-foreground hover:bg-muted/80 hover:text-foreground"
				>
					{tag}
				</a>
			{/each}
		</div>

		{#key `${pathKey}:${op.method}`}
			<RequestBuilder path={pathKey} operations={[op]} />
		{/key}

		<div class="px-6 pb-6">
			<Separator class="my-8" />
			<h2
				class="mb-4 text-sm font-semibold tracking-wide text-muted-foreground uppercase"
			>
				Documentation
			</h2>
			<OperationSection {op} path={pathKey} currentPage />
		</div>
	</div>
{:else if isTagPage}
	<div class="max-w-4xl p-6">
		<div class="mb-6">
			<div class="flex flex-wrap items-center gap-2">
				<h1 class="text-xl font-bold capitalize">
					{tagName.replace(/\//g, " / ")}
				</h1>
				{#if tagDef?.["x-wip"]}
					<WIPBadge />
				{/if}
			</div>

			{#if tagDef?.["x-wip-note"]}
				<div
					class="mt-3 rounded-md border-l-4 border-l-yellow-400 bg-yellow-50 px-4 py-3 dark:bg-yellow-900/10"
				>
					<Markdown text={tagDef["x-wip-note"]} />
				</div>
			{/if}

			{#if tagDef?.description}
				<div class="mt-3 text-muted-foreground">
					<Markdown text={tagDef.description} />
				</div>
			{/if}
		</div>

		{#if tagOperations.length}
			<div class="mb-10">
				<h2
					class="mb-4 text-sm font-semibold tracking-wide text-muted-foreground uppercase"
				>
					Endpoints
				</h2>
				{#each tagOperations as { path, op }, i (`${op.method}:${path}`)}
					{#if i > 0}
						<Separator class="my-8" />
					{/if}
					<OperationSection {op} {path} />
				{/each}
			</div>
		{/if}

		{#if paramGroups.length}
			<div class="mb-10">
				<h2
					class="mb-4 text-sm font-semibold tracking-wide text-muted-foreground uppercase"
				>
					Parameter Groups
				</h2>
				{#each paramGroups as { name, group } (name)}
					<div id={name} class="mb-6 scroll-mt-4">
						<h3 class="mb-1 font-mono text-sm font-semibold">{name}</h3>
						{#if group["x-inherits"]}
							<p class="mb-2 text-xs text-muted-foreground">
								Extends: <span class="font-mono">{group["x-inherits"]}</span>
							</p>
						{/if}
						<ParamTable parameters={resolveGroupParams(group)} />
					</div>
				{/each}
			</div>
		{/if}

		{#if tagSchemas.length}
			<div>
				<h2
					class="mb-4 text-sm font-semibold tracking-wide text-muted-foreground uppercase"
				>
					Schemas
				</h2>
				{#each tagSchemas as { name } (name)}
					<SchemaSection {name} />
					<Separator class="my-4" />
				{/each}
			</div>
		{/if}
	</div>
{:else}
	<div class="p-6 text-muted-foreground">
		<p class="text-sm">
			No endpoint or tag found for <code class="font-mono">{pathKey}</code>
		</p>
	</div>
{/if}
