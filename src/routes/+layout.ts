import { error } from "@sveltejs/kit";
import { loadOpenApi, OPENAPI_URL } from "$lib/openapi";
import { buildNav } from "$lib/components/sidebar";

// Tauri doesn't have a Node.js server to do proper SSR
// so we use adapter-static with a fallback to index.html to put the site in SPA mode
// See: https://svelte.dev/docs/kit/single-page-apps
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info
export const ssr = false;

export async function load() {
	try {
		await loadOpenApi();
	} catch (e) {
		error(
			503,
			`Could not load the OpenAPI spec from ${OPENAPI_URL}: ${e instanceof Error ? e.message : e}`,
		);
	}
	buildNav();
}
