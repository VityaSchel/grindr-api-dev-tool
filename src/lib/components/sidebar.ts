import schema from "$lib/schema/openapi.json";

const HTTP_METHODS = [
	"get",
	"post",
	"put",
	"patch",
	"delete",
	"head",
	"options",
] as const;

export type NavEndpoint = { title: string; url: string };
export type NavSubGroup = { title: string; tagName: string; items: NavEndpoint[] };
export type NavEntry =
	| { kind: "standalone"; title: string; items: NavEndpoint[] }
	| { kind: "grouped"; label: string; subGroups: NavSubGroup[] };

type Operation = { summary?: string; tags?: string[] };
type PathItem = Partial<Record<(typeof HTTP_METHODS)[number], Operation>>;
type SidebarOrderItem = string | { group: string; items: string[] };

const tagEndpoints: Record<string, NavEndpoint[]> = {};

for (const [pathKey, pathItem] of Object.entries(
	schema.paths as unknown as Record<string, PathItem>,
)) {
	for (const method of HTTP_METHODS) {
		const op = pathItem[method];
		if (!op?.summary || !op?.tags) continue;
		for (const tag of op.tags) {
			(tagEndpoints[tag] ??= []).push({ title: op.summary, url: pathKey });
		}
	}
}

export const navEntries: NavEntry[] = (
	schema["x-sidebar-order"] as SidebarOrderItem[]
).flatMap<NavEntry>((entry) => {
	if (typeof entry === "string") {
		const items = tagEndpoints[entry];
		if (!items?.length) return [];
		return [{ kind: "standalone" as const, title: entry, items }];
	}
	const subGroups = entry.items.flatMap((tag) => {
		const slashIdx = tag.indexOf("/");
		const suffix = slashIdx !== -1 ? tag.slice(slashIdx + 1) : tag;
		const items = tagEndpoints[tag];
		if (!items?.length) return [];
		return [{ title: suffix, tagName: tag, items }];
	});
	if (!subGroups.length) return [];
	return [{ kind: "grouped" as const, label: entry.group, subGroups }];
});