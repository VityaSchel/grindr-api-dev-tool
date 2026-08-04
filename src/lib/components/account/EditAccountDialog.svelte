<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import { Button } from "$lib/components/ui/button";
	import { Label } from "$lib/components/ui/label";
	import { Skeleton } from "$lib/components/ui/skeleton";
	import CopyButton from "$lib/components/CopyButton.svelte";
	import DeviceFields from "./DeviceFields.svelte";
	import {
		api,
		type AccountInfo,
		type DeviceInfo,
		type Session,
		type SessionRestriction,
	} from "$lib/api";
	import EyeIcon from "phosphor-svelte/lib/EyeIcon";
	import EyeSlashIcon from "phosphor-svelte/lib/EyeSlashIcon";

	let {
		open = $bindable(false),
		account,
	}: { open?: boolean; account: AccountInfo | null } = $props();

	let session = $state<Session | null>(null);
	let device = $state<DeviceInfo | null>(null);
	let revealed = $state(false);
	let loadingDevice = $state(false);
	let saving = $state(false);
	let error = $state<string | null>(null);

	let loadSeq = 0;
	$effect(() => {
		const id = open ? account?.id : undefined;
		loadSeq += 1;
		session = null;
		device = null;
		revealed = false;
		error = null;
		if (id) void load(id, loadSeq);
	});

	async function load(id: string, seq: number) {
		try {
			const details = await api.accountDetails(id);
			if (seq !== loadSeq) return;
			session = details.session;
			device = details.device;
		} catch (e) {
			if (seq === loadSeq) error = String(e);
		}
	}

	async function regenerateDevice() {
		loadingDevice = true;
		try {
			device = await api.generateDevice();
		} catch (e) {
			error = String(e);
		} finally {
			loadingDevice = false;
		}
	}

	function expiry(seconds: number): string {
		if (!seconds) return "—";
		const at = new Date(seconds * 1000);
		return at.getTime() < Date.now()
			? `${at.toLocaleString()} (expired)`
			: at.toLocaleString();
	}

	function restriction(value: SessionRestriction): string {
		return typeof value === "string" ? value : JSON.stringify(value);
	}

	const PLACEHOLDER_ROWS = [
		"Profile ID",
		"Email",
		"Sign-in",
		"Expires",
		"Session ID",
		"Auth token",
	].map((label) => ({ label, value: "", secret: false }));

	const rows = $derived(
		session
			? [
					{ label: "Profile ID", value: session.profile_id, secret: false },
					{ label: "Email", value: session.email, secret: false },
					{ label: "Sign-in", value: session.kind, secret: false },
					{
						label: "Expires",
						value: expiry(session.expires_at),
						secret: false,
					},
					...(session.restriction
						? [
								{
									label: "Restriction",
									value: restriction(session.restriction),
									secret: false,
								},
							]
						: []),
					{ label: "Session ID", value: session.session_id, secret: true },
					{ label: "Auth token", value: session.auth_token, secret: true },
				]
			: PLACEHOLDER_ROWS,
	);

	async function save() {
		if (!account || !device || saving) return;
		saving = true;
		error = null;
		try {
			await api.updateAccountDevice(account.id, device);
			open = false;
		} catch (e) {
			error = String(e);
		} finally {
			saving = false;
		}
	}
</script>

<Dialog.Root bind:open>
	<Dialog.Content class="max-h-[85vh] max-w-md overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Edit account</Dialog.Title>
			<Dialog.Description>
				{account?.email ?? ""}
			</Dialog.Description>
		</Dialog.Header>

		<div class="flex min-w-0 flex-col gap-2 rounded-lg border p-3">
			<div class="flex items-center justify-between">
				<Label class="text-xs font-medium">Session</Label>
				<Button
					variant="ghost"
					size="xs"
					onclick={() => (revealed = !revealed)}
					disabled={!session}
				>
					{#if revealed}
						<EyeSlashIcon /> Hide secrets
					{:else}
						<EyeIcon /> Show secrets
					{/if}
				</Button>
			</div>
			{#each rows as row (row.label)}
				<div class="flex h-6 items-center gap-2">
					<span class="w-20 shrink-0 text-xs text-muted-foreground">
						{row.label}
					</span>
					{#if session}
						<span class="min-w-0 flex-1 truncate font-mono text-xs select-text">
							{row.secret && !revealed ? "••••••••••••" : row.value}
						</span>
						<CopyButton
							value={row.value}
							variant="ghost"
							size="icon-xs"
							title="Copy {row.label.toLowerCase()}"
						/>
					{:else}
						<Skeleton class="h-3 flex-1" />
						<span class="size-6 shrink-0"></span>
					{/if}
				</div>
			{/each}
		</div>

		<DeviceFields
			bind:device
			description="Applied to the live session — the account stays signed in."
			busy={loadingDevice}
			onRegenerate={regenerateDevice}
		/>

		{#if error}
			<p
				class="rounded-md bg-destructive/10 px-3 py-2 text-xs wrap-break-word text-destructive select-text"
			>
				{error}
			</p>
		{/if}

		<Dialog.Footer>
			<Button variant="ghost" onclick={() => (open = false)}>Cancel</Button>
			<Button onclick={save} disabled={saving || !device}>
				{#if saving}
					Saving...
				{:else}
					Save
				{/if}
			</Button>
		</Dialog.Footer>
	</Dialog.Content>
</Dialog.Root>
