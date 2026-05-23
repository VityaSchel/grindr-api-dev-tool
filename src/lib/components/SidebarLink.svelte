<script lang="ts">
	import { page } from "$app/state";

	let { ...props }: import("svelte/elements").SvelteHTMLElements["a"] =
		$props();
</script>

<a
	{...props}
	class={[
		props.class,
		{
			"bg-accent": page?.url.pathname === props.href,
		},
	]}>{@render props.children?.()}</a
>
