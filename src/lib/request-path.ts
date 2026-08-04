import type { Param, SchemaObject } from "./openapi";

function pair(name: string, value: unknown): string {
	return `${encodeURIComponent(name)}=${encodeURIComponent(String(value))}`;
}

function isFilled(value: unknown): boolean {
	return value !== undefined && value !== null && value !== "";
}

export function buildRequestPath(
	template: string,
	pathParams: Param[],
	pathModel: Record<string, unknown>,
	queryParams: Param[],
	queryModel: Record<string, unknown>,
	unknownQuery: string[] = [],
): string {
	let path = template;
	for (const param of pathParams) {
		path = path.replaceAll(
			`{${param.name}}`,
			encodeURIComponent(String(pathModel[param.name] ?? "")),
		);
	}

	const query: string[] = [];
	for (const param of queryParams) {
		const value = queryModel[param.name];
		if (!isFilled(value)) continue;
		if (Array.isArray(value)) {
			for (const item of value)
				if (isFilled(item)) query.push(pair(param.name, item));
		} else if (typeof value === "object" && value !== null) {
			if (Object.keys(value).length)
				query.push(pair(param.name, JSON.stringify(value)));
		} else {
			query.push(pair(param.name, value));
		}
	}
	query.push(...unknownQuery);

	return query.length ? `${path}?${query.join("&")}` : path;
}

function coerce(schema: SchemaObject | undefined, raw: string): unknown {
	switch (schema?.type) {
		case "integer":
		case "number": {
			const n = Number(raw);
			return raw.trim() !== "" && Number.isFinite(n) ? n : raw;
		}
		case "boolean":
			return raw === "true" ? true : raw === "false" ? false : raw;
		case "object":
			try {
				return JSON.parse(raw);
			} catch {
				return raw;
			}
		default:
			return raw;
	}
}

export type ParsedRequestPath = {
	path: Record<string, unknown>;
	query: Record<string, unknown>;
	unknownQuery: string[];
};

export function parseRequestPath(
	template: string,
	text: string,
	pathParams: Param[],
	queryParams: Param[],
): ParsedRequestPath | null {
	const cut = text.indexOf("?");
	const rawPath = cut === -1 ? text : text.slice(0, cut);
	const rawQuery = cut === -1 ? "" : text.slice(cut + 1);

	const literals = template.split(/\{[^}]+\}/);
	const names = [...template.matchAll(/\{([^}]+)\}/g)].map((m) => m[1]);
	const pattern = literals
		.map((literal) => literal.replace(/[.*+?^${}()|[\]\\]/g, "\\$&"))
		.join("([^/]*)");
	const match = new RegExp(`^${pattern}$`).exec(rawPath);
	if (!match) return null;

	const path: Record<string, unknown> = {};
	names.forEach((name, i) => {
		const raw = match[i + 1] ?? "";
		const param = pathParams.find((p) => p.name === name);
		path[name] =
			raw === "" ? "" : coerce(param?.schema, decodeURIComponent(raw));
	});

	const params = new URLSearchParams(rawQuery);
	const query: Record<string, unknown> = {};
	for (const param of queryParams) {
		const values = params.getAll(param.name);
		if (!values.length) {
			query[param.name] = undefined;
			continue;
		}
		query[param.name] =
			param.schema?.type === "array"
				? values.map((v) => coerce(param.schema?.items, v))
				: coerce(param.schema, values[0]);
	}

	const declared = new Set(queryParams.map((p) => p.name));
	const unknownQuery = [...params]
		.filter(([name]) => !declared.has(name))
		.map(([name, value]) => pair(name, value));

	return { path, query, unknownQuery };
}
