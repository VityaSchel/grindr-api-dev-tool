<script>
	import { Badge } from "$lib/components/ui/badge";
	import KeyIcon from "phosphor-svelte/lib/KeyIcon";
</script>

<Badge
	variant="outline"
	href="/rest/authentication"
	class="border-red-800/20 bg-red-600/20 hover:bg-red-600/30! hover:text-black!"
>
	<KeyIcon />
	Auth Required
</Badge>
