/**
 * Open endpoint/tag tabs. A tab is identified by its `[...path]` rest value
 * (e.g. "v3/cascade") *plus* the HTTP method it's scoped to (e.g. "post"), so the
 * same URL can live as separate per-method tabs instead of merging into one page.
 * Tag pages and the custom-request page carry a `null` method.
 *
 * Each open tab keeps a mounted view in the layout so its in-progress request state
 * survives switching away and back — which is why we no longer spawn a separate OS
 * window per request.
 *
 * The *active* tab is derived from the current URL (`page.params.path` + the `?m=`
 * query), so it lives with the router rather than here; this store only owns the set
 * and order of tabs.
 */
export type Tab = { path: string; method: string | null };

/** Stable key for a tab, unique per (path, method) — used for keyed `{#each}`. */
export function tabKey(path: string, method: string | null): string {
	return method ? `${method}:${path}` : path;
}

function sameTab(t: Tab, path: string, method: string | null): boolean {
	return t.path === path && t.method === method;
}

class TabsStore {
	tabs = $state<Tab[]>([]);

	has(path: string, method: string | null): boolean {
		return this.tabs.some((t) => sameTab(t, path, method));
	}

	/** Ensure a tab exists for `(path, method)` (no-op if already open). */
	ensure(path: string, method: string | null) {
		if (!this.has(path, method)) this.tabs.push({ path, method });
	}

	/**
	 * Remove the tab for `(path, method)`. Returns the neighbouring tab — the one
	 * that should become active if the caller was viewing the closed tab — or `null`
	 * if no tabs remain.
	 */
	close(path: string, method: string | null): Tab | null {
		const idx = this.tabs.findIndex((t) => sameTab(t, path, method));
		if (idx < 0) return null;
		this.tabs.splice(idx, 1);
		return this.tabs[idx] ?? this.tabs[idx - 1] ?? null;
	}
}

export const tabs = new TabsStore();
