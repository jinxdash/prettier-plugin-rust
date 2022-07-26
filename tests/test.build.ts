import prettier from "prettier";
import { testBuilds } from "../ext/jinx-rust/scripts/utils/build";
import * as plugin_esm from "../index.js";
import * as plugin from "../src/index";

testBuilds(
	plugin,
	{
		esm: plugin_esm,
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
