<script lang="ts">
	import { resolve } from "$app/paths";
	import * as Sidebar from "$lib/components/ui/sidebar";
	import { Input } from "$lib/components/ui/input";
	import {
		navEntries,
		type NavEndpoint,
		type NavEntry,
	} from "$lib/components/sidebar";
	import SidebarLink from "$lib/components/SidebarLink.svelte";

	let filter = $state("");

	export function toTitle(str: string): string {
		return str
			.split(/[-_/]/)
			.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
			.join(" ");
	}

	const filteredEntries = $derived.by(() => {
		const q = filter.trim().toLowerCase();
		if (!q) return navEntries;
		return navEntries.flatMap<NavEntry>((entry) => {
			if (entry.kind === "standalone") {
				if (entry.title.toLowerCase().includes(q)) return [entry];
				const items = entry.items.filter(
					(item) =>
						item.title.toLowerCase().includes(q) ||
						item.url.toLowerCase().includes(q),
				);
				return items.length ? [{ ...entry, items }] : [];
			} else {
				if (entry.label.toLowerCase().includes(q)) return [entry];
				const subGroups = entry.subGroups.flatMap((sg) => {
					if (sg.title.toLowerCase().includes(q)) return [sg];
					const items = sg.items.filter(
						(item) =>
							item.title.toLowerCase().includes(q) ||
							item.url.toLowerCase().includes(q),
					);
					return items.length ? [{ ...sg, items }] : [];
				});
				return subGroups.length ? [{ ...entry, subGroups }] : [];
			}
		});
	});
</script>

{#snippet endpointList(items: NavEndpoint[])}
	<Sidebar.MenuSub>
		{#each items as item (`${item.url}:${item.title}`)}
			<Sidebar.MenuSubItem>
				<Sidebar.MenuSubButton>
					{#snippet child({ props })}
						<a
							href={resolve(`/grindr-api/${item.url.substring(1)}`)}
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
	{#each filteredEntries as entry (entry.kind === "standalone" ? entry.title : entry.label)}
		<Sidebar.Group>
			<Sidebar.Menu>
				<Sidebar.MenuItem>
					<Sidebar.MenuButton class="font-medium">
						{#snippet child({ props })}
							<SidebarLink
								href={resolve(
									`/grindr-api/${entry.kind === "standalone" ? entry.title : entry.label}`,
								)}
								{...props}
							>
								{toTitle(
									entry.kind === "standalone" ? entry.title : entry.label,
								)}
							</SidebarLink>
						{/snippet}
					</Sidebar.MenuButton>
					{#if entry.kind === "standalone"}
						{#if entry.items.length}
							{@render endpointList(entry.items)}
						{/if}
					{:else}
						<Sidebar.MenuSub>
							{#each entry.subGroups as subGroup (subGroup.title)}
								<Sidebar.MenuSubItem>
									<Sidebar.MenuSubButton class="font-medium">
										{#snippet child({ props })}
											<SidebarLink
												href={resolve(`/grindr-api/${subGroup.tagName}`)}
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
