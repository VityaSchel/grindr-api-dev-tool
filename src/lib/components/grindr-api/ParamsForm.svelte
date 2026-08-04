<script lang="ts">
	import type { Param } from "$lib/openapi";
	import SchemaField from "./SchemaField.svelte";

	let {
		pathParams,
		queryParams,
		pathModel,
		queryModel,
	}: {
		pathParams: Param[];
		queryParams: Param[];
		pathModel: Record<string, unknown>;
		queryModel: Record<string, unknown>;
	} = $props();
</script>

{#snippet section(
	title: string,
	params: Param[],
	model: Record<string, unknown>,
)}
	<section class="flex flex-col gap-3">
		<h3 class="text-xs font-semibold tracking-wide uppercase">{title}</h3>
		{#each params as param (param.name)}
			<SchemaField
				schema={param.schema ?? { type: "string" }}
				label={param.name}
				description={param.description}
				required={param.required}
				value={model[param.name]}
				setValue={(v) => (model[param.name] = v)}
			/>
		{/each}
	</section>
{/snippet}

{#if pathParams.length}
	{@render section("Path parameters", pathParams, pathModel)}
{/if}
{#if queryParams.length}
	{@render section("Query parameters", queryParams, queryModel)}
{/if}
{#if pathParams.length + queryParams.length === 0}
	<p class="text-sm text-muted-foreground select-none">
		This endpoint takes no parameters.
	</p>
{/if}
