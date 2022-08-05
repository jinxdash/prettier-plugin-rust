import { rs } from "jinx-rust";
import path from "node:path";
import prettier, { Config, resolveConfig } from "prettier";
import * as plugin from "prettier-plugin-rust";
import { ExtensionContext, languages, Position, Range, TextDocument, TextEdit, window } from "vscode";

const console = createOutputChannel("Prettier (Rust)");

console.log(`VSCode Extension: ${process.env.EXTENSION_NAME || "jinxdash.prettier-rust"}@${process.env.EXTENSION_VERSION || "dev"}`);
export async function activate(context: ExtensionContext) {
	context.subscriptions.push(
		languages.registerDocumentFormattingEditProvider("rust", {
			async provideDocumentFormattingEdits(document) {
				const config: Config = {
					filepath: cmd(document.fileName),
					...((await resolveConfig(document.fileName, { editorconfig: true, useCache: false })) ?? {}),
				};
				console.log("", `# Formatting using prettier@${prettier.version}`, {
					...config,
					plugins: config.plugins?.flatMap((p) => (typeof p === "string" ? p : p.printers && Object.keys(p.printers))),
				});
				if (
					!config.plugins?.some((plugin) => typeof plugin === "object" && plugin.languages?.some((lang) => lang.name === "Rust"))
				) {
					config.parser = "jinx-rust";
					config.plugins ??= [plugin];
				}
				return format(document, config);
			},
		})
	);
}

function format(document: TextDocument, config: Config) {
	// length of common prefix
	const next = tryFormat(document, config);
	const prev = document.getText();
	if (prev === next) {
		// console.log("No changes");
		return [];
	}
	const end = Math.min(prev.length, next.length);
	var i = 0;
	var j = 0;
	for (var i = 0; i < end && compare(i, i); ++i);
	for (var j = 0; i + j < end && compare(prev.length - j - 1, next.length - j - 1); ++j);
	// console.log([i, j]);
	return [TextEdit.replace(new Range(document.positionAt(i), document.positionAt(prev.length - j)), next.substring(i, next.length - j))];
	function compare(i: number, j: number) {
		return prev.charCodeAt(i) === next.charCodeAt(j);
	}
}

function tryFormat(doc: TextDocument, config: prettier.Config) {
	try {
		return prettier.format(doc.getText(), config);
	} catch (e) {
		if ((e as any).loc) {
			try {
				rs.parseFile(doc.getText(), { filepath: config.filepath });
			} catch (_e) {
				const e2 = _e as rs.ParserError;
				const pos = new Position(e2.loc.start.line - 1, e2.loc.start.column - 1);
				window.showTextDocument(doc, { selection: new Range(pos, pos) });
				window.showErrorMessage(e2.message);
			}
		} else {
			console.log(e);
		}
		return doc.getText();
	}
}

function createOutputChannel(name: string) {
	const out = window.createOutputChannel(name);
	return {
		log(...arr: any[]) {
			for (var item of arr) {
				if (typeof item === "string") {
					out.appendLine(unstyle(item));
				} else if (item instanceof Error) {
					if (item?.message) out.appendLine(unstyle(item.message));
					if (item?.stack) out.appendLine(unstyle(item.stack));
				} else {
					out.appendLine(JSON.stringify(item, null, 2));
				}
			}
		},
	};
}
function unstyle(str: string) {
	return str.replace(/\x1B\[[0-9][0-9]?m/g, "");
}

function cmd(filepath: string | undefined, frompath = "") {
	return normPath(path.relative(frompath, normPath(filepath ?? ""))) || ".";
}
function normPath(filepath: string) {
	return filepath.replace(/^file:\/\/\//, "").replace(/\\\\?/g, "/");
}
