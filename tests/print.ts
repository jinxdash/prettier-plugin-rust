import { createPrettierPrinter, rs_print_samples } from "../ext/jinx-rust/scripts/utils";
import plugin from "../src/index";

// for_each_ts_file(path.resolve("src"), (file) => {
// 	console.log(cmd(file.path), file.content.includes("\r"));
// 	update_file(file.path, file.content.replace(/\r/g, ""), { force: true, sync: true, prettier: false });
// });
const printer = createPrettierPrinter(
	{
		parser: "jinx-rust",
		plugins: [plugin],
		printWidth: 80,
		tabWidth: 2,
	},
	false
);
rs_print_samples(["tests/samples/"], "tests/output/", [printer]);
rs_print_samples(["ext/jinx-rust/tests/samples"], "tests/output-ext/", [printer]);
