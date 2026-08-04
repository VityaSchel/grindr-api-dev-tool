<script lang="ts">
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { Button } from "$lib/components/ui/button";
	import CopyButton from "$lib/components/CopyButton.svelte";
	import AddAccountDialog from "./AddAccountDialog.svelte";
	import EditAccountDialog from "./EditAccountDialog.svelte";
	import { accounts } from "$lib/accounts.svelte";
	import type { AccountInfo } from "$lib/api";
	import UserCircleIcon from "phosphor-svelte/lib/UserCircleIcon";
	import CaretUpDownIcon from "phosphor-svelte/lib/CaretUpDownIcon";
	import CheckIcon from "phosphor-svelte/lib/CheckIcon";
	import PencilSimpleIcon from "phosphor-svelte/lib/PencilSimpleIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";
	import PlusIcon from "phosphor-svelte/lib/PlusIcon";

	let menuOpen = $state(false);
	let addOpen = $state(false);
	let editOpen = $state(false);
	let editing = $state<AccountInfo | null>(null);
	let busy = $state(false);

	const label = $derived(accounts.active?.email ?? "Unauthorized");

	async function switchTo(id: string | null) {
		if (busy) return;
		busy = true;
		try {
			await accounts.setActive(id);
		} catch (e) {
			console.error("failed to switch account", e);
		} finally {
			busy = false;
		}
	}

	async function remove(id: string) {
		if (busy) return;
		busy = true;
		try {
			await accounts.remove(id);
		} catch (e) {
			console.error("failed to delete account", e);
		} finally {
			busy = false;
		}
	}

	function openAfterMenuCloses(show: () => void) {
		menuOpen = false;
		setTimeout(show, 0);
	}
</script>

<div class="flex items-center gap-2">
	{#if accounts.active}
		<CopyButton
			value={accounts.active.profile_id}
			variant="outline"
			size="sm"
			class="gap-1.5"
			title="Copy profile ID ({accounts.active.profile_id})"
		>
			ID
		</CopyButton>
	{/if}

	<DropdownMenu.Root bind:open={menuOpen}>
		<DropdownMenu.Trigger>
			{#snippet child({ props })}
				<Button
					variant="outline"
					size="sm"
					class="gap-2"
					disabled={busy}
					{...props}
				>
					<UserCircleIcon class="size-4" />
					<span
						class="max-w-44 truncate"
						class:text-muted-foreground={!accounts.active}
					>
						{label}
					</span>
					<CaretUpDownIcon class="size-3.5 text-muted-foreground" />
				</Button>
			{/snippet}
		</DropdownMenu.Trigger>
		<DropdownMenu.Content align="end" class="w-64">
			<DropdownMenu.Label>Accounts</DropdownMenu.Label>
			<DropdownMenu.Separator />

			<DropdownMenu.Item class="gap-2" onSelect={() => switchTo(null)}>
				{#if accounts.activeId === null}
					<CheckIcon class="size-4 shrink-0" />
				{:else}
					<span class="size-4 shrink-0"></span>
				{/if}
				<span class="flex-1">Unauthorized</span>
			</DropdownMenu.Item>

			{#each accounts.accounts as account (account.id)}
				<DropdownMenu.Item class="gap-2" onSelect={() => switchTo(account.id)}>
					{#if accounts.activeId === account.id}
						<CheckIcon class="size-4 shrink-0" />
					{:else}
						<span class="size-4 shrink-0"></span>
					{/if}
					<span class="min-w-0 flex-1 truncate" title={account.email}>
						{account.email}
					</span>
					<button
						type="button"
						class="rounded p-1 text-muted-foreground hover:bg-muted hover:text-foreground"
						aria-label="Edit account"
						onpointerdown={(e) => e.stopPropagation()}
						onpointerup={(e) => e.stopPropagation()}
						onclick={(e) => {
							e.stopPropagation();
							e.preventDefault();
							editing = account;
							openAfterMenuCloses(() => (editOpen = true));
						}}
					>
						<PencilSimpleIcon class="size-3.5" />
					</button>
					<button
						type="button"
						class="-mr-1 rounded p-1 text-muted-foreground hover:bg-destructive/10 hover:text-destructive"
						aria-label="Delete account"
						onpointerdown={(e) => e.stopPropagation()}
						onpointerup={(e) => e.stopPropagation()}
						onclick={(e) => {
							e.stopPropagation();
							e.preventDefault();
							void remove(account.id);
						}}
					>
						<XIcon class="size-3.5" />
					</button>
				</DropdownMenu.Item>
			{/each}

			<DropdownMenu.Separator />
			<DropdownMenu.Item
				class="gap-2"
				onSelect={() => openAfterMenuCloses(() => (addOpen = true))}
			>
				<PlusIcon class="size-4 shrink-0" />
				<span class="flex-1">Add account</span>
			</DropdownMenu.Item>
		</DropdownMenu.Content>
	</DropdownMenu.Root>
</div>

<AddAccountDialog bind:open={addOpen} />
<EditAccountDialog bind:open={editOpen} account={editing} />
