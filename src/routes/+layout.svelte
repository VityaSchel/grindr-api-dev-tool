<script lang="ts">
	import "./layout.css";
	import * as Resizable from "$lib/components/ui/resizable";
	import AppSidebar from "$lib/components/AppSidebar.svelte";

	const { children } = $props();
</script>

<Resizable.PaneGroup direction="horizontal" class="max-h-dvh">
	<Resizable.Pane defaultSize={20} maxSize={60} class="flex min-w-60 flex-col">
		<AppSidebar />
	</Resizable.Pane>
	<Resizable.Handle
		class="cursor-col-resize! bg-transparent px-2 after:w-px after:bg-border"
	/>
	<Resizable.Pane>
		<div class="flex flex-1 flex-col gap-4 p-4 overflow-auto h-full">
			{@render children?.()}
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
