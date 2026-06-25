<script lang="ts">
	import * as DropdownMenu from "$lib/components/ui/dropdown-menu";
	import { Button } from "$lib/components/ui/button";
	import { goto } from "$app/navigation";
	import { accounts } from "$lib/accounts.svelte";
	import { grindrApiHref, CUSTOM_TAB_PATH } from "$lib/links";
	import AddAccountDialog from "./AddAccountDialog.svelte";
	import UserCircleIcon from "phosphor-svelte/lib/UserCircleIcon";
	import CaretUpDownIcon from "phosphor-svelte/lib/CaretUpDownIcon";
	import CheckIcon from "phosphor-svelte/lib/CheckIcon";
	import CopyIcon from "phosphor-svelte/lib/CopyIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";
	import PlusIcon from "phosphor-svelte/lib/PlusIcon";

	let addOpen = $state(false);
	let busy = $state(false);
	let copied = $state(false);
	let copyTimer: ReturnType<typeof setTimeout> | undefined;

	const label = $derived(accounts.active?.email ?? "Unauthorized");

	function copyProfileId() {
		const id = accounts.active?.profile_id;
		if (!id) return;
		void navigator.clipboard.writeText(id);
		copied = true;
		clearTimeout(copyTimer);
		copyTimer = setTimeout(() => (copied = false), 1500);
	}

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
</script>

<header
	class="flex h-12 shrink-0 items-center justify-between gap-2 border-b px-3"
	data-slot="navbar"
>
	<div class="flex min-w-0 items-center gap-4">
		<span class="truncate text-sm font-medium text-muted-foreground">
			Grindr API
		</span>
		<Button
			variant="outline"
			size="xs"
			class="gap-2"
			onclick={() => goto(grindrApiHref(CUSTOM_TAB_PATH))}
		>
			<PlusIcon />
			<span class="hidden sm:inline">Custom request</span>
		</Button>
	</div>

	<div class="flex items-center gap-2">
		{#if accounts.active}
			<Button
				variant="outline"
				size="sm"
				class="gap-1.5"
				onclick={copyProfileId}
				title="Copy profile ID ({accounts.active.profile_id})"
			>
				{#if copied}
					<CheckIcon class="size-3.5 text-green-600 dark:text-green-400" />
				{:else}
					<CopyIcon class="size-3.5" />
				{/if}
				ID
			</Button>
		{/if}

		<DropdownMenu.Root>
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
					<DropdownMenu.Item
						class="gap-2"
						onSelect={() => switchTo(account.id)}
					>
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
					onSelect={() => {
						setTimeout(() => (addOpen = true), 0);
					}}
				>
					<PlusIcon class="size-4 shrink-0" />
					<span class="flex-1">Add account</span>
				</DropdownMenu.Item>
			</DropdownMenu.Content>
		</DropdownMenu.Root>
	</div>
</header>

<AddAccountDialog bind:open={addOpen} />
