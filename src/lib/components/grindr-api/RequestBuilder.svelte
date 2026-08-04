<script lang="ts">
	import { onMount, untrack, type Snippet } from "svelte";
	import type { Operation, Param } from "$lib/openapi";
	import { getParamGroupsForTag, resolveGroupParams } from "$lib/openapi";
	import { accounts } from "$lib/accounts.svelte";
	import { api, type BodyFile } from "$lib/api";
	import { methodColor } from "$lib/methods";
	import { API_BASE_URL, grindrApiHref } from "$lib/links";
	import { buildRequestPath, parseRequestPath } from "$lib/request-path";
	import { RequestBodyModel } from "$lib/request-body.svelte";
	import type { RequestRunner } from "$lib/request.svelte";
	import ParamsForm from "./ParamsForm.svelte";
	import BodyEditor from "./BodyEditor.svelte";
	import * as Tabs from "$lib/components/ui/tabs";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import PaperPlaneTiltIcon from "phosphor-svelte/lib/PaperPlaneTiltIcon";
	import LockIcon from "phosphor-svelte/lib/LockIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";

	let {
		path,
		op,
		runner,
		docs,
	}: {
		path: string;
		op: Operation;
		runner: RequestRunner;
		docs: Snippet;
	} = $props();

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
		const merged = op.parameters.filter((p) => p.in === "query");
		for (const p of groupParams) {
			if (!merged.some((m) => m.name === p.name)) merged.push(p);
		}
		return merged;
	});

	const pathModel = $state<Record<string, unknown>>({});
	const queryModel = $state<Record<string, unknown>>({});
	let unknownQuery = $state<string[]>([]);
	const body = $derived(
		op.requestBody ? new RequestBodyModel(op.requestBody, path) : null,
	);

	let tab = $state("params");
	let rootEl = $state<HTMLElement | null>(null);

	const previewPath = $derived.by(() => {
		try {
			return buildRequestPath(
				path,
				pathParams,
				pathModel,
				queryParams,
				queryModel,
				unknownQuery,
			);
		} catch {
			return path;
		}
	});

	let urlHasFocus = $state(false);
	let urlText = $state("");

	function requestPathOf(text: string): string | null {
		try {
			const url = new URL(text, API_BASE_URL);
			return url.pathname + url.search;
		} catch {
			return null;
		}
	}

	$effect(() => {
		const canonical = API_BASE_URL + previewPath;
		if (urlHasFocus) return;
		if (untrack(() => urlText) !== canonical) urlText = canonical;
	});

	function syncModelsFromUrl(e: Event & { currentTarget: HTMLInputElement }) {
		const requestPath = requestPathOf(e.currentTarget.value);
		if (requestPath === null) return;
		const parsed = parseRequestPath(path, requestPath, pathParams, queryParams);
		if (!parsed) return;
		Object.assign(pathModel, parsed.path);
		for (const [name, value] of Object.entries(parsed.query)) {
			if (value === undefined) delete queryModel[name];
			else queryModel[name] = value;
		}
		unknownQuery = parsed.unknownQuery;
	}

	const blocked = $derived(!!op.security?.length && accounts.activeId === null);

	function send() {
		if (runner.sending || blocked) return;
		const requestPath = requestPathOf(urlText) ?? previewPath;
		let json: unknown | null = null;
		let file: BodyFile | null = null;
		try {
			if (body?.binaryContentType) file = body.toFile();
			else json = body?.toJson() ?? null;
		} catch (e) {
			runner.fail(String(e));
			return;
		}
		void runner.run((id) =>
			api.sendRequest(op.method.toUpperCase(), requestPath, json, id, file),
		);
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

<Tabs.Root bind:ref={rootEl} bind:value={tab} class="gap-0">
	<div class="sticky top-0 z-10 border-b bg-background">
		<div class="flex flex-col gap-3 px-6 pt-4 pb-3">
			{#if op.tags?.length}
				<div class="flex flex-wrap gap-1.5">
					{#each op.tags as tag (tag)}
						<a
							href={grindrApiHref(tag)}
							class="rounded-full bg-muted px-2 py-0.5 font-mono text-xs text-muted-foreground hover:bg-muted/80 hover:text-foreground"
						>
							{tag}
						</a>
					{/each}
				</div>
			{/if}

			<div class="flex items-center gap-2">
				<span
					class="shrink-0 rounded-lg border px-3 py-1.5 font-mono text-sm font-bold select-none"
					style="color: {methodColor(op.method)}"
				>
					{op.method.toUpperCase()}
				</span>

				<Input
					bind:value={urlText}
					oninput={syncModelsFromUrl}
					onfocus={() => (urlHasFocus = true)}
					onblur={() => (urlHasFocus = false)}
					spellcheck={false}
					aria-label="Request URL"
					title={urlText}
					class="min-w-0 flex-1 font-mono text-sm"
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
					<Button onclick={send} disabled={blocked} class="shrink-0">
						{#if blocked}<LockIcon />{:else}<PaperPlaneTiltIcon />{/if}
						Send
					</Button>
				{/if}
			</div>

			{#if blocked}
				<p class="text-xs text-muted-foreground">
					This endpoint requires authorization. Select an account to send it.
				</p>
			{/if}
		</div>

		<div class="border-t bg-muted/20 px-6 py-2">
			<Tabs.List>
				<Tabs.Trigger value="params">
					Params
					{#if pathParams.length + queryParams.length > 0}
						<span class="text-muted-foreground">
							({pathParams.length + queryParams.length})
						</span>
					{/if}
				</Tabs.Trigger>
				{#if body}
					<Tabs.Trigger value="body">Body</Tabs.Trigger>
				{/if}
				<Tabs.Trigger value="docs">Docs</Tabs.Trigger>
			</Tabs.List>
		</div>
	</div>

	<div class="px-6 pt-4 pb-6 select-text">
		<Tabs.Content value="params" class="flex flex-col gap-5">
			<ParamsForm {pathParams} {queryParams} {pathModel} {queryModel} />
		</Tabs.Content>
		{#if body}
			<Tabs.Content value="body" class="flex flex-col gap-3">
				<BodyEditor {body} />
			</Tabs.Content>
		{/if}
		<Tabs.Content value="docs">
			{@render docs()}
		</Tabs.Content>
	</div>
</Tabs.Root>
