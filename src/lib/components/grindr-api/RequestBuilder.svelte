<script lang="ts">
	import type { Operation, Param, SchemaObject } from "$lib/openapi";
	import { getParamGroupsForTag, resolveGroupParams } from "$lib/openapi";
	import { accounts } from "$lib/accounts.svelte";
	import { api } from "$lib/api";
	import { METHOD_COLORS, methodColor } from "$lib/methods";
	import SchemaField from "./SchemaField.svelte";
	import JsonView from "./JsonView.svelte";
	import * as Select from "$lib/components/ui/select";
	import * as Tabs from "$lib/components/ui/tabs";
	import { Switch } from "$lib/components/ui/switch";
	import { Textarea } from "$lib/components/ui/textarea";
	import { Button } from "$lib/components/ui/button";
	import { Separator } from "$lib/components/ui/separator";
	import PaperPlaneTiltIcon from "phosphor-svelte/lib/PaperPlaneTiltIcon";
	import LockIcon from "phosphor-svelte/lib/LockIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";

	let { path, operations }: { path: string; operations: Operation[] } =
		$props();

	const BASE_URL = "https://grindr.mobi";

	let selectedMethod = $state<string | null>(null);
	const op = $derived(
		operations.find((o) => o.method === selectedMethod) ?? operations[0],
	);
	const opColor = $derived(methodColor(op.method));

	// ── Parameters ──
	const pathParams = $derived(op.parameters.filter((p) => p.in === "path"));
	const groupParams = $derived.by(() => {
		const names = op["x-query-groups"] ?? [];
		if (!names.length) return [] as Param[];
		const out: Param[] = [];
		for (const tag of op.tags ?? []) {
			for (const { name, group } of getParamGroupsForTag(tag)) {
				if (names.includes(name)) out.push(...resolveGroupParams(group));
			}
		}
		return out;
	});
	const queryParams = $derived.by(() => {
		const own = op.parameters.filter((p) => p.in === "query");
		const merged = [...own];
		for (const p of groupParams) {
			if (!merged.some((m) => m.name === p.name)) merged.push(p);
		}
		return merged;
	});

	// ── Body ──
	const jsonSchema = $derived(
		op.requestBody?.content?.["application/json"]?.schema as
			| SchemaObject
			| undefined,
	);
	const contentType = $derived(
		op.requestBody ? Object.keys(op.requestBody.content)[0] : undefined,
	);

	// ── Models (reset when the operation changes) ──
	let pathModel = $state<Record<string, unknown>>({});
	let queryModel = $state<Record<string, unknown>>({});
	let bodyModel = $state<unknown>(undefined);
	let bodyView = $state<"form" | "json">("form");
	let bodyText = $state("");
	let bodyError = $state<string | null>(null);
	let tab = $state("params");

	let opKey = "";
	$effect(() => {
		const key = `${path}:${op.method}`;
		if (key !== opKey) {
			opKey = key;
			pathModel = {};
			queryModel = {};
			bodyModel = undefined;
			bodyView = "form";
			bodyError = null;
			tab = "params";
		}
	});

	// The form is the source of truth; mirror it into the editor while in form view.
	const builtJson = $derived(
		bodyModel === undefined ? "" : JSON.stringify(bodyModel, null, 2),
	);
	$effect(() => {
		if (bodyView === "form") bodyText = builtJson;
	});

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

	// Switch view. JSON→Form syncs the edited JSON back into the form, but only if
	// it parses; otherwise we stay on JSON and surface the error.
	function setView(toJson: boolean) {
		if (toJson) {
			bodyView = "json";
			return;
		}
		const t = bodyText.trim();
		if (t === "") {
			bodyModel = undefined;
			bodyError = null;
			bodyView = "form";
			return;
		}
		try {
			bodyModel = JSON.parse(t);
			bodyError = null;
			bodyView = "form";
		} catch (err) {
			bodyError = `Cannot switch to form — invalid JSON: ${err}`;
		}
	}

	function buildPath(): string {
		let p = path;
		for (const param of pathParams) {
			const val = pathModel[param.name];
			p = p.replaceAll(
				`{${param.name}}`,
				encodeURIComponent(String(val ?? "")),
			);
		}
		const pairs: string[] = [];
		const enc = (name: string, x: unknown) =>
			`${encodeURIComponent(name)}=${encodeURIComponent(String(x))}`;
		for (const param of queryParams) {
			const val = queryModel[param.name];
			if (val === undefined || val === null || val === "") continue;
			if (Array.isArray(val)) {
				for (const item of val)
					if (item !== undefined && item !== null && item !== "")
						pairs.push(enc(param.name, item));
			} else if (typeof val === "object") {
				if (Object.keys(val).length)
					pairs.push(enc(param.name, JSON.stringify(val)));
			} else {
				pairs.push(enc(param.name, val));
			}
		}
		const q = pairs.join("&");
		return q ? `${p}?${q}` : p;
	}

	const previewPath = $derived.by(() => {
		try {
			return buildPath();
		} catch {
			return path;
		}
	});

	function bodyToSend(): unknown | null {
		if (!op.requestBody) return null;
		const t = bodyText.trim();
		if (t === "") return null;
		if (!jsonSchema) return t; // non-JSON content type: best-effort raw text
		let parsed: unknown;
		try {
			parsed = JSON.parse(t);
		} catch {
			throw new Error("Request body is not valid JSON");
		}
		if (
			parsed !== null &&
			typeof parsed === "object" &&
			!Array.isArray(parsed) &&
			Object.keys(parsed).length === 0 &&
			!op.requestBody.required
		) {
			return null;
		}
		return parsed;
	}

	// ── Auth gating ──
	const requiresAuth = $derived(!!op.security?.length);
	const blocked = $derived(requiresAuth && accounts.activeId === null);

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
		if (sending || blocked) return;
		sending = true;
		cancelled = false;
		sendError = null;
		response = null;
		elapsed = null;
		tab = "response";
		const id = crypto.randomUUID();
		currentRequestId = id;
		const start = performance.now();
		try {
			const body = bodyToSend();
			response = await api.sendRequest(
				op.method.toUpperCase(),
				buildPath(),
				body,
				id,
			);
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
</script>

<div class="flex flex-col gap-4">
	<div class="flex items-center gap-2">
		{#if operations.length > 1}
			<Select.Root
				type="single"
				value={op.method}
				onValueChange={(v) => (selectedMethod = v)}
			>
				<Select.Trigger
					class="w-24 shrink-0 font-mono font-bold"
					style="color: {opColor}"
				>
					{op.method.toUpperCase()}
				</Select.Trigger>
				<Select.Content>
					{#each operations as o (o.method)}
						<Select.Item
							value={o.method}
							style="color: {METHOD_COLORS[o.method]}"
							class="font-mono font-bold"
						>
							{o.method.toUpperCase()}
						</Select.Item>
					{/each}
				</Select.Content>
			</Select.Root>
		{:else}
			<span
				class="shrink-0 rounded-lg border px-3 py-1.5 font-mono text-sm font-bold"
				style="color: {opColor}">{op.method.toUpperCase()}</span
			>
		{/if}

		<div
			class="flex-1 truncate rounded-lg border bg-muted/40 px-3 py-1.5 font-mono text-sm text-foreground"
			title={BASE_URL + previewPath}
		>
			<span class="text-muted-foreground">{BASE_URL}</span>{previewPath}
		</div>

		{#if sending}
			<Button variant="destructive" onclick={cancel} class="shrink-0">
				<XIcon /> Cancel
			</Button>
		{:else}
			<Button onclick={send} disabled={blocked} class="shrink-0">
				{#if blocked}<LockIcon />{:else}<PaperPlaneTiltIcon />{/if}
				Send
			</Button>
		{/if}
	</div>

	{#if blocked}
		<p class="-mt-2 text-xs text-muted-foreground">
			This endpoint requires authorization. Select an account to send it.
		</p>
	{/if}

	<Tabs.Root bind:value={tab}>
		<Tabs.List>
			<Tabs.Trigger value="params">
				Params
				{#if pathParams.length + queryParams.length > 0}
					<span class="text-muted-foreground">
						({pathParams.length + queryParams.length})
					</span>
				{/if}
			</Tabs.Trigger>
			{#if op.requestBody}
				<Tabs.Trigger value="body">Body</Tabs.Trigger>
			{/if}
			<Tabs.Trigger value="response">Response</Tabs.Trigger>
		</Tabs.List>

		<Tabs.Content value="params" class="flex flex-col gap-5 pt-2">
			{#if pathParams.length}
				<section class="flex flex-col gap-3">
					<h3 class="text-xs font-semibold tracking-wide uppercase">
						Path parameters
					</h3>
					{#each pathParams as param (param.name)}
						<SchemaField
							schema={param.schema ?? { type: "string" }}
							label={param.name}
							description={param.description}
							required={param.required}
							value={pathModel[param.name]}
							setValue={(v) => (pathModel[param.name] = v)}
						/>
					{/each}
				</section>
			{/if}
			{#if queryParams.length}
				<section class="flex flex-col gap-3">
					<h3 class="text-xs font-semibold tracking-wide uppercase">
						Query parameters
					</h3>
					{#each queryParams as param (param.name)}
						<SchemaField
							schema={param.schema ?? { type: "string" }}
							label={param.name}
							description={param.description}
							required={param.required}
							value={queryModel[param.name]}
							setValue={(v) => (queryModel[param.name] = v)}
						/>
					{/each}
				</section>
			{/if}
			{#if pathParams.length + queryParams.length === 0}
				<p class="text-sm text-muted-foreground">
					This endpoint takes no parameters.
				</p>
			{/if}
		</Tabs.Content>

		{#if op.requestBody}
			<Tabs.Content value="body" class="flex flex-col gap-3 pt-2">
				<div class="flex items-center justify-between">
					<span class="font-mono text-xs text-muted-foreground"
						>{contentType}</span
					>
					{#if jsonSchema}
						<div class="flex items-center gap-2 text-xs">
							<span
								class={bodyView === "form"
									? "font-medium"
									: "text-muted-foreground"}>Form</span
							>
							<Switch
								bind:checked={() => bodyView === "json", (v) => setView(v)}
								aria-label="Toggle JSON editor"
							/>
							<span
								class={bodyView === "json"
									? "font-medium"
									: "text-muted-foreground"}>JSON</span
							>
						</div>
					{/if}
				</div>

				{#if jsonSchema && bodyView === "form"}
					<div class="rounded-lg border p-3">
						<SchemaField
							schema={jsonSchema}
							value={bodyModel}
							setValue={(v) => (bodyModel = v)}
						/>
					</div>
				{:else}
					<Textarea
						value={bodyText}
						oninput={onBodyInput}
						spellcheck={false}
						class="min-h-48 font-mono text-xs"
						placeholder={jsonSchema
							? "JSON request body"
							: `Raw ${contentType} body`}
					/>
					{#if bodyError}
						<span class="text-xs text-destructive">{bodyError}</span>
					{/if}
					{#if !jsonSchema}
						<span class="text-xs text-muted-foreground">
							Typed form is only available for application/json bodies.
						</span>
					{/if}
				{/if}
			</Tabs.Content>
		{/if}

		<Tabs.Content value="response" class="flex flex-col gap-2 pt-2">
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
					<p class="text-xs text-muted-foreground italic">
						Empty response body.
					</p>
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
		</Tabs.Content>
	</Tabs.Root>
</div>
