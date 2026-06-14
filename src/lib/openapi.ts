import { isTauri } from "@tauri-apps/api/core";
import { api } from "$lib/api";

export const OPENAPI_URL = "https://opengrind.org/openapi.json";

type OpenApiDocument = {
	paths: Record<string, unknown>;
	tags: Tag[];
	components: { schemas: Record<string, unknown> };
} & Record<string, unknown>;

let schema: OpenApiDocument;

export async function loadOpenApi(): Promise<void> {
	if (isTauri()) {
		schema = JSON.parse(await api.fetchOpenapi());
	} else {
		const resp = await fetch("/openapi.json");
		if (!resp.ok)
			throw new Error(`${OPENAPI_URL} returned HTTP ${resp.status}`);
		schema = await resp.json();
	}
}

export function openapiDocument(): OpenApiDocument {
	return schema;
}

export const HTTP_METHODS = [
	"get",
	"post",
	"put",
	"patch",
	"delete",
	"head",
	"options",
] as const;
export type HttpMethod = (typeof HTTP_METHODS)[number];

// Resolve a $ref string like "#/components/parameters/Foo" into the actual object
export function resolveRef(ref: string): unknown {
	return ref
		.replace(/^#\//, "")
		.split("/")
		.reduce(
			(obj: Record<string, unknown>, key) => {
				return obj[key] as Record<string, unknown>;
			},
			schema as unknown as Record<string, unknown>,
		);
}

export type SchemaObject = {
	type?: string;
	format?: string;
	$ref?: string;
	enum?: unknown[];
	description?: string;
	nullable?: boolean;
	deprecated?: boolean;
	items?: SchemaObject;
	properties?: Record<string, SchemaObject>;
	required?: string[];
	allOf?: SchemaObject[];
	oneOf?: SchemaObject[];
	anyOf?: SchemaObject[];
	additionalProperties?: boolean | SchemaObject;
	"x-render-on-tag"?: string;
	"x-display-name"?: string;
	"x-enum-labels"?: Record<string, string>;
	"x-wip"?: boolean;
	"x-exclude-from-markdown"?: boolean;
	"x-property-groups"?: Array<{
		heading: string;
		properties?: Record<string, SchemaObject>;
		allOf?: Array<{ $ref: string }>;
	}>;
};

export type Param = {
	name: string;
	in: "query" | "path" | "header" | "cookie";
	description?: string;
	required?: boolean;
	deprecated?: boolean;
	schema?: SchemaObject;
};

export type RequestBody = {
	required?: boolean;
	description?: string;
	content: Record<string, { schema: SchemaObject }>;
};

export type Response = {
	description: string;
	content?: Record<string, { schema: SchemaObject }>;
};

export type Operation = {
	method: HttpMethod;
	summary?: string;
	description?: string;
	operationId?: string;
	tags: string[];
	parameters: Param[];
	requestBody?: RequestBody;
	responses: Record<string, Response>;
	security?: Record<string, string[]>[];
	deprecated?: boolean;
	"x-wip"?: boolean;
	"x-wip-note"?: string;
	"x-legacy"?: boolean;
	"x-idempotent"?: boolean;
	"x-paid"?: boolean;
	"x-notes"?: string[];
	"x-errors"?: Record<string, string>;
	"x-see-also"?: string[];
	"x-query-groups"?: string[];
};

export type Tag = {
	name: string;
	description?: string;
	"x-wip"?: boolean;
	"x-wip-note"?: string;
};

export type ParameterGroup = {
	"x-render-on-tag": string;
	"x-inherits"?: string;
	parameters: Array<{ $ref: string } | Param>;
};

export type TagOperation = {
	path: string;
	op: Operation;
};

// ─── Path operations ─────────────────────────────────────────────────────────

type RawOp = Omit<Operation, "method" | "parameters"> & {
	parameters?: ({ $ref: string } | Param)[];
};
type RawPathItem = Partial<Record<HttpMethod, RawOp>>;

function resolveParams(raw: RawOp): Param[] {
	return (raw.parameters ?? []).map((p) => {
		if ("$ref" in p) return resolveRef(p.$ref) as Param;
		return p;
	});
}

/** Return all HTTP operations for a given OpenAPI path key (e.g. "/v3/cascade"). */
export function getOperations(pathKey: string): Operation[] {
	const pathItem = (schema.paths as unknown as Record<string, RawPathItem>)[
		pathKey
	];
	if (!pathItem) return [];
	return HTTP_METHODS.flatMap((method) => {
		const raw = pathItem[method];
		if (!raw) return [];
		return [
			{
				...raw,
				method,
				tags: raw.tags ?? [],
				parameters: resolveParams(raw),
			} as Operation,
		];
	});
}

/** Return all operations tagged with a given tag name, in spec order. */
export function getOperationsForTag(tagName: string): TagOperation[] {
	const result: TagOperation[] = [];
	for (const [pathKey, pathItem] of Object.entries(
		schema.paths as unknown as Record<string, RawPathItem>,
	)) {
		for (const method of HTTP_METHODS) {
			const raw = pathItem[method];
			if (!raw?.tags?.includes(tagName)) continue;
			result.push({
				path: pathKey,
				op: {
					...raw,
					method,
					tags: raw.tags ?? [],
					parameters: resolveParams(raw),
				} as Operation,
			});
		}
	}
	return result;
}

// ─── Tag lookups ─────────────────────────────────────────────────────────────

/** Look up a tag definition by name. */
export function getTag(name: string): Tag | undefined {
	return (schema.tags as Tag[]).find((t) => t.name === name);
}

// ─── Schema lookups ───────────────────────────────────────────────────────────

export function getSchema(name: string): SchemaObject | undefined {
	return (schema.components.schemas as unknown as Record<string, SchemaObject>)[name];
}

/** Return the tag page this schema is rendered on (from x-render-on-tag). */
export function schemaPageTag(name: string): string | undefined {
	return getSchema(name)?.["x-render-on-tag"];
}

/** Return all schemas assigned to the given tag page, sorted by display name. */
export function getSchemasForTag(
	tagName: string,
): Array<{ name: string; schema: SchemaObject }> {
	return Object.entries(
		schema.components.schemas as unknown as Record<string, SchemaObject>,
	)
		.filter(([, s]) => s["x-render-on-tag"] === tagName)
		.map(([name, s]) => ({ name, schema: s }))
		.sort((a, b) => {
			const dA = a.schema["x-display-name"] ?? a.name;
			const dB = b.schema["x-display-name"] ?? b.name;
			return dA.localeCompare(dB);
		});
}

// ─── Parameter groups ─────────────────────────────────────────────────────────

export function getParamGroupsForTag(
	tagName: string,
): Array<{ name: string; group: ParameterGroup }> {
	const groups =
		(
			schema as unknown as {
				"x-parameter-groups"?: Record<string, ParameterGroup>;
			}
		)["x-parameter-groups"] ?? {};
	return Object.entries(groups)
		.filter(([, g]) => g["x-render-on-tag"] === tagName)
		.map(([name, group]) => ({ name, group }));
}

/** Resolve all parameters in a parameter group. */
export function resolveGroupParams(group: ParameterGroup): Param[] {
	return group.parameters.map((p) => {
		if ("$ref" in p) return resolveRef(p.$ref) as Param;
		return p;
	});
}

// ─── Type labels ─────────────────────────────────────────────────────────────

/** Extract a human-readable type label from a schema object (non-linking). */
export function schemaTypeLabel(s: SchemaObject | undefined): string {
	if (!s) return "";
	if (s.$ref) {
		const parts = s.$ref.split("/");
		return parts[parts.length - 1] ?? "";
	}
	if (s.type === "array" && s.items) return `${schemaTypeLabel(s.items)}[]`;
	if (s.enum) return s.enum.map(String).join(" | ");
	return s.format ? `${s.type} (${s.format})` : (s.type ?? "");
}
