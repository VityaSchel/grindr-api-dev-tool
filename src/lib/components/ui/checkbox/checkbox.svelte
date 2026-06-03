<script lang="ts">
	import { Checkbox as CheckboxPrimitive } from "bits-ui";
	import CheckIcon from "phosphor-svelte/lib/CheckIcon";
	import MinusIcon from "phosphor-svelte/lib/MinusIcon";
	import { cn } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		checked = $bindable(false),
		indeterminate = $bindable(false),
		class: className,
		...restProps
	}: CheckboxPrimitive.RootProps = $props();
</script>

<CheckboxPrimitive.Root
	bind:ref
	data-slot="checkbox"
	class={cn(
		"peer size-4 shrink-0 rounded-[4px] border border-input shadow-xs transition-shadow outline-none focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:cursor-not-allowed disabled:opacity-50 aria-invalid:border-destructive aria-invalid:ring-destructive/20 data-[state=checked]:border-primary data-[state=checked]:bg-primary data-[state=checked]:text-primary-foreground dark:bg-input/30 dark:aria-invalid:ring-destructive/40",
		className,
	)}
	bind:checked
	bind:indeterminate
	{...restProps}
>
	{#snippet children({ checked, indeterminate })}
		<div
			data-slot="checkbox-indicator"
			class="flex size-full items-center justify-center text-current transition-none"
		>
			{#if indeterminate}
				<MinusIcon class="size-3.5" weight="bold" />
			{:else if checked}
				<CheckIcon class="size-3.5" weight="bold" />
			{/if}
		</div>
	{/snippet}
</CheckboxPrimitive.Root>
