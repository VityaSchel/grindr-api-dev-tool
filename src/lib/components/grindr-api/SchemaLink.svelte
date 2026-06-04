<script lang="ts">
	import { resolve } from "$app/paths";
	import { schemaPageTag } from "$lib/openapi";
	import type { SchemaObject } from "$lib/openapi";

	let { schema }: { schema: SchemaObject | undefined } = $props();

	type LinkInfo = {
		label: string;
		tag: string | undefined;
		name: string | undefined;
		suffix: string;
	};

	function getLinkInfo(s: SchemaObject | undefined, depth = 0): LinkInfo {
		if (!s) return { label: "", tag: undefined, name: undefined, suffix: "" };
		if (s.$ref) {
			const name = s.$ref.replace("#/components/schemas/", "");
			const tag = schemaPageTag(name);
			return { label: name, tag, name, suffix: "" };
		}
		if (s.type === "array" && s.items && depth < 3) {
			const inner = getLinkInfo(s.items, depth + 1);
			return { ...inner, suffix: "[]" + inner.suffix };
		}
		if (s.enum) {
			return {
				label: s.enum.map(String).join(" | "),
				tag: undefined,
				name: undefined,
				suffix: "",
			};
		}
		const label = s.format ? `${s.type} (${s.format})` : (s.type ?? "");
		return { label, tag: undefined, name: undefined, suffix: "" };
	}

	const info = $derived(getLinkInfo(schema));
	const link = $derived(
		info.tag && info.name
			? resolve(`/grindr-api/${info.tag}`) + `#${info.name}`
			: null,
	);
</script>

{#if link}
	<a
		href={link}
		class="font-mono text-xs text-blue-500 hover:underline dark:text-blue-400"
	>
		{info.label}
	</a>{info.suffix}
{:else}
	<span class="font-mono text-xs text-muted-foreground">
		{info.label}{info.suffix}
	</span>
{/if}
