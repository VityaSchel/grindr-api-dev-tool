<script lang="ts">
	import * as Tabs from "$lib/components/ui/tabs";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { resolve } from "$app/paths";
	import { openUrl } from "@tauri-apps/plugin-opener";
	import ArrowSquareOutIcon from "phosphor-svelte/lib/ArrowSquareOutIcon";
	import type { SignInForm } from "./sign-in";

	const GOOGLE_OAUTH_EXTENSION_URL =
		"https://git.opengrind.org/open-grind/grindr-google-oauth-webextension";

	let {
		form,
		onNavigate,
		onError,
	}: {
		form: SignInForm;
		onNavigate: () => void;
		onError: (message: string) => void;
	} = $props();
</script>

<Tabs.Root bind:value={form.method}>
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
				bind:value={form.email}
				placeholder="you@example.com"
				autocomplete="off"
			/>
		</div>
		<div class="flex flex-col gap-1.5">
			<Label for="add-token">Auth token</Label>
			<Input
				id="add-token"
				type="password"
				bind:value={form.authToken}
				placeholder="long-lived auth token"
				autocomplete="off"
			/>
		</div>
		<a
			href={resolve("/grindr-api/v8/sessions")}
			class="w-fit font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
			onclick={onNavigate}
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
				bind:value={form.email}
				placeholder="you@example.com"
				autocomplete="off"
			/>
		</div>
		<div class="flex flex-col gap-1.5">
			<Label for="add-password">Password</Label>
			<Input
				id="add-password"
				type="password"
				bind:value={form.password}
				placeholder="account password"
				autocomplete="off"
			/>
		</div>
		<p class="text-xs text-muted-foreground select-text">
			A shortcut for the same <code>/v8/sessions</code> request — the auth token is
			fetched for you.
		</p>
	</Tabs.Content>

	<Tabs.Content value="google" class="flex flex-col gap-4 pt-2">
		<div class="flex flex-col gap-1.5">
			<Label for="add-google-token">Google access token</Label>
			<Input
				id="add-google-token"
				type="password"
				bind:value={form.googleToken}
				placeholder="ya29..."
				autocomplete="off"
			/>
		</div>
		<button
			type="button"
			class="flex w-fit items-center gap-1 font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
			onclick={() =>
				openUrl(GOOGLE_OAUTH_EXTENSION_URL).catch((e) => onError(String(e)))}
		>
			<ArrowSquareOutIcon />
			Get a token with the browser extension
		</button>
	</Tabs.Content>
</Tabs.Root>
