<script lang="ts">
	import { goto } from "$app/navigation";
	import PlusIcon from "phosphor-svelte/lib/PlusIcon";

	import { Button } from "$lib/components/ui/button";
	import { CUSTOM_TAB_PATH, grindrApiHref } from "$lib/links";
	import AccountSwitcher from "./account/AccountSwitcher.svelte";
</script>

<header
	class="flex h-12 shrink-0 items-center justify-between gap-2 border-b px-3"
	data-slot="navbar"
>
	<div class="flex min-w-0 items-center gap-4">
		<span class="truncate text-sm font-medium text-muted-foreground">
			Grindr API
		</span>
		<Button
			variant="outline"
			size="xs"
			class="gap-2"
			onclick={() => goto(grindrApiHref(CUSTOM_TAB_PATH))}
		>
			<PlusIcon />
			<span class="hidden sm:inline">Custom request</span>
		</Button>
	</div>

	<AccountSwitcher />
</header>
