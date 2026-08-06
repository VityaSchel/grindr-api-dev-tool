import { existsSync, readdirSync, readFileSync, statSync } from "node:fs";
import path from "node:path";

const root = path.resolve(import.meta.dirname, "..");

export const DIST = path.join(root, "dist");
export const RELEASE_DIR = path.join(DIST, "release");
export const PLATFORMS = ["macos", "linux", "windows"] as const;
export type Platform = (typeof PLATFORMS)[number];

const EXTENSIONS = ["rpm", "deb", "AppImage", "dmg", "exe"] as const;
type Extension = (typeof EXTENSIONS)[number];

const CANONICAL_ARCH: Record<string, string> = {
	amd64: "x86_64",
	x64: "x86_64",
	x86_64: "x86_64",
	arm64: "aarch64",
	aarch64: "aarch64",
	universal: "universal",
};

const ARCH_SPELLING: Record<Extension, Record<string, string>> = {
	rpm: { x86_64: "x86_64", aarch64: "aarch64" },
	deb: { x86_64: "amd64", aarch64: "arm64" },
	AppImage: { x86_64: "x86_64", aarch64: "aarch64" },
	dmg: { x86_64: "x86_64", aarch64: "aarch64", universal: "universal" },
	exe: { x86_64: "x64", aarch64: "arm64" },
};

const ARCH_TOKENS_LONGEST_FIRST = Object.keys(CANONICAL_ARCH).sort(
	(a, b) => b.length - a.length,
);

const BUNDLED_DIRECTORY_SUFFIXES = [".app", ".AppDir"];

export type Manifest = { stem: string; version: string };

function readJson(file: string): Record<string, unknown> {
	return JSON.parse(readFileSync(file, "utf8")) as Record<string, unknown>;
}

export function readManifest(): Manifest {
	const pkg = readJson(path.join(root, "package.json"));
	const tauri = readJson(path.join(root, "src-tauri/tauri.conf.json"));
	const version = String(pkg.version);
	if (version !== String(tauri.version)) {
		throw new Error(
			`version mismatch: package.json ${version}, tauri.conf.json ${String(tauri.version)}`,
		);
	}
	const stem = String(tauri.productName)
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
	return { stem, version };
}

function extensionOf(file: string): Extension | undefined {
	const name = file.toLowerCase();
	return EXTENSIONS.find((extension) =>
		name.endsWith(`.${extension.toLowerCase()}`),
	);
}

function archOf(file: string): string | undefined {
	const name = path.basename(file).toLowerCase();
	const token = ARCH_TOKENS_LONGEST_FIRST.find((candidate) =>
		new RegExp(`(^|[^a-z0-9])${candidate}([^a-z0-9]|$)`).test(name),
	);
	return token ? CANONICAL_ARCH[token] : undefined;
}

export function targetName(
	file: string,
	{ stem, version }: Manifest,
): string | undefined {
	const extension = extensionOf(file);
	const arch = archOf(file);
	if (!extension || !arch) return undefined;
	const spelling = ARCH_SPELLING[extension][arch];
	return spelling ? `${stem}-${version}.${spelling}.${extension}` : undefined;
}

function isBundledDirectory(entry: string): boolean {
	return BUNDLED_DIRECTORY_SUFFIXES.some((suffix) => entry.endsWith(suffix));
}

export function findArtifacts(dir: string): string[] {
	if (!existsSync(dir)) return [];
	const found: string[] = [];
	for (const entry of readdirSync(dir)) {
		const full = path.join(dir, entry);
		if (!statSync(full).isDirectory()) {
			if (extensionOf(entry)) found.push(full);
		} else if (!isBundledDirectory(entry)) {
			found.push(...findArtifacts(full));
		}
	}
	return found.sort();
}

export function bundleDirs(platform?: Platform): string[] {
	const platforms = platform ? [platform] : PLATFORMS;
	return platforms.map((name) => path.join(DIST, name));
}
