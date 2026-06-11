import { HTTP_METHODS, type HttpMethod } from "$lib/openapi";

export const METHOD_COLORS: Record<string, string> = {
	get: "oklch(0.7 0.1702 146.12)",
	post: "oklch(0.7 0.1258 72)",
	put: "oklch(0.7 0.1455 253.06)",
	delete: "oklch(0.7 0.185 31.76)",
	patch: "oklch(0.7 0.1949 316.59)",
	head: "oklch(0.65 0.01 220)",
	options: "oklch(0.65 0.01 220)",
};

export function methodColor(method: string): string {
	return METHOD_COLORS[method.toLowerCase()] ?? "var(--foreground)";
}

export const SELECTABLE_METHODS: HttpMethod[] = [...HTTP_METHODS];
