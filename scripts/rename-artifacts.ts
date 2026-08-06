import { renameSync } from "node:fs";
import path from "node:path";

import {
	bundleDirs,
	findArtifacts,
	type Platform,
	PLATFORMS,
	readManifest,
	targetName,
} from "./artifacts";

function main(platform?: Platform): void {
	const manifest = readManifest();
	let renamed = 0;
	let kept = 0;

	for (const dir of bundleDirs(platform)) {
		for (const file of findArtifacts(dir)) {
			const target = targetName(file, manifest);
			if (!target) {
				console.warn(`skipped (no architecture in name): ${file}`);
				continue;
			}
			if (path.basename(file) === target) {
				kept += 1;
				continue;
			}
			renameSync(file, path.join(path.dirname(file), target));
			console.log(`${path.basename(file)} -> ${target}`);
			renamed += 1;
		}
	}

	console.log(`renamed ${renamed}, already named ${kept}`);
}

const requested = process.argv[2] as Platform | undefined;
if (requested && !PLATFORMS.includes(requested)) {
	console.error(`usage: rename-artifacts.ts [${PLATFORMS.join("|")}]`);
	process.exit(2);
}

try {
	main(requested);
} catch (error) {
	console.error(error instanceof Error ? error.message : String(error));
	process.exit(1);
}
