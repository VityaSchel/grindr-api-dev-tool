<script lang="ts">
	import { onMount } from "svelte";
	import { accounts } from "$lib/accounts.svelte";
	import { api } from "$lib/api";
	import { API_BASE_URL } from "$lib/links";
	import { SELECTABLE_METHODS, methodColor } from "$lib/methods";
	import { RequestRunner } from "$lib/request.svelte";
	import RequestShell from "./RequestShell.svelte";
	import ResponsePane from "./ResponsePane.svelte";
	import * as Select from "$lib/components/ui/select";
	import { Input } from "$lib/components/ui/input";
	import { Textarea } from "$lib/components/ui/textarea";
	import { Button } from "$lib/components/ui/button";
	import PaperPlaneTiltIcon from "phosphor-svelte/lib/PaperPlaneTiltIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";

	const runner = new RequestRunner();

	let rootEl = $state<HTMLElement | null>(null);
	let method = $state("GET");
	let url = $state("");
	let bodyText = $state("");
	let bodyError = $state<string | null>(null);

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

	function send() {
		if (runner.sending) return;
		const path = normalizePath(url);
		if (!path.startsWith("/")) {
			runner.fail("Enter a path or URL (e.g. /v3/me/profile).");
			return;
		}
		const text = bodyText.trim();
		let body: unknown | null = null;
		if (text !== "") {
			try {
				body = JSON.parse(text);
			} catch (e) {
				runner.fail(`Request body is not valid JSON — ${e}`);
				return;
			}
		}
		void runner.run((id) =>
			api.sendRequest(method.toUpperCase(), path, body, id),
		);
	}

	function onUrlKeydown(e: KeyboardEvent) {
		if (e.key === "Enter") {
			e.preventDefault();
			send();
		}
	}

	onMount(() => {
		function onKeydown(e: KeyboardEvent) {
			if ((e.metaKey || e.ctrlKey) && e.key === "Enter") {
				if (!rootEl || rootEl.offsetParent === null) return;
				e.preventDefault();
				send();
			}
		}
		window.addEventListener("keydown", onKeydown);
		return () => window.removeEventListener("keydown", onKeydown);
	});
</script>

<RequestShell showResponse={runner.started}>
	{#snippet request()}
		<div
			bind:this={rootEl}
			class="mx-auto flex max-w-4xl flex-col gap-4 px-6 pt-6 pb-4 select-text"
		>
			<div>
				<h1 class="text-xl font-bold">Custom request</h1>
				<p class="mt-1 text-sm text-muted-foreground">
					Send any method to any grindr.mobi path. Authentication headers and
					the session are filled in from the active account
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
						style="color: {methodColor(method)}"
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

				{#if runner.sending}
					<Button
						variant="destructive"
						onclick={() => runner.cancel()}
						class="shrink-0"
					>
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
					title={API_BASE_URL + previewPath}
				>
					{API_BASE_URL}{previewPath}
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
		</div>
	{/snippet}

	{#snippet response()}
		<ResponsePane {runner} />
	{/snippet}
</RequestShell>
