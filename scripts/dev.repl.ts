import { createASTtoJSONPrinter, createPrettierPrinter, rs_createREPL } from "../ext/jinx-rust/scripts/utils";
import { plugin } from "../src/format/plugin";

rs_createREPL([
	createPrettierPrinter(
		{
			parser: "jinx-rust",
			plugins: [plugin],
		},
		true
	),
	createASTtoJSONPrinter(),
]);
