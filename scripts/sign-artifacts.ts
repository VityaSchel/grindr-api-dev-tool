import { spawnSync } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, rmSync, statSync } from "node:fs";
import { homedir } from "node:os";
import path from "node:path";
import {
	bundleDirs,
	DIST,
	findArtifacts,
	readManifest,
	RELEASE_DIR,
	targetName,
} from "./artifacts";

const untilde = (file: string) => file.replace(/^~/, homedir());

function requireMinisign(): void {
	if (spawnSync("minisign", ["-v"], { stdio: "ignore" }).error) {
		throw new Error("minisign not found — install it (brew install minisign)");
	}
}

function collect(): string[] {
	const manifest = readManifest();
	const sources = bundleDirs().flatMap((dir) => findArtifacts(dir));
	if (!sources.length) {
		throw new Error(`no artifacts under ${DIST} — run 'bun run bundle' first`);
	}

	rmSync(RELEASE_DIR, { recursive: true, force: true });
	mkdirSync(RELEASE_DIR, { recursive: true });

	const collected: string[] = [];
	for (const source of sources) {
		const target = targetName(source, manifest);
		if (!target) {
			console.warn(`skipped (no architecture in name): ${source}`);
			continue;
		}
		const destination = path.join(RELEASE_DIR, target);
		if (existsSync(destination)) {
			throw new Error(`two artifacts map to ${target}, refusing to overwrite`);
		}
		copyFileSync(source, destination);
		collected.push(destination);
	}
	return collected;
}

function sign(files: string[]): void {
	const secret = process.env.OPEN_GRIND_MINISIGN_KEY;
	const secretArgs = secret ? ["-s", untilde(secret)] : [];
	const { status } = spawnSync(
		"minisign",
		["-S", ...secretArgs, "-m", ...files],
		{ stdio: "inherit" },
	);
	if (status !== 0) throw new Error(`minisign exited ${status}`);
}

function publicKeyArgs(): string[] | undefined {
	const key = process.env.OPEN_GRIND_MINISIGN_PUBKEY;
	if (key) return ["-P", key];
	const file = untilde("~/.minisign/minisign.pub");
	return existsSync(file) ? ["-p", file] : undefined;
}

function verify(files: string[]): boolean {
	const keyArgs = publicKeyArgs();
	if (!keyArgs) {
		console.warn(
			"no public key (set OPEN_GRIND_MINISIGN_PUBKEY or keep ~/.minisign/minisign.pub) — signatures are unverified",
		);
		return false;
	}
	for (const file of files) {
		const { status } = spawnSync("minisign", ["-Vqm", file, ...keyArgs], {
			stdio: ["inherit", "ignore", "inherit"],
		});
		if (status !== 0) {
			throw new Error(`signature does not verify: ${path.basename(file)}`);
		}
	}
	return true;
}

function report(files: string[], verified: boolean): void {
	for (const file of files) {
		const megabytes = (statSync(file).size / 1024 ** 2).toFixed(1);
		console.log(`  ${path.basename(file).padEnd(48)} ${megabytes} MB`);
		console.log(`  ${path.basename(file)}.minisig`);
	}
	const suffix = verified ? " (verified)" : "";
	console.log(
		`\n${files.length} artifacts + ${files.length} signatures in ${RELEASE_DIR}${suffix}`,
	);
}

function main(): void {
	requireMinisign();
	const files = collect();
	sign(files);
	report(files, verify(files));
}

try {
	main();
} catch (error) {
	console.error(error instanceof Error ? error.message : String(error));
	process.exit(1);
}
