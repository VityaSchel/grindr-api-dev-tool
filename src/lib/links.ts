import { base } from "$app/paths";

/**
 * Percent-encode each path segment so characters that are structural in a URL —
 * notably `?` (query) and `#` (fragment), but also `{`/`}` in templated paths —
 * survive as part of the path. Slashes stay as separators.
 */
function encodeSegments(path: string): string {
	return path.split("/").map(encodeURIComponent).join("/");
}

/** Query-param key carrying the HTTP method, so the same URL can open as separate
 * per-method tabs (e.g. GET vs POST `/v3/cascade`). */
export const METHOD_PARAM = "m";

/** Reserved `[...path]` value for the ad-hoc custom-request tab (not a real API path). */
export const CUSTOM_TAB_PATH = "~custom";

export type HrefOptions = {
	/** HTTP method to scope an endpoint page to (lowercase, e.g. "post"). */
	method?: string | null;
	/** In-page anchor (schema name, parameter group, doc section). */
	anchor?: string;
};

/**
 * Build the href for a `/grindr-api/<path>` page from a `[...path]` value (e.g.
 * "v?/sessions/thirdparty"), optionally scoped to an HTTP `method` and/or pointing
 * at an in-page `anchor`. Without the segment encoding, a `?` in the path would
 * start a query string and the route would resolve to the wrong (truncated) path;
 * the literal `?` separating our own `method` query is added afterwards, so it
 * stays distinct from any encoded `%3F` inside the path.
 */
export function grindrApiHref(path: string, opts?: HrefOptions): string {
	let href = `${base}/grindr-api/${encodeSegments(path)}`;
	if (opts?.method) {
		href += `?${METHOD_PARAM}=${encodeURIComponent(opts.method)}`;
	}
	if (opts?.anchor) href += `#${opts.anchor}`;
	return href;
}
