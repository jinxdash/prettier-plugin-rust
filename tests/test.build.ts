import { exec } from "node:child_process";
import { promisify } from "node:util";
import prettier from "prettier";
import { testBuilds } from "../ext/jinx-rust/scripts/utils/build";
// import * as plugin_esm from "../index.js";
import * as plugin from "../src/index";

// test esm import
await promisify(exec)('node -e "import(`./index.js`)"');

testBuilds(
	plugin,
	{
		esm: await import("../index.js"),
		cjs: (await import("../index.cjs")).default,
	},
	function formatWithPlugin(file, plugin) {
		return {
			ext: "rs",
			content: prettier.format(file.content, {
				parser: "jinx-rust",
				plugins: [plugin],
				printWidth: 80,
				tabWidth: 2,
				filepath: file.cmd,
			}),
		};
	},
	["tests/samples", "ext/jinx-rust/tests/samples"]
);
