<script lang="ts">
	import ArrowsClockwiseIcon from "phosphor-svelte/lib/ArrowsClockwiseIcon";

	import { DEVICE_FIELD_LABELS, type DeviceInfo } from "$lib/api";
	import { Button } from "$lib/components/ui/button";
	import { Input } from "$lib/components/ui/input";
	import { Label } from "$lib/components/ui/label";
	import { Skeleton } from "$lib/components/ui/skeleton";

	let {
		device = $bindable(),
		description,
		busy = false,
		onRegenerate,
	}: {
		device: DeviceInfo | null;
		description: string;
		busy?: boolean;
		onRegenerate: () => void;
	} = $props();

	const uid = $props.id();
	const keys = Object.keys(DEVICE_FIELD_LABELS) as (keyof DeviceInfo)[];

	function clampU8(value: string): number {
		const n = Math.trunc(Number(value));
		return Number.isFinite(n) ? Math.min(255, Math.max(0, n)) : 0;
	}

	// A 0 -> 0 clamp re-renders nothing, so rejected text would linger in the field.
	let deviceTypeText = $derived(device ? String(device.device_type) : "");

	function setField(key: keyof DeviceInfo, value: string) {
		if (!device) return;
		device = {
			...device,
			[key]: key === "device_type" ? clampU8(value) : value,
		};
	}
</script>

<div class="flex flex-col gap-2">
	<div class="flex items-center justify-between">
		<Label class="text-xs font-medium">Device parameters</Label>
		<Button
			type="button"
			variant="outline"
			size="xs"
			onclick={onRegenerate}
			disabled={busy}
		>
			<ArrowsClockwiseIcon />
			Regenerate
		</Button>
	</div>
	<p class="text-xs text-muted-foreground">{description}</p>
	<div class="grid grid-cols-2 gap-2">
		{#each keys as key (key)}
			<div class="flex flex-col gap-1">
				<Label for="{uid}-{key}" class="text-xs text-muted-foreground">
					{DEVICE_FIELD_LABELS[key]}
				</Label>
				{#if device}
					{#if key === "device_type"}
						<Input
							id="{uid}-{key}"
							class="h-7 font-mono text-xs"
							bind:value={deviceTypeText}
							oninput={(e) =>
								setField(key, e.currentTarget.value)}
						/>
					{:else}
						<Input
							id="{uid}-{key}"
							class="h-7 font-mono text-xs"
							value={String(device[key])}
							oninput={(e) =>
								setField(key, e.currentTarget.value)}
						/>
					{/if}
				{:else}
					<Skeleton class="h-7 w-full" />
				{/if}
			</div>
		{/each}
	</div>
</div>
