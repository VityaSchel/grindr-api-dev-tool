<script lang="ts">
	import "./layout.css";
	import { onMount } from "svelte";
	import { beforeNavigate } from "$app/navigation";
	import { WebviewWindow } from "@tauri-apps/api/webviewWindow";
	import * as Resizable from "$lib/components/ui/resizable";
	import AppSidebar from "$lib/components/AppSidebar.svelte";
	import Navbar from "$lib/components/Navbar.svelte";
	import { accounts } from "$lib/accounts.svelte";
	import { requestState } from "$lib/request-state.svelte";

	const { children } = $props();

	onMount(() => {
		// Load saved accounts on launch; failures degrade gracefully to unauthorized.
		void accounts.load();
	});

	// If the current request has unsaved edits, a link click (to a type or another
	// endpoint) opens the target in a new window instead of discarding the request.
	beforeNavigate((nav) => {
		if (!requestState.dirty || nav.type !== "link" || !nav.to) return;
		nav.cancel();
		const { pathname, search, hash } = nav.to.url;
		const label = `win-${Date.now()}`;
		try {
			new WebviewWindow(label, {
				url: `${pathname}${search}${hash}`,
				title: "Grindr API",
				width: 1100,
				height: 760,
			});
		} catch (e) {
			console.error("failed to open link in a new window", e);
		}
	});
</script>

<Resizable.PaneGroup direction="horizontal" class="max-h-dvh">
	<Resizable.Pane defaultSize={20} maxSize={60} class="flex min-w-60 flex-col">
		<AppSidebar />
	</Resizable.Pane>
	<Resizable.Handle
		class="cursor-col-resize! bg-transparent px-2 after:w-px after:bg-border"
	/>
	<Resizable.Pane class="flex min-w-0 flex-col">
		<Navbar />
		<div class="flex min-h-0 flex-1 flex-col gap-4 overflow-auto p-4">
			{@render children?.()}
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
