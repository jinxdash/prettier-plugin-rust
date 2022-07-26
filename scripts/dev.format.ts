import prettier from "prettier";
import { for_each_rs_file } from "../ext/jinx-rust/scripts/utils/common";
import { update_file } from "../ext/jinx-rust/scripts/utils/fs";
import { plugin } from "../src/format/plugin";

function format(code: string, filepath: string) {
	const next = prettier.format(code, {
		parser: "jinx-rust",
		plugins: [plugin],
		printWidth: 100,
		tabWidth: 4,
		filepath,
	});
	if (code !== next && code.trim() !== next.trim()) {
		update_file(filepath, next, { external: true });
	}
}

globalThis.TESTS_FORMAT_DEV = true;

for_each_rs_file(
	[`E:/dev/github/rust/rust-lang/`],
	(file) => {
		format(file.content, file.cmd);
	},
	["test", "tests"]
).then(() => {
	globalThis.TESTS_FORMAT_DEV = false;
});
