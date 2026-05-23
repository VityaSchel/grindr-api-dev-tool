<script lang="ts">
	let { errors }: { errors: Record<string, string> } = $props();
</script>

<div class="overflow-hidden rounded-md border">
	<table class="w-full text-sm">
		<thead>
			<tr
				class="border-b bg-muted/40 text-left text-xs tracking-wide text-muted-foreground uppercase"
			>
				<th class="px-3 py-2 font-medium">Error Code</th>
				<th class="px-3 py-2 font-medium">Description</th>
			</tr>
		</thead>
		<tbody>
			{#each Object.entries(errors) as [code, desc] (code)}
				<tr class="border-b last:border-0 hover:bg-muted/20">
					<td class="px-3 py-2 align-top font-mono text-xs text-destructive"
						>{code}</td
					>
					<td class="px-3 py-2 align-top text-xs text-muted-foreground"
						>{desc}</td
					>
				</tr>
			{/each}
		</tbody>
	</table>
</div>
