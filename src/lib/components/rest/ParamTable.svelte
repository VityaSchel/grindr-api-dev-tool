<script lang="ts">
	import type { Param } from "$lib/openapi";
	import SchemaLink from "./SchemaLink.svelte";

	let { parameters }: { parameters: Param[] } = $props();
</script>

<div class="overflow-hidden rounded-md border">
	<table class="w-full text-sm">
		<thead>
			<tr
				class="border-b bg-muted/40 text-left text-xs tracking-wide text-muted-foreground uppercase"
			>
				<th class="px-3 py-2 font-medium">Name</th>
				<th class="px-3 py-2 font-medium">In</th>
				<th class="px-3 py-2 font-medium">Type</th>
				<th class="px-3 py-2 font-medium">Req</th>
				<th class="px-3 py-2 font-medium">Description</th>
			</tr>
		</thead>
		<tbody>
			{#each parameters as param (param.name)}
				<tr class="border-b last:border-0 hover:bg-muted/20">
					<td class="px-3 py-2 align-top font-mono text-xs">
						{param.name}
						{#if param.deprecated}
							<span class="ml-1 text-xs text-muted-foreground line-through"
								>depr</span
							>
						{/if}
					</td>
					<td class="px-3 py-2 align-top text-xs text-muted-foreground"
						>{param.in}</td
					>
					<td class="px-3 py-2 align-top">
						<SchemaLink schema={param.schema} />
					</td>
					<td class="px-3 py-2 align-top text-xs">
						{#if param.required}
							<span class="font-bold text-destructive">✓</span>
						{:else}
							<span class="text-muted-foreground">–</span>
						{/if}
					</td>
					<td class="px-3 py-2 align-top text-xs text-muted-foreground"
						>{param.description ?? ""}</td
					>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
