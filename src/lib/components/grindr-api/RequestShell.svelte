<script lang="ts">
	import type { Snippet } from "svelte";
	import {
		responsePane,
		MIN_REQUEST_HEIGHT,
		MIN_RESPONSE_HEIGHT,
	} from "$lib/response-pane.svelte";

	let {
		request,
		response,
		showResponse = true,
	}: { request: Snippet; response: Snippet; showResponse?: boolean } = $props();

	let root = $state<HTMLElement | null>(null);

	function fit(px: number): number {
		const room = (root?.clientHeight ?? 0) - MIN_REQUEST_HEIGHT;
		return Math.min(
			Math.max(px, MIN_RESPONSE_HEIGHT),
			Math.max(room, MIN_RESPONSE_HEIGHT),
		);
	}

	function startDrag(e: PointerEvent) {
		const handle = e.currentTarget as HTMLElement;
		const bottom = root?.getBoundingClientRect().bottom ?? 0;
		e.preventDefault();
		handle.setPointerCapture(e.pointerId);

		const move = (ev: PointerEvent) =>
			(responsePane.current = fit(bottom - ev.clientY));
		const stop = () => {
			handle.removeEventListener("pointermove", move);
			handle.removeEventListener("pointerup", stop);
			handle.removeEventListener("pointercancel", stop);
			if (handle.hasPointerCapture(e.pointerId))
				handle.releasePointerCapture(e.pointerId);
			responsePane.remember();
		};
		handle.addEventListener("pointermove", move);
		handle.addEventListener("pointerup", stop);
		handle.addEventListener("pointercancel", stop);
	}

	function nudge(e: KeyboardEvent) {
		const step = e.shiftKey ? 64 : 16;
		if (e.key === "ArrowUp")
			responsePane.current = fit(responsePane.current + step);
		else if (e.key === "ArrowDown")
			responsePane.current = fit(responsePane.current - step);
		else return;
		responsePane.remember();
		e.preventDefault();
	}
</script>

<div bind:this={root} class="flex h-full min-h-0 flex-col">
	<div data-scroll-container class="min-h-0 flex-1 overflow-auto">
		{@render request()}
	</div>

	{#if showResponse}
		<button
			type="button"
			aria-label="Resize response"
			class="flex h-2 shrink-0 cursor-row-resize items-center justify-center after:h-px after:w-full after:bg-border hover:after:bg-ring focus-visible:outline-hidden focus-visible:after:bg-ring"
			onpointerdown={startDrag}
			onkeydown={nudge}
		></button>

		<div
			class="flex shrink-0 flex-col"
			style="height: {responsePane.current}px; max-height: calc(100% - {MIN_REQUEST_HEIGHT}px)"
		>
			{@render response()}
		</div>
	{/if}
</div>
