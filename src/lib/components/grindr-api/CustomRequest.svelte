<script lang="ts">
	import { onMount } from "svelte";
	import { accounts } from "$lib/accounts.svelte";
	import { api } from "$lib/api";
	import { SELECTABLE_METHODS, methodColor } from "$lib/methods";
	import JsonView from "./JsonView.svelte";
	import * as Select from "$lib/components/ui/select";
	import { Input } from "$lib/components/ui/input";
	import { Textarea } from "$lib/components/ui/textarea";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import PaperPlaneTiltIcon from "phosphor-svelte/lib/PaperPlaneTiltIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";

	const BASE_URL = "https://grindr.mobi";

	let rootEl = $state<HTMLElement | null>(null);
	let method = $state("GET");
	let url = $state("");
	let bodyText = $state("");
	let bodyError = $state<string | null>(null);

	const methodColorValue = $derived(methodColor(method));

	function normalizePath(input: string): string {
		let s = input.trim();
		if (/^https?:\/\//i.test(s)) {
			try {
				const u = new URL(s);
				s = u.pathname + u.search;
			} catch {
				/* fall through and treat as a raw path */
			}
		}
		if (s && !s.startsWith("/")) s = "/" + s;
		return s;
	}

	const previewPath = $derived(normalizePath(url));

	function onBodyInput(e: Event & { currentTarget: HTMLTextAreaElement }) {
		bodyText = e.currentTarget.value;
		if (bodyText.trim() === "") {
			bodyError = null;
			return;
		}
		try {
			JSON.parse(bodyText);
			bodyError = null;
		} catch (err) {
			bodyError = String(err);
		}
	}

	function bodyToSend(): unknown | null {
		const t = bodyText.trim();
		if (t === "") return null;
		try {
			return JSON.parse(t);
		} catch {
			throw new Error("Request body is not valid JSON");
		}
	}

	// ── Sending ──
	let sending = $state(false);
	let cancelled = $state(false);
	let currentRequestId = "";
	let sendError = $state<string | null>(null);
	let response = $state<{ status: number; body: string } | null>(null);
	let elapsed = $state<number | null>(null);

	const parsedResponse = $derived.by(() => {
		if (!response) return { ok: false, value: null as unknown };
		try {
			return { ok: true, value: JSON.parse(response.body) as unknown };
		} catch {
			return { ok: false, value: null as unknown };
		}
	});

	function statusClass(status: number): string {
		if (status >= 200 && status < 300)
			return "text-green-600 dark:text-green-400";
		if (status >= 400) return "text-destructive";
		return "text-muted-foreground";
	}

	async function send() {
		if (sending) return;
		const path = normalizePath(url);
		if (!path.startsWith("/")) {
			sendError = "Enter a path or URL (e.g. /v3/me/profile).";
			return;
		}
		sending = true;
		cancelled = false;
		sendError = null;
		response = null;
		elapsed = null;
		const id = crypto.randomUUID();
		currentRequestId = id;
		const start = performance.now();
		try {
			const body = bodyToSend();
			response = await api.sendRequest(method.toUpperCase(), path, body, id);
		} catch (e) {
			// A user-initiated cancel rejects too; show it as neutral, not an error.
			if (!cancelled) sendError = String(e);
		} finally {
			elapsed = Math.round(performance.now() - start);
			sending = false;
		}
	}

	function cancel() {
		if (!sending) return;
		cancelled = true;
		void api.cancelRequest(currentRequestId);
	}

	function onUrlKeydown(e: KeyboardEvent) {
		if (e.key === "Enter") {
			e.preventDefault();
			void send();
		}
	}

	onMount(() => {
		function onKeydown(e: KeyboardEvent) {
			if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
				if (!rootEl || rootEl.offsetParent === null) return;
				e.preventDefault();
				void send();
			}
		}
		window.addEventListener("keydown", onKeydown);
		return () => window.removeEventListener("keydown", onKeydown);
	});
</script>

<div bind:this={rootEl} class="mx-auto flex max-w-4xl flex-col gap-4 p-6">
	<div>
		<h1 class="text-xl font-bold">Custom request</h1>
		<p class="mt-1 text-sm text-muted-foreground">
			Send any method to any grindr.mobi path. Authentication headers and the
			session are filled in from the active account
			{#if accounts.active}
				(<span class="font-medium">{accounts.active.email}</span>).
			{:else}
				— currently <span class="font-medium">unauthorized</span>, so only
				no-auth endpoints will work.
			{/if}
		</p>
	</div>

	<div class="flex items-center gap-2">
		<Select.Root
			type="single"
			value={method}
			onValueChange={(v) => (method = v)}
		>
			<Select.Trigger
				class="w-28 shrink-0 font-mono font-bold"
				style="color: {methodColorValue}"
			>
				{method.toUpperCase()}
			</Select.Trigger>
			<Select.Content>
				{#each SELECTABLE_METHODS as m (m)}
					<Select.Item
						value={m.toUpperCase()}
						style="color: {methodColor(m)}"
						class="font-mono font-bold"
					>
						{m.toUpperCase()}
					</Select.Item>
				{/each}
			</Select.Content>
		</Select.Root>

		<Input
			bind:value={url}
			spellcheck={false}
			placeholder="/v3/me/profile"
			class="flex-1 font-mono text-sm"
			onkeydown={onUrlKeydown}
		/>

		{#if sending}
			<Button variant="destructive" onclick={cancel} class="shrink-0">
				<XIcon /> Cancel
			</Button>
		{:else}
			<Button onclick={send} class="shrink-0">
				<PaperPlaneTiltIcon /> Send
			</Button>
		{/if}
	</div>

	{#if previewPath}
		<div
			class="-mt-1 truncate font-mono text-xs text-muted-foreground"
			title={BASE_URL + previewPath}
		>
			{BASE_URL}{previewPath}
		</div>
	{/if}

	<div class="flex flex-col gap-2">
		<span class="text-xs font-semibold tracking-wide uppercase">Body</span>
		<Textarea
			value={bodyText}
			oninput={onBodyInput}
			spellcheck={false}
			class="min-h-40 font-mono text-xs"
			placeholder="JSON request body (leave empty for none)"
		/>
		{#if bodyError}
			<span class="text-xs text-destructive">{bodyError}</span>
		{/if}
	</div>

	<Separator />

	<div class="flex flex-col gap-2">
		<span class="text-xs font-semibold tracking-wide uppercase">Response</span>
		{#if sendError}
			<div
				class="rounded-lg border border-destructive/30 bg-destructive/10 p-3"
			>
				<p class="text-sm wrap-break-word text-destructive">{sendError}</p>
			</div>
		{:else if response}
			<div class="flex items-center gap-3">
				<span
					class="font-mono text-sm font-bold {statusClass(response.status)}"
				>
					{response.status}
				</span>
				{#if elapsed !== null}
					<span class="text-xs text-muted-foreground">{elapsed} ms</span>
				{/if}
			</div>
			<Separator />
			{#if response.body.trim() === ""}
				<p class="text-xs text-muted-foreground italic">Empty response body.</p>
			{:else if parsedResponse.ok}
				<JsonView json={parsedResponse.value} />
			{:else}
				<pre
					class="max-h-112 overflow-auto rounded-lg border bg-muted/40 p-3 font-mono text-xs whitespace-pre-wrap">{response.body}</pre>
			{/if}
		{:else if cancelled}
			<p class="text-sm text-muted-foreground">Request cancelled.</p>
		{:else}
			<p class="text-sm text-muted-foreground">
				{#if sending}Sending...{:else}No response yet. Hit Send.{/if}
			</p>
		{/if}
	</div>
</div>
