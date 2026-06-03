<script lang="ts">
	import { Accordion as AccordionPrimitive } from "bits-ui";

	let { ref = $bindable(null), ...restProps }: AccordionPrimitive.RootProps =
		$props();
</script>

<AccordionPrimitive.Root bind:ref data-slot="accordion" {...restProps} />
