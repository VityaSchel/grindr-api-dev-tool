import { includeIgnoreFile } from "@eslint/compat";
import { sveltekit } from "@opengrind/config/eslint/svelte";
import { defineConfig } from "eslint/config";
import path from "node:path";

import svelteConfig from "./svelte.config.js";

const gitignorePath = path.resolve(import.meta.dirname, ".gitignore");

export default defineConfig(
	includeIgnoreFile(gitignorePath),
	...sveltekit({
		svelteConfig,
		tailwindEntry: "src/routes/layout.css",
		vendoredGlob: "src/lib/components/ui/**",
		ignores: ["src-tauri/"],
	}),
);
