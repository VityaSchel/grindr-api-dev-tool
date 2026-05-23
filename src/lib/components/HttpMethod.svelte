<script lang="ts">
	import * as Select from "$lib/components/ui/select";

	const methods = [
		{ value: "GET", label: "GET", color: "oklch(0.7 0.1702 146.12)" },
		{ value: "POST", label: "POST", color: "oklch(0.7 0.1258 72)" },
		{ value: "PUT", label: "PUT", color: "oklch(0.7 0.1455 253.06)" },
		{ value: "DELETE", label: "DELETE", color: "oklch(0.7 0.185 31.76)" },
		{ value: "PATCH", label: "PATCH", color: "oklch(0.7 0.1949 316.59)" },
	];

	let value = $state(methods[0].value);
	const item = methods.find((f) => f.value === value);
</script>

<Select.Root type="single" name="httpMethod" bind:value>
	<Select.Trigger class="w-20 shrink-0" style="color: {item?.color}">
		{item?.label}
	</Select.Trigger>
	<Select.Content class="max-w-20">
		<Select.Group>
			{#each methods as method (method.value)}
				<Select.Item
					value={method.value}
					label={method.label}
					style="color: {method.color}"
				>
					{method.label}
				</Select.Item>
			{/each}
		</Select.Group>
	</Select.Content>
</Select.Root>
