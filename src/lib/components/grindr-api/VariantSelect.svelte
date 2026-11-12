<script lang="ts">
	import * as Select from "$lib/components/ui/select";

	let {
		labels,
		selected,
		onSelect,
	}: {
		labels: string[];
		selected: number;
		onSelect: (index: number) => void;
	} = $props();
</script>

<span class="text-xs text-muted-foreground">
	Accepts {labels.length} shapes — pick the one to send:
</span>
<Select.Root
	type="single"
	value={String(selected)}
	onValueChange={(v) => onSelect(Number(v))}
>
	<Select.Trigger class="w-full">{labels[selected]}</Select.Trigger>
	<Select.Content>
		{#each labels as label, i (i)}
			<Select.Item value={String(i)}>{label}</Select.Item>
		{/each}
	</Select.Content>
</Select.Root>
