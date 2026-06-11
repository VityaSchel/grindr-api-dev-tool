<script lang="ts">
	import { page } from "$app/state";
	import { goto } from "$app/navigation";
	import { resolve } from "$app/paths";
	import { tabs, tabKey, type Tab } from "$lib/tabs.svelte";
	import { grindrApiHref, METHOD_PARAM, CUSTOM_TAB_PATH } from "$lib/links";
	import { navTitle } from "$lib/components/sidebar";
	import { methodColor } from "$lib/methods";
	import * as Tabs from "$lib/components/ui/tabs";
	import XIcon from "phosphor-svelte/lib/XIcon";

	// The active tab is whichever matches the current URL (path + method query).
	const activePath = $derived(page.params.path ?? null);
	const activeMethod = $derived(page.url.searchParams.get(METHOD_PARAM));
	const activeKey = $derived(
		activePath === null ? "" : tabKey(activePath, activeMethod),
	);

	function hrefFor(tab: Tab): string {
		return grindrApiHref(tab.path, { method: tab.method });
	}

	function activate(key: string) {
		if (!key || key === activeKey) return;
		const tab = tabs.tabs.find((t) => tabKey(t.path, t.method) === key);
		if (tab) void goto(hrefFor(tab));
	}

	function close(tab: Tab) {
		const neighbour = tabs.close(tab.path, tab.method);
		// Only navigate if we closed the tab we were viewing.
		if (!(tab.path === activePath && tab.method === activeMethod)) return;
		void goto(neighbour ? hrefFor(neighbour) : resolve("/"));
	}

	function tabLabel(tab: Tab): string {
		return tab.path === CUSTOM_TAB_PATH
			? "Custom request"
			: navTitle(tab.path, tab.method);
	}
</script>

{#if tabs.tabs.length}
	<div class="shrink-0 overflow-x-auto border-b px-2 py-1.5">
		<Tabs.Root bind:value={() => activeKey, (v) => activate(v)}>
			<Tabs.List variant="line">
				{#each tabs.tabs as tab (tabKey(tab.path, tab.method))}
					<Tabs.Trigger
						value={tabKey(tab.path, tab.method)}
						class="max-w-52"
						title={tabLabel(tab)}
					>
						{#if tab.method}
							<span
								class="shrink-0 font-mono text-[0.6rem] font-bold"
								style="color: {methodColor(tab.method)}"
							>
								{tab.method.toUpperCase()}
							</span>
						{/if}
						<span class="min-w-0 truncate">{tabLabel(tab)}</span>
						<!-- A <span> (not <button>) so it's valid inside the trigger button;
						     pointerdown/click are stopped so closing doesn't also select. -->
						<span
							role="button"
							tabindex={0}
							aria-label="Close tab"
							class="ml-0.5 inline-flex shrink-0 items-center rounded p-0.5 opacity-60 hover:bg-destructive/10 hover:text-destructive hover:opacity-100"
							onpointerdown={(e) => e.stopPropagation()}
							onclick={(e) => {
								e.stopPropagation();
								close(tab);
							}}
							onkeydown={(e) => {
								if (e.key === "Enter" || e.key === " ") {
									e.preventDefault();
									e.stopPropagation();
									close(tab);
								}
							}}
						>
							<XIcon class="size-3" />
						</span>
					</Tabs.Trigger>
				{/each}
			</Tabs.List>
		</Tabs.Root>
	</div>
{/if}
