<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Accordion from "$lib/components/ui/accordion";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import { accounts } from "$lib/accounts.svelte";
	import { api, type DeviceInfo } from "$lib/api";
	import DeviceFields from "./DeviceFields.svelte";
	import SignInMethodTabs from "./SignInMethodTabs.svelte";
	import { emptySignInForm, toCredentials } from "./sign-in";

	let { open = $bindable(false) }: { open?: boolean } = $props();

	let form = $state(emptySignInForm());
	let geohash = $state("");
	let device = $state<DeviceInfo | null>(null);
	let loadingDevice = $state(false);
	let submitting = $state(false);
	let error = $state<string | null>(null);

	// The backend has no cancel: a started sign-in lands whatever the dialog does.
	const dismissable = $derived(!submitting);

	const credentials = $derived(toCredentials(form));

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

	$effect(() => {
		if (open) {
			form = emptySignInForm();
			geohash = "";
			error = null;
			device = null;
			submitting = false;
			void regenerateDevice();
		}
	});

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		if (submitting) return;
		error = null;

		const creds = credentials;
		if (!creds) return;

		// Google has no email to dedupe on; the backend upserts it by profile id.
		if (creds.method !== "google") {
			const normalizedEmail = creds.email.toLowerCase();
			if (
				accounts.accounts.some((a) => a.email.toLowerCase() === normalizedEmail)
			) {
				error = "This account is already added.";
				return;
			}
		}

		submitting = true;
		try {
			await accounts.add(creds, device, geohash.trim() || null);
			open = false;
		} catch (err) {
			error = String(err);
		} finally {
			submitting = false;
		}
	}
</script>

<Dialog.Root
	bind:open={
		() => open,
		(v) => {
			if (v || dismissable) open = v;
		}
	}
>
	<Dialog.Content class="max-h-[85vh] max-w-md overflow-y-auto">
		<Dialog.Header>
			<Dialog.Title>Add account</Dialog.Title>
			<Dialog.Description>
				Sign in to keep the session alive and refreshed automatically.
			</Dialog.Description>
		</Dialog.Header>

		<form onsubmit={submit} class="flex flex-col gap-4">
			<SignInMethodTabs
				{form}
				onNavigate={() => dismissable && (open = false)}
				onError={(message) => (error = message)}
			/>

			<Accordion.Root type="single" class="rounded-lg border px-3">
				<Accordion.Item value="advanced" class="border-b-0">
					<Accordion.Trigger>Advanced</Accordion.Trigger>
					<Accordion.Content class="flex flex-col gap-4">
						<div class="flex flex-col gap-1.5">
							<Label for="add-geohash" class="text-xs font-medium">
								Geohash
							</Label>
							<Input
								id="add-geohash"
								class="h-7 font-mono text-xs"
								bind:value={geohash}
								placeholder="e.g. 9q8yyk8yuv"
								autocomplete="off"
							/>
							<p class="text-xs text-muted-foreground">
								Sent only with the sign-in request. Leave empty to omit.
							</p>
						</div>

						<DeviceFields
							bind:device
							description="Auto-generated. Override if needed."
							busy={loadingDevice}
							onRegenerate={regenerateDevice}
						/>
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root>

			{#if error}
				<p
					class="rounded-md bg-destructive/10 px-3 py-2 text-xs wrap-break-word text-destructive select-text"
				>
					{error}
				</p>
			{/if}

			<Dialog.Footer>
				<Button
					type="button"
					variant="ghost"
					disabled={!dismissable}
					onclick={() => (open = false)}
				>
					Cancel
				</Button>
				<Button type="submit" disabled={submitting || !credentials}>
					{#if submitting}
						Authenticating...
					{:else}
						Add account
					{/if}
				</Button>
			</Dialog.Footer>
		</form>
	</Dialog.Content>
</Dialog.Root>
