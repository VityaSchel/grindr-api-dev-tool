<script lang="ts">
	import CheckIcon from "phosphor-svelte/lib/CheckIcon";
	import CopyIcon from "phosphor-svelte/lib/CopyIcon";

	import { Button, type ButtonProps } from "$lib/components/ui/button";

	let { value, children, ...rest }: ButtonProps & { value: string } =
		$props();

	let copied = $state(false);
	let timer: ReturnType<typeof setTimeout> | undefined;

	function copy() {
		void navigator.clipboard.writeText(value);
		copied = true;
		clearTimeout(timer);
		timer = setTimeout(() => (copied = false), 1500);
	}

	$effect(() => () => clearTimeout(timer));
</script>

<Button onclick={copy} {...rest}>
	{#if copied}
		<CheckIcon class="text-green-600 dark:text-green-400" />
	{:else}
		<CopyIcon />
	{/if}
	{@render children?.()}
</Button>
