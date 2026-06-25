<script lang="ts">
	import "./layout.css";
	import { onMount, untrack } from "svelte";
	import { page } from "$app/state";
	import { goto, afterNavigate } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { listen } from "@tauri-apps/api/event";
	import * as Resizable from "$lib/components/ui/resizable";
	import AppSidebar from "$lib/components/AppSidebar.svelte";
	import Navbar from "$lib/components/Navbar.svelte";
	import TabBar from "$lib/components/grindr-api/TabBar.svelte";
	import EndpointView from "$lib/components/grindr-api/EndpointView.svelte";
	import CustomRequest from "$lib/components/grindr-api/CustomRequest.svelte";
	import { accounts } from "$lib/accounts.svelte";
	import { tabs, tabKey } from "$lib/tabs.svelte";
	import { grindrApiHref, METHOD_PARAM, CUSTOM_TAB_PATH } from "$lib/links";
	import { getOperations } from "$lib/openapi";

	const { children } = $props();

	const activePath = $derived(page.params.path ?? null);
	const activeMethod = $derived(page.url.searchParams.get(METHOD_PARAM));

	const isMac =
		typeof navigator !== "undefined" && navigator.userAgent.includes("Mac");

	$effect(() => {
		const path = activePath;
		const method = activeMethod;
		if (!path) return;
		untrack(() => {
			if (path !== CUSTOM_TAB_PATH && method === null) {
				const ops = getOperations("/" + path);
				if (ops.length) {
					void goto(grindrApiHref(path, { method: ops[0].method }), {
						replaceState: true,
					});
					return;
				}
			}
			tabs.ensure(path, method);
		});
	});

	function closeActiveTab() {
		if (activePath === null) return;
		const neighbour = tabs.close(activePath, activeMethod);
		void goto(
			neighbour
				? grindrApiHref(neighbour.path, { method: neighbour.method })
				: resolve("/"),
		);
	}

	function onKeydown(e: KeyboardEvent) {
		if ((e.metaKey || e.ctrlKey) && (e.key === "w" || e.key === "W")) {
			e.preventDefault();
			closeActiveTab();
		}
	}

	function scrollToHash(attempt = 0) {
		const hash = page.url.hash.slice(1);
		if (!hash) return;
		const container = document.querySelector<HTMLElement>(
			'[data-scroll-container][data-active="true"]',
		);
		const target = container?.querySelector<HTMLElement>(
			`[id="${CSS.escape(hash)}"]`,
		);
		if (target) {
			target.scrollIntoView({ behavior: "smooth", block: "start" });
		} else if (attempt < 10) {
			requestAnimationFrame(() => scrollToHash(attempt + 1));
		}
	}

	afterNavigate(() => scrollToHash());

	const LINK_PREFIX = "/grindr-api/";

	function backgroundOpen(e: MouseEvent) {
		if (!(e.button === 1 || e.metaKey || e.ctrlKey)) return;
		const anchor = (e.target as HTMLElement | null)?.closest("a[href]");
		const href = anchor?.getAttribute("href");
		if (!href?.startsWith(LINK_PREFIX)) return;
		e.preventDefault();
		e.stopPropagation();
		const url = new URL(href, location.origin);
		// Decode the pathname back to the `[...path]` value; method rides in `?m=`.
		const path = decodeURIComponent(url.pathname.slice(LINK_PREFIX.length));
		tabs.ensure(path, url.searchParams.get(METHOD_PARAM));
	}

	onMount(() => {
		void accounts.load();

		let unlistenCloseTab: (() => void) | undefined;
		listen("close-tab", () => closeActiveTab())
			.then((u) => (unlistenCloseTab = u))
			.catch(() => {});

		document.addEventListener("click", backgroundOpen, true);
		document.addEventListener("auxclick", backgroundOpen, true);
		if (!isMac) window.addEventListener("keydown", onKeydown);
		return () => {
			document.removeEventListener("click", backgroundOpen, true);
			document.removeEventListener("auxclick", backgroundOpen, true);
			if (!isMac) window.removeEventListener("keydown", onKeydown);
			unlistenCloseTab?.();
		};
	});
</script>

<Resizable.PaneGroup direction="horizontal" class="h-dvh!">
	<Resizable.Pane defaultSize={20} maxSize={60} class="flex min-w-60 flex-col">
		<AppSidebar />
	</Resizable.Pane>
	<Resizable.Handle
		class="cursor-col-resize! bg-transparent px-2 after:w-px after:bg-border"
	/>
	<Resizable.Pane class="flex min-w-0 flex-col">
		<Navbar />
		<TabBar />
		<div class="flex min-h-0 flex-1 flex-col">
			{#each tabs.tabs as tab (tabKey(tab.path, tab.method))}
				{@const isActive =
					tab.path === activePath && tab.method === activeMethod}
				<div
					data-scroll-container
					data-active={isActive}
					class={["min-h-0 flex-1 overflow-auto", !isActive && "hidden"]}
				>
					{#if tab.path === CUSTOM_TAB_PATH}
						<CustomRequest />
					{:else}
						<EndpointView path={tab.path} method={tab.method} />
					{/if}
				</div>
			{/each}
			{#if activePath === null}
				<div class="min-h-0 flex-1 overflow-auto p-4">
					{@render children?.()}
				</div>
			{/if}
		</div>
	</Resizable.Pane>
</Resizable.PaneGroup>
