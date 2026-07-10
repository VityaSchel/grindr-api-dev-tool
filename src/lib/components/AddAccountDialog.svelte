<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Accordion from "$lib/components/ui/accordion";
	import * as Tabs from "$lib/components/ui/tabs";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import { accounts } from "$lib/accounts.svelte";
	import {
		api,
		DEVICE_FIELD_LABELS,
		type DeviceInfo,
		type SignInCredentials,
	} from "$lib/api";
	import ArrowsClockwiseIcon from "phosphor-svelte/lib/ArrowsClockwiseIcon";
	import ArrowSquareOutIcon from "phosphor-svelte/lib/ArrowSquareOutIcon";
	import { resolve } from "$app/paths";
	import { openUrl } from "@tauri-apps/plugin-opener";

	const GOOGLE_OAUTH_EXTENSION_URL =
		"https://git.opengrind.org/open-grind/grindr-google-oauth-webextension";

	let { open = $bindable(false) }: { open?: boolean } = $props();

	type Method = SignInCredentials["method"];

	let method = $state<Method>("token");
	let email = $state("");
	let authToken = $state("");
	let password = $state("");
	let googleToken = $state("");
	let geohash = $state("");
	let device = $state<DeviceInfo | null>(null);
	let loadingDevice = $state(false);
	let submitting = $state(false);
	let error = $state<string | null>(null);

	const deviceKeys = Object.keys(DEVICE_FIELD_LABELS) as (keyof DeviceInfo)[];

	const credentials = $derived.by<SignInCredentials | null>(() => {
		switch (method) {
			case "token": {
				const e = email.trim();
				const t = authToken.trim();
				return e && t ? { method: "token", email: e, authToken: t } : null;
			}
			case "password": {
				const e = email.trim();
				return e && password
					? { method: "password", email: e, password }
					: null;
			}
			case "google": {
				const t = googleToken.trim();
				return t ? { method: "google", googleToken: t } : null;
			}
		}
	});

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

	function setField(key: keyof DeviceInfo, value: string) {
		if (!device) return;
		device = {
			...device,
			[key]: key === "device_type" ? Number(value) || 0 : value,
		} as DeviceInfo;
	}

	$effect(() => {
		if (open) {
			method = "token";
			email = "";
			authToken = "";
			password = "";
			googleToken = "";
			geohash = "";
			error = null;
			device = null;
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

<Dialog.Root bind:open>
	<Dialog.Content class="max-w-md">
		<Dialog.Header>
			<Dialog.Title>Add account</Dialog.Title>
			<Dialog.Description>
				Sign in to keep the session alive and refreshed automatically.
			</Dialog.Description>
		</Dialog.Header>

		<form onsubmit={submit} class="flex flex-col gap-4">
			<Tabs.Root bind:value={method}>
				<Tabs.List class="grid w-full grid-cols-3">
					<Tabs.Trigger value="token">Token</Tabs.Trigger>
					<Tabs.Trigger value="password">Password</Tabs.Trigger>
					<Tabs.Trigger value="google">Google</Tabs.Trigger>
				</Tabs.List>

				<Tabs.Content value="token" class="flex flex-col gap-4 pt-2">
					<div class="flex flex-col gap-1.5">
						<Label for="add-email">Email</Label>
						<Input
							id="add-email"
							type="email"
							bind:value={email}
							placeholder="you@example.com"
							autocomplete="off"
						/>
					</div>
					<div class="flex flex-col gap-1.5">
						<Label for="add-token">Auth token</Label>
						<Input
							id="add-token"
							type="password"
							bind:value={authToken}
							placeholder="long-lived auth token"
							autocomplete="off"
						/>
					</div>
					<a
						href={resolve("/grindr-api/v8/sessions")}
						class="w-fit font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
						onclick={() => (open = false)}
					>
						&gt; Sign in via API request
					</a>
				</Tabs.Content>

				<Tabs.Content value="password" class="flex flex-col gap-4 pt-2">
					<div class="flex flex-col gap-1.5">
						<Label for="add-email-pw">Email</Label>
						<Input
							id="add-email-pw"
							type="email"
							bind:value={email}
							placeholder="you@example.com"
							autocomplete="off"
						/>
					</div>
					<div class="flex flex-col gap-1.5">
						<Label for="add-password">Password</Label>
						<Input
							id="add-password"
							type="password"
							bind:value={password}
							placeholder="account password"
							autocomplete="off"
						/>
					</div>
					<p class="text-xs text-muted-foreground">
						A shortcut for the same <code>/v8/sessions</code> request — the auth token
						is fetched for you.
					</p>
				</Tabs.Content>

				<Tabs.Content value="google" class="flex flex-col gap-4 pt-2">
					<div class="flex flex-col gap-1.5">
						<Label for="add-google-token">Google access token</Label>
						<Input
							id="add-google-token"
							type="password"
							bind:value={googleToken}
							placeholder="ya29..."
							autocomplete="off"
						/>
					</div>
					<button
						type="button"
						class="flex w-fit items-center gap-1 font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
						onclick={() =>
							openUrl(GOOGLE_OAUTH_EXTENSION_URL).catch(
								(err) => (error = String(err)),
							)}
					>
						<ArrowSquareOutIcon />
						Get a token with the browser extension
					</button>
				</Tabs.Content>
			</Tabs.Root>

			<Accordion.Root type="single" class="rounded-lg border px-3">
				<Accordion.Item value="advanced" class="border-b-0">
					<Accordion.Trigger>Advanced</Accordion.Trigger>
					<Accordion.Content class="flex flex-col gap-4">
						<div class="flex flex-col gap-1.5">
							<Label for="add-geohash" class="text-xs font-medium">
								Geohash</Label
							>
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

						<div class="flex flex-col gap-2">
							<div class="flex items-center justify-between">
								<Label class="text-xs font-medium">Device parameters</Label>
								<Button
									type="button"
									variant="outline"
									size="xs"
									onclick={regenerateDevice}
									disabled={loadingDevice}
								>
									<ArrowsClockwiseIcon />
									Regenerate
								</Button>
							</div>
							<p class="text-xs text-muted-foreground">
								Auto-generated. Override if needed.
							</p>
							{#if device}
								<div class="grid grid-cols-2 gap-2">
									{#each deviceKeys as key (key)}
										<div class="flex flex-col gap-1">
											<Label
												for={`dev-${key}`}
												class="text-xs text-muted-foreground"
											>
												{DEVICE_FIELD_LABELS[key]}
											</Label>
											<Input
												id={`dev-${key}`}
												class="h-7 font-mono text-xs"
												value={String(device[key])}
												oninput={(e) => setField(key, e.currentTarget.value)}
											/>
										</div>
									{/each}
								</div>
							{:else}
								<p class="text-xs text-muted-foreground">
									Generating device...
								</p>
							{/if}
						</div>
					</Accordion.Content>
				</Accordion.Item>
			</Accordion.Root>

			{#if error}
				<p
					class="rounded-md bg-destructive/10 px-3 py-2 text-xs wrap-break-word text-destructive"
				>
					{error}
				</p>
			{/if}

			<Dialog.Footer>
				<Button type="button" variant="ghost" onclick={() => (open = false)}>
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
