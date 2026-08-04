import { open } from "@tauri-apps/plugin-dialog";
import { api, type BodyFile } from "./api";
import {
	binaryBodyContentType,
	type RequestBody,
	type SchemaObject,
} from "./openapi";
import { guessMimeType } from "./utils";

const SIGNED_UPLOADS = /\/v5\/media\/upload|\/v6\/chat\/media\/upload/;

function isEmptyObject(value: unknown): boolean {
	return (
		value !== null &&
		typeof value === "object" &&
		!Array.isArray(value) &&
		Object.keys(value).length === 0
	);
}

export class RequestBodyModel {
	readonly contentType: string;
	readonly jsonSchema: SchemaObject | undefined;
	readonly binaryContentType: string | undefined;

	view = $state<"form" | "json">("form");
	model = $state<unknown>(undefined);
	text = $state("");
	error = $state<string | null>(null);
	file = $state<{ path: string; name: string; size: number } | null>(null);
	fileContentType = $state("");
	fileError = $state<string | null>(null);
	signed = $state(false);

	readonly #spec: RequestBody;

	constructor(spec: RequestBody, path: string) {
		this.#spec = spec;
		this.contentType = Object.keys(spec.content)[0];
		this.jsonSchema = spec.content["application/json"]?.schema;
		this.binaryContentType = binaryBodyContentType(spec);
		this.fileContentType = this.binaryContentType ?? "";
		this.signed = SIGNED_UPLOADS.test(path);
	}

	// `SchemaField` edits `model` in place, so a mirror kept on write goes stale.
	#modelJson(): string {
		const value = $state.snapshot(this.model);
		return value === undefined ? "" : JSON.stringify(value, null, 2);
	}

	setModel(value: unknown) {
		this.model = value;
	}

	setText(text: string) {
		this.text = text;
		if (text.trim() === "") {
			this.error = null;
			return;
		}
		try {
			JSON.parse(text);
			this.error = null;
		} catch (e) {
			this.error = String(e);
		}
	}

	setView(view: "form" | "json") {
		if (view === "json") {
			this.text = this.#modelJson();
			this.error = null;
			this.view = "json";
			return;
		}
		const text = this.text.trim();
		if (text === "") {
			this.model = undefined;
			this.error = null;
			this.view = "form";
			return;
		}
		try {
			this.model = JSON.parse(text);
			this.error = null;
			this.view = "form";
		} catch (e) {
			this.error = `Cannot switch to form — invalid JSON: ${e}`;
		}
	}

	async pickFile() {
		try {
			const selected = await open({ multiple: false, directory: false });
			if (typeof selected !== "string") return;
			const meta = await api.statFile(selected);
			this.file = { path: selected, name: meta.name, size: meta.size };
			this.fileContentType =
				guessMimeType(meta.name) ??
				this.binaryContentType ??
				"application/octet-stream";
			this.fileError = null;
		} catch (e) {
			this.fileError = String(e);
		}
	}

	toJson(): unknown | null {
		const text =
			this.view === "form" && this.jsonSchema
				? this.#modelJson()
				: this.text.trim();
		if (text.trim() === "") return null;
		if (!this.jsonSchema) return text; // non-JSON content type: best-effort raw text
		let parsed: unknown;
		try {
			parsed = JSON.parse(text);
		} catch {
			throw new Error("Request body is not valid JSON");
		}
		return isEmptyObject(parsed) && !this.#spec.required ? null : parsed;
	}

	toFile(): BodyFile {
		if (!this.file) throw new Error("Select a file to upload.");
		return {
			path: this.file.path,
			contentType:
				this.fileContentType.trim() ||
				this.binaryContentType ||
				"application/octet-stream",
			signed: this.signed,
		};
	}
}
