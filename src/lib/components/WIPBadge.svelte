<script>
	import { Badge } from "$lib/components/ui/badge";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import BarricadeIcon from "phosphor-svelte/lib/BarricadeIcon";
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger class="flex">
			<Badge variant="outline" class="border-amber-800/20 bg-amber-300/20">
				<BarricadeIcon />
				WIP
			</Badge>
		</Tooltip.Trigger>
		<Tooltip.Content class="text-center">
			<p>This section has not been fully researched or documented yet.</p>
		</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
