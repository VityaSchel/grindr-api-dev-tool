<script>
	import { Badge } from "$lib/components/ui/badge";
	import * as Tooltip from "$lib/components/ui/tooltip";
	import ArrowsClockwiseIcon from "phosphor-svelte/lib/ArrowClockwiseIcon";
</script>

<Tooltip.Provider>
	<Tooltip.Root>
		<Tooltip.Trigger class="flex">
			<Badge variant="outline" class="bg-blue-600/20 border-blue-800/20">
				<ArrowsClockwiseIcon />
				Idempotent
			</Badge>
		</Tooltip.Trigger>
		<Tooltip.Content>
			<p>Repeated requests are completed without errors.</p>
		</Tooltip.Content>
	</Tooltip.Root>
</Tooltip.Provider>
