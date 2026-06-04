<script lang="ts">
	import { getSchema } from "$lib/openapi";
	import type { SchemaObject } from "$lib/openapi";
	import SchemaLink from "./SchemaLink.svelte";
	import Markdown from "$lib/components/Markdown.svelte";
	import { SvelteSet } from "svelte/reactivity";
	import WIPBadge from "$lib/components/WIPBadge.svelte";
	import CheckIcon from "phosphor-svelte/lib/CheckIcon";

	let { name }: { name: string } = $props();

	const schema = $derived(getSchema(name));
	const displayName = $derived(schema?.["x-display-name"] ?? name);

	const baseRefs = $derived(
		schema?.allOf
			? schema.allOf
					.filter((s) => s.$ref)
					.map((s) => s.$ref!.replace("#/components/schemas/", ""))
			: [],
	);

	const mergedProperties = $derived.by(() => {
		if (!schema) return {} as Record<string, SchemaObject>;
		if (schema.properties) return schema.properties;
		if (schema.allOf) {
			const merged: Record<string, SchemaObject> = {};
			for (const entry of schema.allOf) {
				if (entry.properties) Object.assign(merged, entry.properties);
			}
			return merged;
		}
		return {} as Record<string, SchemaObject>;
	});

	const requiredSet = $derived.by(() => {
		const req = new SvelteSet<string>(schema?.required ?? []);
		if (schema?.allOf) {
			for (const entry of schema.allOf) {
				for (const r of entry.required ?? []) req.add(r);
			}
		}
		return req;
	});

	const variants = $derived(schema?.oneOf ?? schema?.anyOf ?? []);
	const isEnum = $derived(!!(schema?.["x-enum-labels"] || schema?.enum));
	const hasProperties = $derived(Object.keys(mergedProperties).length > 0);
	const hasVariants = $derived(variants.length > 0);
</script>

<div id={name} class="mb-8 scroll-mt-4">
	<div class="mb-2 flex flex-wrap items-center gap-2">
		<h3 class="font-mono text-sm font-semibold">{displayName}</h3>
		{#if schema?.["x-wip"]}
			<WIPBadge />
		{/if}
		{#if displayName !== name}
			<span class="font-mono text-xs text-muted-foreground">({name})</span>
		{/if}
	</div>

	{#if !schema}
		<p class="text-sm text-muted-foreground">Schema not found.</p>
	{:else}
		{#if schema.description}
			<div class="mb-3 text-sm text-muted-foreground">
				<Markdown text={schema.description} />
			</div>
		{/if}

		{#if isEnum}
			{#if schema["x-enum-labels"]}
				<div class="overflow-hidden rounded-md border">
					<table class="w-full text-sm">
						<thead>
							<tr
								class="border-b bg-muted/40 text-left text-xs tracking-wide text-muted-foreground uppercase"
							>
								<th class="px-3 py-2 font-medium">Value</th>
								<th class="px-3 py-2 font-medium">Label</th>
							</tr>
						</thead>
						<tbody>
							{#each Object.entries(schema["x-enum-labels"]) as [val, label] (val)}
								<tr class="border-b last:border-0 hover:bg-muted/20">
									<td class="px-3 py-2 font-mono text-xs">{val}</td>
									<td class="px-3 py-2 text-xs text-muted-foreground">
										{label}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{:else}
				<div class="flex flex-wrap gap-1.5">
					{#each schema.enum ?? [] as val (String(val))}
						<code class="rounded bg-muted px-1.5 py-0.5 text-xs">
							{String(val)}
						</code>
					{/each}
				</div>
			{/if}
		{:else if hasVariants}
			<div class="text-sm">
				<span
					class="mr-1 text-xs font-medium tracking-wide text-muted-foreground uppercase"
				>
					One of:
				</span>
				{#each variants as variant, vi (vi)}
					{#if vi > 0}<span class="text-muted-foreground">, </span>{/if}
					{#if variant.$ref}
						<SchemaLink schema={variant} />
					{:else}
						<span class="font-mono text-xs">{variant.type ?? "?"}</span>
					{/if}
				{/each}
			</div>
		{:else}
			{#if baseRefs.length}
				<div class="mb-3 text-sm">
					<span
						class="mr-2 text-xs font-medium tracking-wide text-muted-foreground uppercase"
					>
						Extends:
					</span>
					{#each baseRefs as ref, ri (ref)}
						{#if ri > 0}<span class="text-muted-foreground">, </span>{/if}
						<SchemaLink schema={{ $ref: `#/components/schemas/${ref}` }} />
					{/each}
				</div>
			{/if}

			{#if hasProperties}
				<div class="overflow-hidden rounded-md border">
					<table class="w-full text-sm">
						<thead>
							<tr
								class="border-b bg-muted/40 text-left text-xs tracking-wide text-muted-foreground uppercase"
							>
								<th class="px-3 py-2 font-medium">Property</th>
								<th class="px-3 py-2 font-medium">Type</th>
								<th class="px-3 py-2 font-medium">Required</th>
								<th class="px-3 py-2 font-medium">Description</th>
							</tr>
						</thead>
						<tbody>
							{#each Object.entries(mergedProperties) as [propName, propSchema] (propName)}
								<tr class="border-b last:border-0 hover:bg-muted/20">
									<td
										class="px-3 py-2 align-top font-mono text-xs
										{propSchema.deprecated ? 'text-muted-foreground line-through' : ''}"
									>
										{propName}
									</td>
									<td class="px-3 py-2 align-top">
										<SchemaLink schema={propSchema} />
									</td>
									<td class="px-3 py-2 align-top text-xs">
										{#if requiredSet.has(propName)}
											<span class="font-bold text-destructive">
												<CheckIcon />
											</span>
										{:else}
											<span class="text-muted-foreground">–</span>
										{/if}
									</td>
									<td class="px-3 py-2 align-top text-xs text-muted-foreground">
										{propSchema.description ?? ""}
									</td>
								</tr>
							{/each}
						</tbody>
					</table>
				</div>
			{:else if !baseRefs.length}
				<p class="text-sm text-muted-foreground">No properties documented.</p>
			{/if}
		{/if}
	{/if}
</div>
