import {
	Attribute,
	AttributeOrDocComment,
	CallExpression,
	DelimKind,
	ExpressionNode,
	LocArray,
	MacroInvocation,
	MemberExpression,
	Node,
	NodeType,
	NodeWithBodyNoBody,
	NodeWithTypeBounds,
	NTMap,
	ProgramLike,
	rs,
	Snippet,
	StatementNode,
	StructLiteral,
	StructPattern,
	TK,
	TypeBound,
	TypeBoundsStandaloneNode,
	TypeDynBounds,
	TypeTraitBound,
} from "jinx-rust";
import {
	countActualNodeChildren,
	deleteAttributes,
	each_childNode,
	end,
	getActualNodeChildren,
	getBodyOrCases,
	getNodeChildren,
	hasAttributes,
	hasMethod,
	hasTypeBounds,
	includesTK,
	insertNode,
	insertNodes,
	is_AttributeOrDocComment,
	is_BareTypeTraitBound,
	is_BlockExpression,
	is_CallExpression,
	is_ClosureFunctionExpression,
	is_DocCommentAttribute,
	is_ExpressionStatement,
	is_ExpressionWithBodyOrCases,
	is_FlowControlExpression,
	is_Identifier,
	is_IfBlockExpression,
	is_MacroInvocation,
	is_Node,
	is_NodeWithBodyNoBody,
	is_NodeWithBodyOrCases,
	is_PathNode,
	is_Program,
	is_PunctuationToken,
	is_ReassignmentNode,
	is_Snippet,
	is_TypeBoundsStandaloneNode,
	is_TypeDynBounds,
	is_TypeImplBounds,
	is_TypeTraitBound,
	ownStart,
	reassignNodeProperty,
	start,
	transferAttributes,
	unsafe_set_nodeType,
} from "jinx-rust/utils";
import {
	Array_replace,
	Array_splice,
	assert,
	binarySearchIn,
	each,
	exit,
	iLast,
	last_of,
	Map_get,
	spliceAll,
	try_eval,
} from "../utils/common";
import { isPrettierIgnoreAttribute, setPrettierIgnoreTarget } from "./comments";
import { is_StructSpread } from "./core";
import { CustomOptions } from "./external";
import { getOptions } from "./plugin";

export interface ExpressionLikeAttribute extends Attribute {
	segments: LocArray<any, "[]">;
}

export interface CallLikeMacroInvocation extends MacroInvocation {
	segments: LocArray<any, any>;
	callee: MacroInvocation["callee"];
	method: undefined;
	typeArguments: undefined;
	arguments: LocArray<ExpressionNode, "()" | "[]" | "{}">;
}

export interface BlockLikeMacroInvocation extends MacroInvocation {
	segments: LocArray<any, any>;
	body: LocArray<StatementNode, "()" | "[]" | "{}">;
	attributes?: AttributeOrDocComment[];
}

export function is_CallLikeMacroInvocation(node: Node): node is CallLikeMacroInvocation {
	return is_MacroInvocation(node) && "arguments" in node;
}

export function is_BlockLikeMacroInvocation(node: Node): node is BlockLikeMacroInvocation {
	return is_MacroInvocation(node) && "body" in node;
}

export function is_CallExpression_or_CallLikeMacroInvocation(node: any): node is CallExpression | CallLikeMacroInvocation {
	return is_CallExpression(node) || is_CallLikeMacroInvocation(node);
}

const HARDCODED_MACRO_DELIMS = new Map<string, MacroInvocation["segments"]["dk"]>();
each(
	{
		[DelimKind["{}"]]: [
			// std
			"thread_local",
			// crates
			"cfg_if",
		],
		[DelimKind["()"]]: [
			// std
			"assert_eq",
			"assert_ne",
			"assert",
			"cfg",
			"concat_bytes",
			"concat_idents",
			"concat",
			"debug_assert_eq",
			"debug_assert_ne",
			"debug_assert",
			"eprint",
			"eprintln",
			"format_args_nl",
			"format_args",
			"format",
			"matches",
			"panic",
			"print",
			"println",
			"try",
			"unimplemented",
			"unreachable",
			"write",
			"writeln",
			// crates
		],
		[DelimKind["[]"]]: [
			// std
			"vec",
			// crates
		],
	},
	(names, tk) =>
		each(names, (name) => {
			HARDCODED_MACRO_DELIMS.set(name, +tk as MacroInvocation["segments"]["dk"]);
		})
);

const IGNORED_MACROS = new Set([
	// std
	// crates
	"cfg_if",
	"quote",
]);

let DANGLING_ATTRIBUTES: CustomOptions["danglingAttributes"] = undefined!;
let COMMENTS: CustomOptions["comments"] = undefined!;

export function transform_ast(options: CustomOptions) {
	try {
		COMMENTS = options.comments;
		DANGLING_ATTRIBUTES = options.danglingAttributes;
		transformNode(options.rsParsedFile);
	} finally {
		_depth = 0;
		COMMENTS = undefined!;
		DANGLING_ATTRIBUTES = undefined!;
	}
}

let _depth = 0;
const isReadingSnippet = () => 0 !== _depth;

function maybe_transform_node<T extends Node, S extends Snippet>(
	node: T,
	read_snippet: () => S,
	fn: (node: T, snippet: S) => void
): T | undefined {
	const snippet = try_eval(read_snippet);
	if (snippet) {
		++_depth;
		transformNode(snippet);
		--_depth;
		fn(node, snippet);
		transformed.add(node);
		return node;
	}
}
const transformed = new WeakSet<Node>();
export function isTransformed(node: Node) {
	return transformed.has(node);
}

const transform: { [K in NodeType]?: (node: NTMap[K]) => void } = {
	// [NodeType.Snippet](node) {
	// 	registerCommentsAndDanglingAttributes(node);
	// },
	// [NodeType.Program](node) {
	// 	registerCommentsAndDanglingAttributes(node);
	// },
	[NodeType.Attribute](node) {
		maybe_transform_node(
			node as ExpressionLikeAttribute,
			() => rs.toCallExpressionArguments(node.segments),
			(node, snippet) => {
				node.segments = snippet.ast;
			}
		);
	},
	[NodeType.MacroInlineRuleDeclaration](node) {
		node.match.dk = DelimKind["()"];
		node.transform.dk = DelimKind["{}"];
	},
	[NodeType.MacroInvocation](node) {
		const name = getIdentifierName(node.callee);

		if (
			IGNORED_MACROS.has(name) ||
			node.segments.length === 0 ||
			(node.segments.length === 1 && is_PunctuationToken(node.segments[0]))
		) {
			return;
		}

		const tk = transformMacroDelim(name, node);

		if (name === "matches") {
			//
		}

		if (name === "if_chain") {
			//
		}

		if (name === "cfg_if") {
			//
		}

		if (tk === DelimKind["{}"]) {
			transformBlockLike(); /* || (includesTK(node, TK[","]) && transformCallLike()); */
		} else {
			transformCallLike(); /* || (includesTK(node, TK[";"]) && transformBlockLike()); */
		}

		function transformBlockLike() {
			return maybe_transform_node(
				node as BlockLikeMacroInvocation,
				() => rs.toBlockBody(node.segments),
				(node, snippet) => {
					const _body = snippet.ast;
					_body.dk = tk;

					node.body = _body;
					node.segments = _body;
					transferAttributes(snippet, node);
				}
			);
		}

		function transformCallLike() {
			return maybe_transform_node(
				node as CallLikeMacroInvocation,
				() => rs.toCallExpressionArguments(node.segments),
				(node, snippet) => {
					const _arguments = snippet.ast;
					_arguments.dk = tk;

					node.method = undefined;
					node.typeArguments = undefined;
					node.arguments = _arguments;
					node.segments = _arguments;
				}
			);
		}
		function getIdentifierName(node: any) {
			return is_Identifier(node) ? node.name : is_PathNode(node) ? node.segment.name : "";
		}
	},
	[NodeType.CallExpression](node) {
		if (hasMethod(node)) {
			node.callee = rs.mockNode(NodeType.MemberExpression, node.method.loc.cloneFrom(start(node.callee)), {
				expression: node.callee,
				property: node.method,
				computed: false,
			});
			node.method = undefined!;
			getOptions().actuallyMethodNodes.add(node.callee as MemberExpression);
		}
	},

	[NodeType.AutoTraitDeclaration](node) {
		mockBodyNoBody(node);
	},
	[NodeType.NegativeImplDeclaration](node) {
		mockBodyNoBody(node);
	},

	[NodeType.StructLiteral](node) {
		moveSpreadsToEnd(node);
	},
	[NodeType.StructPattern](node) {
		moveSpreadsToEnd(node);
	},
};

function moveSpreadsToEnd(node: StructLiteral | StructPattern) {
	const props = node.properties;
	if (props.some((p, i, a) => is_StructSpread(p) && !iLast(i, a))) {
		const spreads: any[] = [];
		for (let i = 0; i < props.length; i++) {
			const prop = props[i];
			if (is_StructSpread(prop)) {
				Array_splice(props, prop, i--);
				spreads.push(prop);
			}
		}
		props.push(...spreads);
	}
}

function mockBodyNoBody(node: NodeWithBodyNoBody) {
	// @ts-expect-error
	node.body = rs.createLocArray(last_of(rs.toTokens(node).ast).loc.clone(), DelimKind["{}"]);
}

function transformMacroDelim(name: string, node: MacroInvocation): 1 | 2 | 3 {
	if (HARDCODED_MACRO_DELIMS.has(name)) {
		return HARDCODED_MACRO_DELIMS.get(name)!;
	}
	if (node.segments.dk === DelimKind["{}"] && includesTK(node, TK[","])) {
		return DelimKind["()"];
	}
	if (node.segments.dk === DelimKind["()"] && includesTK(node, TK[";"])) {
		return DelimKind["{}"];
	}
	return node.segments.dk;
}

function transformNode(node: Node, parent?: Node, key?: string, index?: any) {
	if (is_Snippet(node) || is_Program(node)) {
		registerCommentsAndDanglingAttributes(node);
	}

	each_childNode(node, transformNode);

	insert_blocks(parent, key, node, index);

	transform[node.nodeType]?.(node as any);

	flatten_typeBounds(node);

	transform_nodeAttributes(node);
}

function insert_blocks(parent: Node | undefined, key: string | undefined, node: Node, index: any) {
	if (parent && key) {
		if (
			!is_ExpressionStatement(parent) &&
			(false ||
				// "1 + break" -> "1 + { break; }"
				is_FlowControlExpression(node) ||
				// "1 + a = b" -> "1 + { a = b; }"
				(!isReadingSnippet() && is_ReassignmentNode(node) && !(is_ReassignmentNode(parent) && parent.left === node)))
		) {
			reassignNodeProperty(blockify(node), parent, key, index);
		} else if (
			is_ClosureFunctionExpression(node) &&
			(false ||
				// "|| -> T x" -> "|| -> T { x }"
				(!!node.returnType && !is_BlockExpression(node.expression)) ||
				// "|| match x {}" -> "|| { match x {} }"
				(is_ExpressionWithBodyOrCases(node.expression) &&
					!is_BlockExpression(node.expression) &&
					!is_IfBlockExpression(node.expression)))
		) {
			node.expression = blockify(node.expression);
		}
	}
	function blockify(node: ExpressionNode) {
		const block = rs.mockNode(NodeType.BlockExpression, node.loc.clone(), {
			label: undefined,
			body: rs.createLocArray(DelimKind["{}"], node.loc.clone(), [
				rs.mockNode(NodeType.ExpressionStatement, node.loc.clone(), { semi: false, expression: node }),
			]),
		});
		transferAttributes(node, block);
		return block;
	}
}

function flatten_typeBounds(topNode: Node) {
	if (hasTypeBounds(topNode)) {
		const nestedBounds: TypeTraitBound[] = topNode.typeBounds.filter(isBoundWithNestedBounds);
		const [first, ...subsequent] = nestedBounds;

		const flatten = (bound: TypeTraitBound) =>
			Array_replace(topNode.typeBounds, bound, ...(bound.typeExpression as unknown as TypeDynBounds).typeBounds);

		if (nestedBounds.every(isBareBoundWithNestedBoundsNoPrefix)) {
			// A + (B + C)
			// -> A + B + C
			each(nestedBounds, flatten);
		} else if (
			!hasDefinedPrefix(topNode) &&
			first === topNode.typeBounds[0] &&
			!isBareBoundWithNestedBoundsNoPrefix(first) &&
			subsequent.every(isBareBoundWithNestedBoundsNoPrefix)
		) {
			if (is_TypeDynBounds(topNode)) {
				if (is_TypeImplBounds(first.typeExpression)) {
					// (impl A) + B
					// -> impl A + B
					unsafe_set_nodeType(topNode, NodeType.TypeImplBounds);
				} else {
					// (dyn A) + B
					// -> dyn A + B
					topNode.dyn = true;
				}
				each(nestedBounds, flatten);
			} else {
				each(subsequent, flatten);
				(first.typeExpression as unknown as TypeDynBounds).typeBounds.push(...topNode.typeBounds.slice(1));
				topNode.typeBounds.length = 1;
			}
		}
	}

	function isBoundWithNestedBounds(bound: TypeBound): bound is TypeTraitBound & { typeExpression: TypeBoundsStandaloneNode } {
		return is_TypeTraitBound(bound) && is_TypeBoundsStandaloneNode(bound.typeExpression);
	}
	function isBareBoundWithNestedBounds(bound: TypeBound): bound is TypeTraitBound & { typeExpression: TypeBoundsStandaloneNode } {
		return isBoundWithNestedBounds(bound) && is_BareTypeTraitBound(bound);
	}
	function isBareBoundWithNestedBoundsNoPrefix(bound: TypeBound): bound is TypeTraitBound & { typeExpression: TypeDynBounds } {
		return isBareBoundWithNestedBounds(bound) && !hasDefinedPrefix(bound.typeExpression);
	}
	function hasDefinedPrefix(node: NodeWithTypeBounds) {
		return (is_TypeDynBounds(node) && node.dyn) || is_TypeImplBounds(node);
	}
}

function transform_nodeAttributes(node: Node) {
	/**
	 * # Inside Token trees:
	 *
	 *  1. DocCommentAttribute 	--is parsed as--> 	Comment
	 *  2. Attribute 			--is parsed as--> 	Token<'#'>, DelimGroup<'[]'>
	 *
	 * # Transforming tokens into a Snippet:
	 *
	 *  1. DocCommentAttribute <--replace from-- Comment
	 *    a) Remove node with same loc from comments
	 *    b) Merge Snippet.danglingAttributes with Program.danglingAttributes
	 *
	 *  2. Attribute (no action needed)
	 *
	 */
	if (hasAttributes(node)) {
		const attrs = node.attributes;
		for (let i = 0; i < attrs.length; i++) {
			const attr = attrs[i];
			if (isReadingSnippet() && is_DocCommentAttribute(attr)) {
				const index = binarySearchIn(COMMENTS, start(attr), start);
				__DEV__: assert(index !== -1), assert(end(COMMENTS[index]) === end(attr));
				COMMENTS.splice(index, 1);
			}
			if (attr.inner) {
				if (isPrettierIgnoreAttribute(attr)) {
					setPrettierIgnoreTarget(is_Program(node) ? node.loc.src : node, attr);
				}
				// @ts-expect-error Inserting Attribute into StatementNode[]
				insertNode(is_Snippet(node) ? node.ast : getBodyOrCases(node)!, attr);
				Array_splice(attrs, attr, i--);
			}
		}
		if (attrs.length === 0) {
			deleteAttributes(node);
		}
	}
}

function registerCommentsAndDanglingAttributes(program: Extract<Node, ProgramLike>) {
	const comments = spliceAll(program.comments);
	const danglingAttributes = spliceAll(program.danglingAttributes);
	for (let i = 0; i < danglingAttributes.length; i++) transformNode(danglingAttributes[i], program, "danglingAttributes", i);
	if (!isReadingSnippet()) insertNodes(COMMENTS, comments);
	insertNodes(DANGLING_ATTRIBUTES, danglingAttributes);
}

const CommentChildNodes = new WeakMap<Node, Node[]>();

export function getCommentChildNodes(n: any): Node[] {
	const children = Map_get(CommentChildNodes, n, getTransformedNodeChildren);
	/**
	 * parent {
	 *  #[attr]
	 * 	#![attr] <-------- list misplaced inner attrs as part of "#[attr] child {}"
	 * 	child {}
	 * }
	 */
	if (is_NodeWithBodyOrCases(n) || is_BlockLikeMacroInvocation(n)) {
		for (let i = 0; i < children.length; i++) {
			const attr = children[i];
			if (is_AttributeOrDocComment(attr)) {
				const target = children.find((n) => start(n) <= start(attr) && ownStart(n) >= end(attr));
				if (target) {
					children.splice(i--, 1);
					insertNode(Map_get(CommentChildNodes, target, getTransformedNodeChildren), attr);
				}
			}
		}
	}
	return children;

	function getTransformedNodeChildren(node: Node) {
		if (is_Program(node)) node.comments ??= []; // prettier deletes it
		const children = getNodeChildren(node);

		if (is_NodeWithBodyNoBody(node)) {
			insertNodes(children, (node as any).body);
		}

		__DEV__: {
			const actual_count = countActualNodeChildren(node);
			if (
				children.length !== actual_count &&
				!(is_MacroInvocation(node) && actual_count - node.segments.length === children.length)
			) {
				const actual = getActualNodeChildren(node);
				const missing = actual.filter((n) => !children.includes(n));
				const unknown = children.filter((n) => !actual.includes(n));
				const duplicates_in_object = actual.filter((n, i, a) => i !== 0 && n === a[i - 1]);
				const duplicates_in_childNodes = children.filter((n, i, a) => i !== 0 && n === a[i - 1]);
				const ctx = { missing, unknown, duplicates_in_object, duplicates_in_childNodes };
				for (let key in ctx) if (ctx[key].length === 0) delete ctx[key];
				exit(`${node.type} was transformed but did not patch its childNodes list`, ctx, node.loc.url(), node);
			}
			for (const child of children)
				if (!is_Node(child)) exit(`${node.type}'s childNodes includes unexpected entries`, { node, child });
		}
		return children;
	}
}
