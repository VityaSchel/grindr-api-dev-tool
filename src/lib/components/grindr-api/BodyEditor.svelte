<script lang="ts">
	import type { RequestBodyModel } from "$lib/request-body.svelte";
	import { formatFileSize } from "$lib/utils";
	import SchemaField from "./SchemaField.svelte";
	import { Switch } from "$lib/components/ui/switch";
	import { Textarea } from "$lib/components/ui/textarea";
	import { Input } from "$lib/components/ui/input";
	import { Button } from "$lib/components/ui/button";
	import FileArrowUpIcon from "phosphor-svelte/lib/FileArrowUpIcon";
	import XIcon from "phosphor-svelte/lib/XIcon";

	let { body }: { body: RequestBodyModel } = $props();
</script>

<div class="flex items-center justify-between">
	<span class="font-mono text-xs text-muted-foreground">
		{body.contentType}
	</span>
	{#if body.jsonSchema}
		<div class="flex items-center gap-2 text-xs">
			<span
				class={body.view === "form" ? "font-medium" : "text-muted-foreground"}
			>
				Form
			</span>
			<Switch
				bind:checked={
					() => body.view === "json", (v) => body.setView(v ? "json" : "form")
				}
				aria-label="Toggle JSON editor"
			/>
			<span
				class={body.view === "json" ? "font-medium" : "text-muted-foreground"}
			>
				JSON
			</span>
		</div>
	{/if}
</div>

{#if body.binaryContentType}
	<div class="flex flex-wrap items-center gap-2">
		<Button variant="outline" size="sm" onclick={() => body.pickFile()}>
			<FileArrowUpIcon /> Select file…
		</Button>
		{#if body.file}
			<div
				class="flex min-w-0 items-center gap-2 rounded-lg border bg-muted/40 px-3 py-1.5 text-xs"
			>
				<span class="truncate font-mono">{body.file.name}</span>
				<span class="shrink-0 text-muted-foreground">
					{formatFileSize(body.file.size)}
				</span>
				<button
					type="button"
					class="shrink-0 text-muted-foreground hover:text-foreground"
					onclick={() => (body.file = null)}
					aria-label="Remove file"
				>
					<XIcon class="size-3.5" />
				</button>
			</div>
		{:else}
			<span class="text-xs text-muted-foreground">No file selected.</span>
		{/if}
	</div>
	{#if body.fileError}
		<span class="text-xs wrap-break-word text-destructive">
			{body.fileError}
		</span>
	{/if}
	<label class="flex flex-col gap-1.5">
		<span class="text-xs font-semibold tracking-wide uppercase"
			>Content-Type</span
		>
		<Input
			bind:value={body.fileContentType}
			spellcheck={false}
			class="font-mono text-xs"
			placeholder={body.binaryContentType}
		/>
	</label>
	<span class="text-xs text-muted-foreground">
		The file is sent as the raw request body with this Content-Type.
	</span>
	<label class="flex items-center justify-between gap-2">
		<span class="text-xs font-semibold tracking-wide uppercase">
			Sign with device key
		</span>
		<Switch bind:checked={body.signed} />
	</label>
	<span class="text-xs text-muted-foreground">
		Required for <code>/v5/media/upload</code> and
		<code>/v6/chat/media/upload</code>: registers a P-256 key and adds the
		<code>X-Sig</code> signing headers.
	</span>
{:else if body.jsonSchema && body.view === "form"}
	<div class="rounded-lg border p-3">
		<SchemaField
			schema={body.jsonSchema}
			value={body.model}
			setValue={(v) => body.setModel(v)}
		/>
	</div>
{:else}
	<Textarea
		value={body.text}
		oninput={(e) => body.setText(e.currentTarget.value)}
		spellcheck={false}
		class="min-h-48 font-mono text-xs"
		placeholder={body.jsonSchema
			? "JSON request body"
			: `Raw ${body.contentType} body`}
	/>
	{#if body.error}
		<span class="text-xs text-destructive">{body.error}</span>
	{/if}
	{#if !body.jsonSchema}
		<span class="text-xs text-muted-foreground">
			Typed form is only available for application/json bodies.
		</span>
	{/if}
{/if}
