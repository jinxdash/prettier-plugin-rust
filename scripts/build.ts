import { build } from "tsup";
import { createStripPlugin } from "../ext/jinx-rust/scripts/utils/build";

await build({
	dts: true,
	tsconfig: "tsconfig.build.json",
	entry: ["src/index.ts"],
	external: ["jinx-rust"],
	outDir: ".",
	format: ["cjs", "esm"],
	plugins: [createStripPlugin({ labels: ["__DEV__"], functionCalls: ["devonly"] })],
	treeshake: {
		preset: "smallest",
		moduleSideEffects: false,
		propertyReadSideEffects: false,
		tryCatchDeoptimization: false,
		unknownGlobalSideEffects: false,
	},
});
