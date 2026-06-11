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

export type NavEndpoint = { title: string; url: string; method: string };
export type NavSubGroup = {
	title: string;
	tagName: string;
	items: NavEndpoint[];
};
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
			(tagEndpoints[tag] ??= []).push({
				title: op.summary,
				url: pathKey,
				method,
			});
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

/** Title-case a tag/segment name, e.g. "muted_profiles" → "Muted Profiles". */
export function toTitle(str: string): string {
	return str
		.split(/[-_/]/)
		.map((word) => word.charAt(0).toUpperCase() + word.slice(1))
		.join(" ");
}

// Map every navigable `[...path]` value to the label the sidebar shows for it, so tabs
// can reuse the same human title (endpoint summary, or title-cased tag/group name).
// Endpoints are also keyed by `method:path`, since one URL can host several methods
// with distinct summaries (e.g. "Get cascade" vs "Refresh cascade").
const navTitleByPath: Record<string, string> = {};
const stripSlash = (u: string) => (u.startsWith("/") ? u.slice(1) : u);
const endpointKey = (method: string, url: string) =>
	`${method}:${stripSlash(url)}`;
function addEndpoint(it: NavEndpoint) {
	navTitleByPath[endpointKey(it.method, it.url)] = it.title;
	// Path-only fallback for method-less lookups (last method seen wins — best effort).
	navTitleByPath[stripSlash(it.url)] ??= it.title;
}
for (const entry of navEntries) {
	if (entry.kind === "standalone") {
		navTitleByPath[entry.title] = toTitle(entry.title);
		for (const it of entry.items) addEndpoint(it);
	} else {
		navTitleByPath[entry.label] = toTitle(entry.label);
		for (const sg of entry.subGroups) {
			navTitleByPath[sg.tagName] = toTitle(sg.title);
			for (const it of sg.items) addEndpoint(it);
		}
	}
}

/** The sidebar's label for a `[...path]` value (scoped to `method` when given),
 * falling back to the path-only title, then its last segment. */
export function navTitle(path: string, method?: string | null): string {
	return (
		(method ? navTitleByPath[endpointKey(method, path)] : undefined) ??
		navTitleByPath[path] ??
		path.split("/").filter(Boolean).pop() ??
		path
	);
}
