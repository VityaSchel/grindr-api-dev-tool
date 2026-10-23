<script lang="ts">
	import {
		type NavEndpoint,
		navEntries,
		type NavEntry,
		toTitle,
	} from "$lib/components/sidebar";
	import SidebarLink from "$lib/components/SidebarLink.svelte";
	import { Input } from "$lib/components/ui/input";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { grindrApiHref } from "$lib/links";

	let filter = $state("");

	const filteredEntries = $derived.by(() => {
		const q = filter.trim().toLowerCase();
		if (!q) return navEntries;
		const includesQuery = (text: string) => text.toLowerCase().includes(q);
		const matches = (item: NavEndpoint) =>
			includesQuery(item.title) || includesQuery(item.url);
		return navEntries.flatMap<NavEntry>((entry) => {
			if (includesQuery(entry.title)) return [entry];
			const items = entry.items.filter(matches);
			const subGroups = entry.subGroups.flatMap((sg) => {
				if (includesQuery(sg.title)) return [sg];
				const sgItems = sg.items.filter(matches);
				return sgItems.length ? [{ ...sg, items: sgItems }] : [];
			});
			return items.length || subGroups.length
				? [{ ...entry, items, subGroups }]
				: [];
		});
	});
</script>

{#snippet endpointList(items: NavEndpoint[])}
	<Sidebar.MenuSub>
		{#each items as item (`${item.method}:${item.url}`)}
			<Sidebar.MenuSubItem>
				<Sidebar.MenuSubButton>
					{#snippet child({ props })}
						<a
							href={grindrApiHref(item.url.substring(1), {
								method: item.method,
							})}
							{...props}
							class={["max-w-full", props.class]}
						>
							<span class="block w-full truncate">
								{item.title}
							</span>
						</a>
					{/snippet}
				</Sidebar.MenuSubButton>
			</Sidebar.MenuSubItem>
		{/each}
	</Sidebar.MenuSub>
{/snippet}

<Sidebar.Header>
	<Input
		bind:value={filter}
		type="search"
		placeholder="Search..."
		class="relative z-1 shrink-0 bg-background"
	/>
</Sidebar.Header>
<Sidebar.Content class="h-full min-h-0 flex-1 shrink scrollbar-auto ">
	{#each filteredEntries as entry (entry.title)}
		<Sidebar.Group>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton class="font-medium">
						{#snippet child({ props })}
							<SidebarLink
								href={grindrApiHref(entry.title)}
								{...props}
							>
								{toTitle(entry.title)}
							</SidebarLink>
						{/snippet}
					</Sidebar.MenuButton>
					{#if entry.items.length}
						{@render endpointList(entry.items)}
					{/if}
					{#if entry.subGroups.length}
						<Sidebar.MenuSub>
							{#each entry.subGroups as subGroup (subGroup.title)}
								<Sidebar.MenuSubItem>
									<Sidebar.MenuSubButton class="font-medium">
										{#snippet child({ props })}
											<SidebarLink
												href={grindrApiHref(
													subGroup.tagName,
												)}
												{...props}
											>
												{toTitle(subGroup.title)}
											</SidebarLink>
										{/snippet}
									</Sidebar.MenuSubButton>
									{#if subGroup.items.length}
										{@render endpointList(subGroup.items)}
									{/if}
								</Sidebar.MenuSubItem>
							{/each}
						</Sidebar.MenuSub>
					{/if}
				</Sidebar.MenuItem>
			</Sidebar.Menu>
		</Sidebar.Group>
	{/each}
</Sidebar.Content>
