<script lang="ts" module>
	import { type VariantProps, tv } from "tailwind-variants";

	export const toggleVariants = tv({
		base: "inline-flex items-center justify-center gap-2 rounded-md text-sm font-medium transition-[color,box-shadow] outline-none hover:bg-muted hover:text-muted-foreground focus-visible:border-ring focus-visible:ring-ring/50 focus-visible:ring-3 disabled:pointer-events-none disabled:opacity-50 data-[state=on]:bg-accent data-[state=on]:text-accent-foreground [&_svg:not([class*='size-'])]:size-4 [&_svg]:pointer-events-none [&_svg]:shrink-0 whitespace-nowrap",
		variants: {
			variant: {
				default: "bg-transparent",
				outline:
					"border border-input data-[state=on]:bg-primary shadow-xs hover:bg-accent hover:text-accent-foreground",
			},
			size: {
				default: "h-8 min-w-8 px-2",
				sm: "h-7 min-w-7 px-1.5 text-xs",
				lg: "h-9 min-w-9 px-2.5",
			},
		},
		defaultVariants: {
			variant: "default",
			size: "default",
		},
	});

	export type ToggleVariant = VariantProps<typeof toggleVariants>["variant"];
	export type ToggleSize = VariantProps<typeof toggleVariants>["size"];
</script>

<script lang="ts">
	import { Toggle as TogglePrimitive } from "bits-ui";
	import type { Snippet } from "svelte";
	import { cn, type WithElementRef } from "$lib/utils.js";

	let {
		ref = $bindable(null),
		pressed = $bindable(false),
		class: className,
		size = "default",
		variant = "default",
		children,
		...restProps
	}: WithElementRef<Omit<TogglePrimitive.RootProps, "children">> & {
		variant?: ToggleVariant;
		size?: ToggleSize;
		children?: Snippet;
	} = $props();
</script>

<TogglePrimitive.Root
	bind:ref
	bind:pressed
	data-slot="toggle"
	class={cn(toggleVariants({ variant, size }), className)}
	{...restProps}
>
	{@render children?.()}
</TogglePrimitive.Root>
