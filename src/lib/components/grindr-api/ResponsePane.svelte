<script lang="ts">
	import type { RequestRunner } from "$lib/request.svelte";
	import JsonView from "./JsonView.svelte";

	let { runner }: { runner: RequestRunner } = $props();

	const parsed = $derived.by(() => {
		const body = runner.response?.body;
		if (body === undefined) return null;
		try {
			return { json: JSON.parse(body) as unknown };
		} catch {
			return null;
		}
	});

	function statusClass(status: number): string {
		if (status >= 200 && status < 300)
			return "text-green-600 dark:text-green-400";
		if (status >= 400) return "text-destructive";
		return "text-muted-foreground";
	}
</script>

<section class="flex h-full min-h-0 flex-col select-text">
	<div class="flex shrink-0 items-center gap-3 px-6 pt-4 pb-3">
		<span
			class="text-xs font-semibold tracking-wide text-muted-foreground uppercase select-none"
		>
			Response
		</span>
		{#if runner.response}
			<span
				class="font-mono text-sm font-bold {statusClass(
					runner.response.status,
				)}"
			>
				{runner.response.status}
			</span>
		{/if}
		{#if runner.sending}
			<span class="text-xs text-muted-foreground">Sending...</span>
		{:else if runner.elapsed !== null}
			<span class="text-xs text-muted-foreground"
				>{runner.elapsed} ms</span
			>
		{/if}
	</div>

	<div
		class={[
			"min-h-0 flex-1 px-6 pb-6",
			{ "flex flex-col": parsed, "overflow-auto": !parsed },
		]}
	>
		{#if runner.error}
			<div
				class="rounded-lg border border-destructive/30 bg-destructive/10 p-3"
			>
				<p class="text-sm wrap-break-word text-destructive">
					{runner.error}
				</p>
			</div>
		{:else if runner.response}
			{#if runner.response.body.trim() === ""}
				<p class="text-xs text-muted-foreground italic select-none">
					Empty response body.
				</p>
			{:else if parsed}
				<JsonView json={parsed.json} fill />
			{:else}
				<pre
					class="rounded-lg border bg-muted/40 p-3 font-mono text-xs whitespace-pre-wrap">{runner
						.response.body}</pre>
			{/if}
		{:else if runner.sending}
			<p class="text-sm text-muted-foreground select-none">
				Waiting for the server...
			</p>
		{:else if runner.cancelled}
			<p class="text-sm text-muted-foreground select-none">
				Request cancelled.
			</p>
		{:else}
			<p class="text-sm text-muted-foreground select-none">
				No response yet. Hit Send.
			</p>
		{/if}
	</div>
</section>
