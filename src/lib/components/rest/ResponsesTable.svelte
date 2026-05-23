<script lang="ts">
	import type { Response } from "$lib/openapi";
	import SchemaLink from "./SchemaLink.svelte";

	let { responses }: { responses: Record<string, Response> } = $props();
</script>

<div class="overflow-hidden rounded-md border">
	<table class="w-full text-sm">
		<thead>
			<tr
				class="border-b bg-muted/40 text-left text-xs tracking-wide text-muted-foreground uppercase"
			>
				<th class="px-3 py-2 font-medium">Code</th>
				<th class="px-3 py-2 font-medium">Description</th>
				<th class="px-3 py-2 font-medium">Schema</th>
			</tr>
		</thead>
		<tbody>
			{#each Object.entries(responses) as [code, resp] (code)}
				<tr class="border-b last:border-0 hover:bg-muted/20">
					<td
						class="px-3 py-2 align-top font-mono text-xs
						{code.startsWith('2')
							? 'text-green-600 dark:text-green-400'
							: code.startsWith('4') || code.startsWith('5')
								? 'text-destructive'
								: 'text-muted-foreground'}"
					>
						{code}
					</td>
					<td class="px-3 py-2 align-top text-xs text-muted-foreground"
						>{resp.description}</td
					>
					<td class="px-3 py-2 align-top">
						{#if resp.content}
							{#each Object.values(resp.content) as mt, mi (mi)}
								<SchemaLink schema={mt.schema} />
							{/each}
						{/if}
					</td>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
