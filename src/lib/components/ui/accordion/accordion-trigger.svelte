<script lang="ts">
	import { Accordion as AccordionPrimitive } from "bits-ui";
	import CaretDownIcon from "phosphor-svelte/lib/CaretDownIcon";
	import { cn, type WithoutChildrenOrChild } from "$lib/utils.js";
	import type { Snippet } from "svelte";

	let {
		ref = $bindable(null),
		class: className,
		level = 3,
		children,
		...restProps
	}: WithoutChildrenOrChild<AccordionPrimitive.TriggerProps> & {
		level?: AccordionPrimitive.HeaderProps["level"];
		children: Snippet;
	} = $props();
</script>

<AccordionPrimitive.Header {level} class="flex">
	<AccordionPrimitive.Trigger
		bind:ref
		data-slot="accordion-trigger"
		class={cn(
			"flex flex-1 items-center justify-between gap-4 rounded-md py-3 text-left text-sm font-medium transition-all outline-none hover:underline focus-visible:border-ring focus-visible:ring-3 focus-visible:ring-ring/50 disabled:pointer-events-none disabled:opacity-50 [&[data-state=open]>svg]:rotate-180",
			className,
		)}
		{...restProps}
	>
		{@render children?.()}
		<CaretDownIcon
			class="pointer-events-none size-4 shrink-0 text-muted-foreground transition-transform duration-200"
		/>
	</AccordionPrimitive.Trigger>
</AccordionPrimitive.Header>
