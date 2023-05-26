import { AttributeOrComment, IfBlockExpression, Node, Program, rs } from "jinx-rust";
import {
	ArrayProps,
	BoolProps,
	NodeProps,
	end,
	hasAttributes,
	insertNodes,
	is_Attribute,
	is_BlockCommentKind,
	is_BlockCommentNode,
	is_Comment,
	is_DocCommentAttribute,
	is_ElseBlock,
	is_LineCommentNode,
	is_MacroInvocation,
	is_MacroRule,
	is_MissingNode,
	is_Node,
	is_PunctuationToken,
	is_UnionPattern,
	start,
} from "jinx-rust/utils";
import { getCommentChildNodes, isTransformed, transform_ast } from "../transform";
import { Narrow, assert, color, each, exit, iLast, is_array, map_tagged_template, print_string } from "../utils/common";
import {
	CF,
	escapeComments,
	getComments,
	handleEndOfLineComment,
	handleOwnLineComment,
	handleRemainingComment,
	hasBreaklineAfter,
	hasComment,
	isDangling,
	isPrettierIgnoreAttribute,
	setDidPrintComment,
	withComments,
} from "./comments";
import { withCheckContext } from "./complexity";
import { isNoopExpressionStatement, maybeEmptyLine } from "./core";
import { AstPath, CustomOptions, Doc, Plugin, Symbol_comments, group, hardline, indent, line, softline } from "./external";
import { printer } from "./printer";
import { needsInnerParens, needsOuterSoftbreakParens, shouldPrintOuterAttributesAbove } from "./styling";

export function is_printing_macro() {
	return getContext().path.stack.some((node) => is_Node(node) && (is_MacroInvocation(node) || is_Attribute(node)));
}

export function assertPathAtNode(name: string, node: Node, ...ctx: any[]) {
	__DEV__: if (getNode() !== node)
		exit(`Attempted to call ${name}() in wrong prettier path context`, { asserted: node, actual: getNode() }, ...ctx);
}

export function f(...args: [strings: TemplateStringsArray, ...values: Doc[]]) {
	let cancel = false;
	const res = map_tagged_template(args, (doc) => {
		cancel ||= !doc || (is_array(doc) && doc.length === 0);
		return doc;
	});
	return cancel ? "" : res;
}

export function sg_single(s: TemplateStringsArray, v_0: Doc) {
	return group([s[0], indent([softline, v_0]), softline, s[1]]);
}
export function sg_duo(s: TemplateStringsArray, v_0: Doc, v_1: Doc) {
	return group([s[0], indent([softline, v_0, s[1], line, v_1]), softline, s[2]]);
}

let ctx: {
	path: AstPath;
	options: CustomOptions;
	print: (path?: AstPath | string | [], args?: any) => Doc;
	args: any;
};

export const getNode = () => ctx.path.stack[ctx.path.stack.length - 1] as Node;
export const stackIncludes = (x: Node | string | number) => ctx.path.stack.includes(x);
export const getContext = () => ctx;
export const getOptions = () => ctx.options;
export const getProgram = () => ctx.options.rsParsedFile.program;
export const getAllComments = () => ctx.options[Symbol_comments];
export const getParentNode = (child?: Node) => {
	__DEV__: if (child) assertPathAtNode("getParentNode", child);
	return ctx.path.getParentNode();
};
export const getGrandParentNode = () => ctx.path.getParentNode(1) as Node;
export const getPrintFn = <T extends Node>(forNode?: T | undefined): print<T> => {
	__DEV__: if (forNode) assertPathAtNode("getPrintFn", forNode);
	return print as print<T>;
};

const get = (property: keyof any) => getNode()[property];
const has = (property: keyof any) => !!get(property);

export function pathCall<T extends Node, K extends keyof NodeProps<T> & keyof T, R>(node: T, key: K, fn: (child: T[K]) => R): R {
	return ctx.path.call(() => fn(getNode() as any), key as any);
}

export function pathCallEach<T extends Node, K extends AK<T>>(
	node: T,
	key: K, // @ts-expect-error
	fn: (child: NonNullable<T[K]>[number], index: number) => void
) {
	__DEV__: assertPathAtNode("", node); // @ts-expect-error
	ctx.path.each((_, i) => fn(getNode() as any, i), key);
}

export function pathCallAtParent<T extends Node, R>(parent: T, fn: (parent: T) => R): R {
	return ctx.path.callParent(() => {
		__DEV__: assertPathAtNode("pathCallParent", parent);
		return fn(parent);
	});
}
export function pathCallParentOf<T extends Node, R>(child: Node, fn: (parent: T) => R): R {
	__DEV__: assertPathAtNode("pathCallParentOf", child);
	return ctx.path.callParent((p) => fn(getNode() as any));
}

export function pathCallTopMostIfBlockExpression<R>(node: IfBlockExpression, fn: (node: IfBlockExpression) => R): R {
	const parent = getParentNode(node)!;
	return is_ElseBlock(node, parent) ? pathCallAtParent(parent, (parent) => pathCallTopMostIfBlockExpression(parent, fn)) : fn(node);
}

function print(property?: any, args?: any): Doc | Doc[] {
	if (!property) return ctx.print(undefined!, args);
	if (Array.isArray(property)) return ctx.print(property as any, args);
	const value = get(property);
	return !!value ? (Array.isArray(value) ? ctx.path.map(ctx.print, property) : ctx.print(property, args)) : "";
}

namespace print {
	export function b(property: string, res = `${property} `): Doc {
		return has(property) ? res : "";
	}
	export function map(property: string, mapItem?: MapFn<any, any>): Doc[] {
		return !has(property) ? [] : ctx.path.map(mapItem ? (p, i, a) => mapItem(a[i], i, a) : () => ctx.print(), property);
	}
	export function join(property: string, sep: SepFn<any, any> | Doc, trailingSep: TrailingSepFn<any, any> | Doc = ""): Doc[] {
		return map_join(property, () => ctx.print(), sep, trailingSep);
	}
	export function map_join(
		property: string,
		mapFn: MapFn<any, any>,
		sep: SepFn<any, any> | Doc,
		sepTrailing: TrailingSepFn<any, any> | Doc = ""
	) {
		const sepFn = typeof sep === "function" ? sep : () => sep;
		return map(property, (v, i, a) => [
			mapFn(v, i, a),
			iLast(i, a as any)
				? typeof sepTrailing === "function"
					? sepTrailing(v)
					: sepTrailing
				: sepFn(v, a[i + 1], i === 0 ? undefined : a[i - 1]),
		]);
	}
}

// prettier-ignore
type SepFn<T extends Node = Node, K extends AK<T> = AK<T>> = <A extends AV<T, K>>(item: A[number], next_item: A[number], prev_item: A[number] | undefined) => Doc;
type MapFn<T extends Node = Node, K extends AK<T> = AK<T>> = <A extends AV<T, K>>(item: A[number], index: number, arr: A) => Doc;
type TrailingSepFn<T extends Node = Node, K extends AK<T> = AK<T>> = <A extends AV<T, K>>(item: A[number]) => Doc;
type AV<T extends Node, K extends keyof T> = Extract<NonNullable<T[K]>, ReadonlyArray<unknown>>;
type AK<T extends Node> = keyof ArrayProps<T> & keyof T;
// type AK<T extends Node> = keyof PickProps<T, {nodeType:number}|{nodeType:number}[]> & keyof T;

export interface print<T extends Node> {
	(property?: [], args?: any): Doc;
	(property?: [AK<T>, number], args?: any): Doc;
	(property?: AK<T>, args?: any): Doc[];
	// (property?: T extends {rules:{nodeType:number}|{nodeType:number}[]} ? "rules" : never, args?: any): Doc[];
	(property?: keyof NodeProps<T> & keyof T, args?: any): Doc;
	b(property: keyof BoolProps<T>, res?: string): Doc;
	map<K extends AK<T>>(property: K & keyof ArrayProps<T>, mapFn?: MapFn<T, K>): Doc[];
	join<K extends AK<T>>(property: K, sep: SepFn<T, K> | Doc, trailingSep?: TrailingSepFn<T, K> | Doc): Doc[];
	map_join<K extends AK<T>>(property: K, mapFn: MapFn<T, K>, sep: SepFn<T, K> | Doc, trailingSep?: TrailingSepFn<T, K> | Doc): Doc[];
}

function genericPrint() {
	return withCheckContext(() => {
		const node = getNode();
		__DEV__: assert(node.nodeType in printer);

		let printed: Doc = hasPrettierIgnore(node) //
			? node.loc.getOwnText()
			: printer[node.nodeType]!(print as any, node as never);

		const inner_parens = needsInnerParens(node);

		if (inner_parens) {
			printed = group(["(", printed, ")"]);
		}

		if (hasAttributes(node)) {
			const print_above = shouldPrintOuterAttributesAbove(node); /*  || node.attributes.length > 1 */
			printed = [
				...print.join(
					"attributes",
					(attr) =>
						print_above
							? maybeEmptyLine(attr)
							: is_LineCommentNode(attr) || (is_BlockCommentNode(attr) && hasBreaklineAfter(attr))
							? hardline
							: " ",
					(attr) =>
						print_above && is_DocCommentAttribute(attr)
							? maybeEmptyLine(attr)
							: print_above || is_LineCommentNode(attr) || (is_BlockCommentNode(attr) && hasBreaklineAfter(attr))
							? hardline
							: " "
				),
				printed,
			];
		}

		printed = withComments(
			node,
			printed,
			hasPrettierIgnore(node) || ((is_Attribute(node) || is_MacroInvocation(node)) && !isTransformed(node))
				? escapeComments(0, (comment) => node.loc.ownContains(comment))
				: is_MacroRule(node)
				? escapeComments(0, (comment) => node.transform.loc.contains(comment))
				: is_UnionPattern(getParentNode() ?? ({ nodeType: 0 } as any))
				? new Set(getComments(node, 0, (comment) => !isDangling(comment)))
				: undefined
		);

		if (!inner_parens && needsOuterSoftbreakParens(node)) {
			printed = [group(["(", indent([softline, printed]), softline, ")"])];
		}

		return printed;
	});

	function hasPrettierIgnore(node: Node) {
		return (
			(node as any).prettierIgnore ||
			hasComment(node, CF.PrettierIgnore) ||
			(hasAttributes(node) && node.attributes.some(isPrettierIgnoreAttribute))
		);
	}
}

export function canAttachComment(n: Node) {
	return !is_Comment(n) && !isNoopExpressionStatement(n) && !is_MissingNode(n) && !is_PunctuationToken(n);
}

export const plugin: Plugin<Node> = {
	languages: [
		{
			name: "Rust",
			aliases: ["rs"],
			parsers: ["jinx-rust"],
			extensions: [".rs", ".rs.in"],
			linguistLanguageId: 327,
			vscodeLanguageIds: ["rust"],
			tmScope: "source.rust",
			aceMode: "rust",
			codemirrorMode: "rust",
			codemirrorMimeType: "text/x-rustsrc",
		},
	],
	parsers: {
		"jinx-rust": {
			astFormat: "jinx-rust",
			locStart: start,
			locEnd: end,
			parse(code, parsers, options: CustomOptions) {
				ctx = { options } as any;

				options.rsParsedFile = rs.parseFile((options.originalText = code), { filepath: options.filepath });

				options.actuallyMethodNodes = new WeakSet();
				options.danglingAttributes = [];
				options.comments = [];

				transform_ast(options);

				const comments: AttributeOrComment[] = [];
				insertNodes(comments, options.comments);
				insertNodes(comments, options.danglingAttributes);

				// @ts-expect-error
				options.rsParsedFile.program.comments = comments;

				options.commentSpans = new Map(comments.map((n) => [start(n), end(n)]));

				return options.rsParsedFile.program;
			},
		},
	},
	printers: {
		"jinx-rust": {
			preprocess: (node: Program) => node.loc.src,
			print(path, options, print, args) {
				if (path.stack.length === 1) {
					__DEV__: Narrow<CustomOptions>(options);
					ctx = { path, options, print, args };
					try {
						const printed = genericPrint();
						__DEV__: devEndCheck(printed);
						return printed;
					} finally {
						ctx = undefined!;
					}
				} else if (args || ctx.args) {
					const prev_args = ctx.args;
					try {
						ctx.args = args;
						return genericPrint();
					} finally {
						ctx.args = prev_args;
					}
				} else {
					return genericPrint();
				}
			},
			hasPrettierIgnore: () => false,
			willPrintOwnComments: () => true,
			isBlockComment: is_BlockCommentKind,
			canAttachComment: canAttachComment,
			getCommentChildNodes: getCommentChildNodes,
			printComment: genericPrint,
			handleComments: {
				// @ts-expect-error
				avoidAstMutation: true,
				ownLine: handleOwnLineComment,
				endOfLine: handleEndOfLineComment,
				remaining: handleRemainingComment,
			},
		},
	},
	options: {},
	defaultOptions: {
		// default prettier (2)  -> rustfmt (4)
		tabWidth: 4,
		// default prettier (80) -> rustfmt (100)
		printWidth: 100,
	},
};

function devEndCheck(printed: Doc) {
	let first = false;
	const comments = getAllComments();
	each(comments, (comment, index) => {
		if (!comment.printed) {
			if (!first) (first = true), console.log(color.red(`Unprinted comments:`));
			const len = 40;
			const msg =
				color.magenta(
					(comment.marker ? `Dangling "${comment.marker}" ` : "") +
						(is_Attribute(comment) ? "Attribute " : is_DocCommentAttribute(comment) ? "DocCommentAttribute" : "Comment") +
						` ${index}/${comments.length}` +
						color.yellow(` ${print_string(comment.loc.sliceText(0, len))}${comment.loc.len() > len ? " ..." : ""}`)
				) + color.grey(`\n    at ${comment.loc.url()}`);
			if (globalThis.TESTS_FORMAT_DEV) exit(msg);
			else console.log(msg);
			setDidPrintComment(comment);
		}
	});
}
