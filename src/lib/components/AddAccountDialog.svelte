<script lang="ts">
	import * as Dialog from "$lib/components/ui/dialog";
	import * as Accordion from "$lib/components/ui/accordion";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Button } from "$lib/components/ui/button";
	import { accounts } from "$lib/accounts.svelte";
	import { api, DEVICE_FIELD_LABELS, type DeviceInfo } from "$lib/api";
	import ArrowsClockwiseIcon from "phosphor-svelte/lib/ArrowsClockwiseIcon";
	import { resolve } from "$app/paths";

	let { open = $bindable(false) }: { open?: boolean } = $props();

	let email = $state("");
	let authToken = $state("");
	let device = $state<DeviceInfo | null>(null);
	let loadingDevice = $state(false);
	let submitting = $state(false);
	let error = $state<string | null>(null);

	const deviceKeys = Object.keys(DEVICE_FIELD_LABELS) as (keyof DeviceInfo)[];

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
			email = "";
			authToken = "";
			error = null;
			device = null;
			void regenerateDevice();
		}
	});

	async function submit(e: SubmitEvent) {
		e.preventDefault();
		if (submitting) return;
		error = null;
		const normalizedEmail = email.trim().toLowerCase();
		if (
			accounts.accounts.some((a) => a.email.toLowerCase() === normalizedEmail)
		) {
			error = "This account is already added.";
			return;
		}
		submitting = true;
		try {
			await accounts.add(email.trim(), authToken.trim(), device);
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
				Sign in with your email and long-lived auth token. The session is then
				refreshed automatically.
			</Dialog.Description>
		</Dialog.Header>

		<form onsubmit={submit} class="flex flex-col gap-4">
			<div class="flex flex-col gap-1.5">
				<Label for="add-email">Email</Label>
				<Input
					id="add-email"
					type="email"
					bind:value={email}
					placeholder="you@example.com"
					autocomplete="off"
					required
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
					required
				/>
			</div>
			<a
				href={resolve("/grindr-api/v8/sessions")}
				class="w-fit font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
				onclick={() => (open = false)}
			>
				&gt; Sign in with email & password
			</a>

			<Accordion.Root type="single" class="rounded-lg border px-3">
				<Accordion.Item value="advanced" class="border-b-0">
					<Accordion.Trigger>Advanced — device parameters</Accordion.Trigger>
					<Accordion.Content>
						<div class="flex items-center justify-between pb-2">
							<p class="text-xs text-muted-foreground">
								Auto-generated. Override if needed.
							</p>
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
							<p class="text-xs text-muted-foreground">Generating device...</p>
						{/if}
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
				<Button type="submit" disabled={submitting || !email || !authToken}>
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
