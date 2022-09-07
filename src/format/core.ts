import {
	AndExpression,
	AttributeOrDocComment,
	BreakExpression,
	CallExpression,
	ClosureFunctionExpression,
	ComparisonExpression,
	DeclarationNode,
	DelimKind,
	EnumDeclaration,
	ExpressionBody,
	ExpressionNode,
	ExpressionPath,
	ExpressionWithBody,
	ForLtParametersBody,
	FunctionDeclaration,
	FunctionLike,
	FunctionNode,
	IfBlockExpression,
	ImplDeclaration,
	LeftRightExpression,
	LocArray,
	MacroDeclaration,
	MacroGroup,
	MacroInlineRuleDeclaration,
	MacroMatchSegment,
	MacroRuleDeclaration,
	MacroRulesDeclaration,
	MatchExpression,
	MatchExpressionCase,
	MaybeBlockBody,
	MaybeGenericArgsTarget,
	MaybeHasLtBounds,
	MaybeReturnTypeConstraint,
	MaybeTypeAnnotationTarget,
	MemberExpression,
	NegativeImplDeclaration,
	Node,
	NodeType,
	NodeWithBodyOrCases,
	NodeWithCondition,
	ObjectNode,
	OperationExpression,
	OrExpression,
	PathNode,
	PatternBody,
	PatternNode,
	PunctuationToken,
	RangeNode,
	RestPattern,
	ReturnExpression,
	StructDeclaration,
	StructLiteral,
	StructLiteralPropertySpread,
	StructLiteralRestUnassigned,
	TK,
	TraitAliasDeclaration,
	TraitDeclaration,
	TupleStructDeclaration,
	TypeAliasDeclaration,
	TypeBoundsConstaint,
	TypeFunctionNode,
	UnaryExpression,
	UnionDeclaration,
	UnionPattern,
	WhileBlockExpression,
	YieldExpression,
} from "jinx-rust";
import {
	DelimChars,
	end,
	getBodyOrCases,
	getDelimChars,
	getParameters,
	hasAttributes,
	hasCondition,
	hasItems,
	hasLetScrutineeCondition,
	hasParameters,
	hasProperties,
	hasSelfParameter,
	hasSemiNoBody,
	hasSemiNoProperties,
	hasTypeBounds,
	is_ArrayOrTupleLiteral,
	is_Attribute,
	is_ClosureBlock,
	is_ClosureFunctionExpression,
	is_DelimGroup,
	is_ElseBlock,
	is_EnumMemberStructDeclaration,
	is_ExpressionAsTypeCast,
	is_ExpressionNode,
	is_ExpressionPath,
	is_ExpressionStatement,
	is_ExpressionTypeCast,
	is_ExpressionWithBodyOrCases,
	is_FlowControlExpression,
	is_FunctionDeclaration,
	is_FunctionParameterDeclaration,
	is_GenericParameterDeclaration,
	is_Identifier,
	is_IdentifierOrIndex,
	is_IfBlockExpression,
	is_ImplDeclarationNode,
	is_LetScrutinee,
	is_LineCommentNode,
	is_Literal,
	is_LiteralBooleanLike,
	is_LiteralNumberLike,
	is_LiteralStringLike,
	is_LogicalExpression,
	is_MacroGroup,
	is_MacroInlineRuleDeclaration,
	is_MacroInvocation,
	is_MacroParameterDeclaration,
	is_MatchExpression,
	is_MatchExpressionCase,
	is_MemberExpression,
	is_MinusExpression,
	is_MissingNode,
	is_NodeWithBodyOrCases,
	is_OrExpression,
	is_PostfixExpression,
	is_Program,
	is_PunctuationToken,
	is_RangeLiteral,
	is_RangePattern,
	is_ReassignmentExpression,
	is_ReassignmentNode,
	is_RestPattern,
	is_StructDeclaration,
	is_StructLiteral,
	is_StructLiteralProperty,
	is_StructLiteralPropertySpread,
	is_StructPattern,
	is_StructPatternPropertyDestructured,
	is_StructProperty,
	is_TupleLiteral,
	is_TupleNode,
	is_TuplePattern,
	is_TupleStructDeclaration,
	is_TypeCallNamedArgument,
	is_TypeTuple,
	is_UnaryExpression,
	is_UnionDeclaration,
	is_UnwrapExpression,
	is_VariableDeclarationNode,
	ownStart,
	start,
} from "jinx-rust/utils";
import { assert, AssertTypesEq, exit, find_last, flat, Identity, iLast, last_of, Map_get, Narrow, spread } from "../utils/common";
import {
	CF,
	getFirstComment,
	getNextNonSpaceNonCommentCharacterIndex,
	hasBreaklineBefore,
	hasComment,
	hasNewline,
	hasNewlineInRange,
	isNextLineEmpty,
	isNextLineEmptyAfterIndex,
	isPreviousLineEmpty,
	printCommentsSeparately,
	printDanglingComments,
	withComments,
} from "./comments";
import {
	hasComplexGenerics,
	hasComplexLtParameters,
	hasComplexTypeAnnotation,
	hasComplexTypeArguments,
	hasComplexTypeBounds,
	isShortGenericParameterDeclaration,
	is_short,
} from "./complexity";
import {
	align,
	AstPath,
	breakParent,
	canBreak,
	cleanDoc,
	conditionalGroup,
	DCM,
	dedentToRoot,
	Doc,
	fill,
	getDocParts,
	group,
	hardline,
	ifBreak,
	indent,
	indentIfBreak,
	isConcat,
	join,
	label,
	line,
	lineSuffixBoundary,
	removeLines,
	softline,
	willBreak,
} from "./external";
import {
	f,
	getContext,
	getGrandParentNode,
	getNode,
	getOptions,
	getParentNode,
	getPrintFn,
	is_printing_macro,
	pathCall,
	pathCallEach,
	print,
} from "./plugin";
import { canInlineBlockBody, emptyContent, needsParens, shouldFlatten } from "./styling";
import { BlockLikeMacroInvocation, CallLikeMacroInvocation, is_CallExpression_or_CallLikeMacroInvocation } from "./transform";

export function isNoopExpressionStatement(node: Node) {
	return is_ExpressionStatement(node) && undefined === node.expression && !hasAttributes(node) && !hasComment(node);
}

export function getLastNotNoopExpressionStatement(parent: MaybeBlockBody) {
	return parent.body && find_last(parent.body, (stmt) => !isNoopExpressionStatement(stmt));
}

export function is_xVariableEqualishLike(node: Node) {
	switch (node.nodeType) {
		case NodeType.LetScrutinee:
		case NodeType.LetVariableDeclaration:
		case NodeType.ConstVariableDeclaration:
		case NodeType.StaticVariableDeclaration:
		case NodeType.TypeAliasDeclaration:
		case NodeType.TraitAliasDeclaration:
			return true;
		default:
			return false;
	}
}

export function is_BinaryishExpression(node: Node): node is OrExpression | AndExpression | OperationExpression | ComparisonExpression {
	switch (node.nodeType) {
		case NodeType.OrExpression:
		case NodeType.AndExpression:
		case NodeType.OperationExpression:
		case NodeType.ComparisonExpression:
			return true;
		default:
			return false;
	}
}

export type StructSpread = StructLiteralPropertySpread | StructLiteralRestUnassigned | RestPattern;
export function is_StructSpread(node: Node): node is StructSpread {
	switch (node.nodeType) {
		case NodeType.StructLiteralPropertySpread:
		case NodeType.StructLiteralRestUnassigned:
		case NodeType.RestPattern:
			return true;
		default:
			return false;
	}
}

type ArrayLikeNode = Exclude<Extract<Node, { items: Node[] }>, UnionPattern>;
function isConciselyPrintedArray(node: ArrayLikeNode) {
	return (
		node.items.length > 1 &&
		(node.items as Node[]).every(
			(element) =>
				(is_LiteralNumberLike(element) ||
					(is_MinusExpression(element) && is_LiteralNumberLike(element.expression) && !hasComment(element.expression))) &&
				!hasComment(element, CF.Trailing | CF.Line, (comment) => !hasBreaklineBefore(comment))
		)
	);
}
export function printCommentsInsideEmptyArray(path: AstPath<Node>) {
	const node = path.getValue();
	if (hasComment(node, CF.Dangling)) {
		return [printDanglingComments(node, false, DCM["items"]), softline];
	} else {
		return "";
	}
}

export function printNumber(rawNumber: string) {
	return rawNumber
		.toLowerCase()
		.replace(/^([\d.]+e)(?:\+|(-))?0*(\d)/, "$1$2$3")
		.replace(/^(\d+)e[+-]?0+$/, "$1.0")
		.replace(/^([\d.]+)e[+-]?0+$/, "$1")
		.replace(/\.(\d+?)0+(?=e|$)/, ".$1")
		.replace(/\.(?=e|$)/, ".0");
}

export function printOnOwnLine(node: Node, printed: Doc) {
	return [printed, maybeEmptyLine(node)];
}
export function maybeEmptyLine(node: Node) {
	return isNextLineEmpty(node) ? [hardline, hardline] : hardline;
}

export function printBodyOrCases<T extends NodeWithBodyOrCases | BlockLikeMacroInvocation>(print: print<T>, node: T) {
	// Note: Inner Attributes are inserted into body/cases (see "./transform.ts")
	// Example: { #[OUTER] #![INNER] 0 }; body: ["#[OUTER] 0", "#![INNER]"]
	const p: { node: Node; doc: Doc }[] = [];
	if (is_MatchExpression(node)) {
		__DEV__: Narrow<print<MatchExpression>>(print);
		pathCallEach(node as Extract<Node, MatchExpression>, "cases", (mCase) => {
			p.push({
				node: mCase,
				doc: is_MatchExpressionCase(mCase) && !is_ExpressionWithBodyOrCases(mCase.expression) ? [print(), ","] : print(),
			});
		});
	} else {
		__DEV__: Narrow<Extract<Node, MaybeBlockBody>>(node);
		pathCallEach(node as Extract<Node, MaybeBlockBody>, "body", (stmt) => {
			if (!isNoopExpressionStatement(stmt)) {
				p.push({ node: stmt, doc: print() });
			}
		});
	}

	const printed: Doc[] = bumpInnerAttributes(p).map(({ doc, node }, i, a) =>
		iLast(i, a) ? group(doc) : printOnOwnLine(node, group(doc))
	);

	const comments = printDanglingCommentsForInline(node, DCM["body"]);
	if (comments) printed.push(comments);
	const ccomments = printDanglingCommentsForInline(node, DCM["cases"]);
	if (ccomments) printed.push(ccomments);

	return printed;

	function bumpInnerAttributes(arr: { node: Node; doc: Doc }[]) {
		return arr.sort((a, b) => ownStart(a.node) - ownStart(b.node));
	}
}

export function printMacroRules<T extends MacroDeclaration | MacroRulesDeclaration>(print: print<T>, node: T) {
	return !Array.isArray(node.rules)
		? print("rules")
		: node.rules.length > 0
		? [" {", indent([hardline, ...print.join("rules", (rule) => maybeEmptyLine(rule))]), hardline, "}"]
		: [" {", printDanglingCommentsForInline(node, DCM["rules"]) || emptyContent(node), "}"];
}
function is_unary_token(item: MacroMatchSegment | undefined) {
	switch (item && is_PunctuationToken(item) ? item.tk : TK.None) {
		case TK["-"]:
		case TK["*"]:
		case TK["&"]:
		case TK["#"]:
		case TK["!"]:
		case TK["~"]:
			return true;
		case TK["?"]:
			return !/\s/.test(getOptions().originalText.charAt(end(item!)));
		default:
			return false;
	}
}
function can_unary(node: MacroMatchSegment) {
	return (!is_PunctuationToken(node) || is_unary_token(node)) && (!is_MacroGroup(node) || is_optional_unary(node));
}
function is_optional_token(item: MacroMatchSegment | undefined): item is MacroGroup<MacroMatchSegment> & { segments: [PunctuationToken] } {
	return !!item && is_MacroGroup(item) && item.kind === "?" && item.segments.length === 1 && is_PunctuationToken(item.segments[0]);
}
function is_optional_unary(item: MacroMatchSegment | undefined) {
	return is_optional_token(item) && is_unary_token(item.segments[0]);
}

function is_optional_segment(item: Node): item is MacroGroup {
	return is_MacroGroup(item) && item.kind === "?";
}

export function printRuleMatch<T extends MacroRuleDeclaration | MacroInlineRuleDeclaration>(print: print<T>, rule: T) {
	// "", ".", "&&", "||", "=", "+", "-", "*", "/", "%", "&", "|", "^", "<<", ">>", "==", "!=", ">", ">=", "<", "<=", "+=", "-=", "*=", "/=",
	// "%=", "&=", "|=", "^=", "<<=", ">>=", "$", "@", "_", "..", "...", "..=", ",", ";", ":", "::", "#", "?", "!", "=>", "->", "~"

	return print_map(rule, "match");

	type ArrProps<T extends {}> = { [K in keyof T]: NonNullable<T[K]> extends readonly any[] ? T[K] & unknown[] : never };
	function print_map<T extends Node, K extends keyof ArrProps<T> & keyof typeof DCM>(node: T, property: K) {
		__DEV__: assert(property in DCM);
		const arr = node[property as any] as LocArray<MacroMatchSegment>;
		const shouldHug = should_hug(arr);
		const dline =
			arr.dk === DelimKind["{}"] ? line : shouldHug ? "" /*  : is_MacroGroup(node) && node.kind !== "?" ? hardline */ : softline;
		const isParamsLike = is_params_like(arr);
		const shouldBreak = should_break(arr);
		const d = getDelimChars(arr);
		if (arr.length === 0) return [d.left, printDanglingCommentsForInline(node, DCM[property]), d.right];
		const printed = flat(print.map_join(property as any, print_item, join_item));

		// const printed = flat(print.map_join(property as any, print_item, join_item))
		// 	.reduce(
		// 		(arr, doc, i, a) => {
		// 			last_of(arr).push(doc);
		// 			if (doc === line && (a[i - 1] === "," || a[i - 1] === ";")) arr.push([]);
		// 			return arr;
		// 		},
		// 		[[]] as Doc[][]
		// 	)
		// 	.map((grp) => group(grp));

		return group([d.left, !dline ? printed : [indent([dline, printed]), dline], d.right], {
			shouldBreak,
			id: getMacroGroupId(node),
		});

		function should_hug(arr: LocArray<MacroMatchSegment>) {
			if (node === (rule as any)) return false;
			let has_nonToken = false;
			return arr.every((item) => !is_MacroGroup(item) && (is_PunctuationToken(item) || has_nonToken !== (has_nonToken = true)));
		}

		function should_break(arr: LocArray<MacroMatchSegment>) {
			let has_decl = false;
			return arr.some(
				(item, i, a) =>
					(is_match_any(item) && arr.length !== 1) ||
					(!iLast(i, a) && isDeclStart(item, a[i + 1]) && has_decl === (has_decl = true))
			);
		}

		function print_item(item: MacroMatchSegment, index: number, arr: MacroMatchSegment[]) {
			switch (item.nodeType) {
				case NodeType.Identifier:
				case NodeType.LtIdentifier:
				case NodeType.Literal:
				case NodeType.PunctuationToken:
				case NodeType.MacroParameterDeclaration:
					return print();
				case NodeType.MacroGroup:
					Narrow<print<MacroGroup>>(print);
					return printComments(["$", print_map(item, "segments"), print("sep"), item.kind]);
				case NodeType.DelimGroup:
					return printComments(print_map(item, "segments"));
			}
			__DEV__: exit.never();

			function printComments(doc: Doc) {
				const printed = withComments(item, doc);
				const comment = getFirstComment(item, CF.Leading | CF.Line);
				return comment && index !== 0
					? isPreviousLineEmpty(comment) &&
					  typeof join_item(arr[index - 1], item, index === 1 ? undefined : arr[index - 2]) === "string"
						? [hardline, hardline, printed]
						: [hardline, printed]
					: printed;
			}
		}

		function is_params_like(arr: MacroMatchSegment[]) {
			return arr.some(function isComma(item) {
				switch (item.nodeType) {
					case NodeType.PunctuationToken:
						return TK[","] === item.tk;
					case NodeType.MacroGroup:
						return (!!item.sep && isComma(item.sep)) || is_params_like(item.segments);
				}
			});
		}

		function join_item(item: MacroMatchSegment, next: MacroMatchSegment, prev: MacroMatchSegment | undefined) {
			if (is_PunctuationToken(item)) {
				switch (item.tk) {
					case TK[","]:
					case TK[";"]:
						return line;
					case TK["::"]:
					case TK[".."]:
					case TK["..."]:
					case TK["."]:
					case TK["#"]:
						return "";
					case TK["!"]:
						if (prev && is_ident(prev) && is_DelimGroup(next)) {
							return next.segments.dk === DelimKind["{}"] ? " " : "";
						}
						break;
					case TK["@"]:
						return is_ident(next) && (!prev || is_MacroGroup(prev) || is_DelimGroup(prev)) ? "" : " ";
				}

				return is_unary_token(item) && //
					(!prev || !is_ident(prev)) &&
					can_unary(next)
					? ""
					: " ";
			}

			switch (is_PunctuationToken(next) ? next.tk : TK.None) {
				case TK[","]:
				case TK[";"]:
				case TK[":"]:
				case TK["::"]:
				case TK[".."]:
				case TK["..."]:
				case TK["."]:
					return "";
				case TK["!"]:
					if (is_ident(item)) {
						return "";
					}
			}

			if (is_match_any(item)) {
				return line;
			}

			{
				const sep_tk = is_MacroGroup(item) && item.sep && is_PunctuationToken(item.sep) ? item.sep.tk : TK.None;
				switch (sep_tk) {
					case TK["::"]:
					case TK["."]:
						return ""; // $(...)::* | $(...).*
					case TK[","]:
					case TK[";"]:
						return sep_tk === maybe_tk(next) ? ifBreak(line, " ", { groupId: getMacroGroupId(item) }) : line;
				}
			}

			if (is_optional_token(item)) {
				switch (item.segments[0].tk) {
					case TK["+"]:
					case TK["|"]:
						return " ";
					case TK["::"]:
						return "";
				}
				if (is_unary_token(item.segments[0])) {
					return "";
				}
			}

			if (is_DelimGroup(item) || is_MacroGroup(item)) {
				if (item.segments.dk === DelimKind["{}"]) {
					return line;
				}

				if (is_MacroGroup(item) && item.segments.length === 2) {
					const { 0: left, 1: right } = item.segments;
					if (is_PunctuationToken(left) && is_DelimGroup(right) && left.tk === TK["#"] && right.segments.dk === DelimKind["[]"]) {
						return hardline;
					}
				}
				return isParamsLike || is_tk(next) ? " " : line;
			}

			const next_1 = next !== last_of(arr) && arr[arr.indexOf(next) + 1];
			if (is_ident(item) && is_DelimGroup(next) && next.segments.dk === DelimKind["()"]) {
				if (!next_1 || !is_match_any(next_1)) {
					return "";
				}
			}

			if (is_match_any(next) && (!is_DelimGroup(next) || (next_1 && is_match_any(next_1)))) {
				return line;
			}

			// if (is_ident(item) && !is_ident(next) && !(is_DelimGroup(next) && next.segments.dk === DelimKind["{}"])) {
			// 	const next_1 = arr[arr.indexOf(next) + 1];
			// 	if (next_1 && typeof join_item(next, next_1, item) === "object") {
			// 		return line;
			// 	}
			// }

			return " ";
		}
	}

	function is_ident(item: MacroMatchSegment) {
		switch (item.nodeType) {
			case NodeType.Identifier:
				return true;
			case NodeType.MacroParameterDeclaration:
				return item.ty.name === "ident";
			default:
				return false;
		}
	}
	function is_tk(item: MacroMatchSegment) {
		return is_PunctuationToken(item) || is_optional_token(item);
	}
	function maybe_tk(item: MacroMatchSegment) {
		switch (item.nodeType) {
			case NodeType.PunctuationToken:
				return item.tk;
			case NodeType.MacroGroup:
				return is_optional_token(item) ? item.segments[0].tk : TK.None;
			default:
				return TK.None;
		}
	}
	function isDeclStart(item: MacroMatchSegment, next: MacroMatchSegment) {
		if (is_Identifier(item)) {
			switch (item.name) {
				case "fn":
				case "mod":
				case "use":
				case "struct":
				case "trait":
				case "union":
				case "enum":
				case "impl":
				case "type":
				case "let":
				case "static":
				case "const":
					if (is_ident(next)) {
						return true;
					}
			}
		}
		return false;
	}
	function is_match_any(item: MacroMatchSegment) {
		return (
			!!item &&
			((is_MacroGroup(item) &&
				!item.sep &&
				(item.kind === "*" || item.kind === "+") &&
				item.segments.length === 1 &&
				is_MacroParameterDeclaration(item.segments[0]) &&
				item.segments[0].ty.name === "tt") ||
				(is_DelimGroup(item) && item.segments.length === 1 && is_match_any(item.segments[0])))
		);
	}
}

export function printRuleTransform<T extends MacroRuleDeclaration | MacroInlineRuleDeclaration>(
	print: print<T>,
	node: T,
	t: DelimChars = getDelimChars(node.transform)
) {
	const text = node.transform.loc.sliceText();
	const fline = is_MacroInlineRuleDeclaration(node) ? hardline : line;
	if (/^. *\n/.test(text)) {
		return [
			dedentToRoot([
				t.left,
				fline,
				text
					.slice(1, -1) //
					.replace(/^ *\n|\n\s*$/g, ""),
			]),
			fline,
			t.right,
		];
	} else if (/\n/.test(text) && node.transform.length === 1) {
		const segment = node.transform[0];
		if (is_DelimGroup(segment) || is_MacroGroup(segment)) {
			const inner = is_DelimGroup(segment)
				? getDelimChars(segment.segments)
				: { left: "$(", right: `)${segment.sep?.loc.getOwnText() ?? ""}${segment.kind}` };
			return [
				dedentToRoot([
					t.left,
					[
						indent(indent([fline, inner.left])),
						line,
						segment.segments.loc.sliceText(1, -1).replace(/^ *\n|\n\s*$/g, ""),
						indent(indent([line, inner.right])),
					],
				]),
				fline,
				t.right,
			];
		}
	}
	return text;

	return node.transform.length > 0
		? group([t.left, indent([line, print("transform")]), line, t.right])
		: [t.left, printDanglingCommentsForInline(node, DCM["transform"]), t.right];
}

function is_AssignmentOrVariableDeclarator(node: Node): boolean {
	return is_ReassignmentNode(node) || is_VariableDeclarationNode(node);
}
function hasLeadingOwnLineComment(node: Node): boolean {
	if (is_NodeWithBodyOrCases(node) && hasComment(node, CF.Leading, is_Attribute)) {
		return true;
	}
	return hasComment(
		node,
		CF.Leading,
		(comment) => hasNewline(end(comment)) && !getContext().options.danglingAttributes.includes(comment as any)
	);
}
function isComplexDestructuring(node: Node): boolean {
	if (is_ReassignmentExpression(node)) {
		const leftNode = node.left;
		return (
			is_StructLiteral(leftNode) && //
			leftNode.properties.length > 2 &&
			leftNode.properties.some((property) => is_StructLiteralProperty(property) || is_StructLiteralPropertySpread(property))
		);
	}
	if (is_VariableDeclarationNode(node) || is_MatchExpressionCase(node) || is_LetScrutinee(node)) {
		const leftNode = node.pattern;
		return (
			is_StructPattern(leftNode) && //
			leftNode.properties.length > 2 &&
			leftNode.properties.some((property) => is_StructPatternPropertyDestructured(property))
		);
	}
	return false;
}

// export function isShortWhereBoundDeclaration(node: WhereBoundDeclaration) {
// 	switch (node.nodeType) {
// 		case NodeType.WhereTypeBoundDeclaration:
// 			__DEV__: Narrow<WhereTypeBoundDeclaration>(node);
// 			return !node.ltParameters && is_Identifier(node.typeTarget);
// 		case NodeType.WhereLtBoundDeclaration:
// 			__DEV__: Narrow<WhereLtBoundDeclaration>(node);
// 			// return !node.typeAnnotation && !node.typeDefault;
// 		default:
// 			exit.never();
// 	}
// }

function isArrowFunctionVariableDeclarator(node: Node) {
	return is_VariableDeclarationNode(node) && node.expression && is_ClosureFunctionExpression(node.expression);
}
function isObjectPropertyWithShortKey(node: Node, keyDoc: Doc) {
	if (!is_StructProperty(node)) return false;
	keyDoc = cleanDoc(keyDoc);
	const MIN_OVERLAP_FOR_BREAK = 3;
	return typeof keyDoc === "string" && keyDoc.length < getContext().options.tabWidth + MIN_OVERLAP_FOR_BREAK;
}

function print_CallExpression_end(print: print<CallExpression>, node: CallExpression) {
	return [f`::${printTypeArguments(print, node)}`, printCallArguments(print, node)];
}

export function printCallExpression(print: print<CallExpression>, node: CallExpression) {
	if (shouldPrint_CallExpression_chain(node) && !pathCall(node, "callee", (node) => needsParens(node))) {
		return printMemberChain(print, node);
	}

	const contents = [print("callee"), ...print_CallExpression_end(print, node)];

	if (is_CallExpression_or_CallLikeMacroInvocation(node.callee)) {
		return group(contents);
	}

	return contents;
}

export function printTypeAnnotation<T extends Extract<Node, MaybeTypeAnnotationTarget<any>>>(print: print<T>, node: T) {
	return node.typeAnnotation && !is_MissingNode(node.typeAnnotation) ? [": ", print("typeAnnotation")] : "";
}
export function printAnnotatedPattern<T extends Extract<Node, MaybeTypeAnnotationTarget<any> & PatternBody>>(print: print<T>, node: T) {
	return [print("pattern"), printTypeAnnotation(print, node)];
}

function isLoneShortArgument(node: Node) {
	if (hasComment(node)) {
		return false;
	}

	if ((is_Identifier(node) && is_short(node.name)) || (is_LiteralNumberLike(node) && !hasComment(node))) {
		return true;
	}

	if (is_LiteralStringLike(node)) {
		return is_short(node.value) && !node.value.includes("\n");
	}

	return is_LiteralBooleanLike(node);
}

// prettier-ignore
const toLayout = ["break-after-operator", "never-break-after-operator", "fluid", "break-lhs", "chain", "chain-tail", "chain-tail-arrow-chain", "only-left"];
const enum Layout {
	"break-after-operator",
	"never-break-after-operator",
	"fluid",
	"break-lhs",
	"chain",
	"chain-tail",
	"chain-tail-arrow-chain",
	"only-left",
}

export function printMemberExpression(print: print<MemberExpression>, node: MemberExpression) {
	const objectDoc = print("expression");
	const lookupDoc = printMemberLookup(print, node);

	const shouldInline = shouldInlineMemberExpression(node, objectDoc);

	return label((objectDoc as any).label === "member-chain" ? "member-chain" : "member", [
		objectDoc,
		shouldInline ? lookupDoc : group(indent([softline, lookupDoc])),
	]);
}

function shouldInlineMemberExpression(node: MemberExpression, objectDoc: Doc) {
	const { path } = getContext();
	const parent = getParentNode()!;
	let i = 0;
	let nmparent: Node | null = parent;
	while (nmparent && (is_MemberExpression(nmparent) || is_PostfixExpression(nmparent))) {
		nmparent = path.getParentNode(i++);
	}

	const shouldInline =
		(nmparent && (is_ExpressionPath(nmparent) || (is_ReassignmentNode(nmparent) && !is_Identifier(nmparent.left)))) ||
		!node.computed ||
		(is_Identifier(node.expression) && is_Identifier(node.property) && !is_MemberExpression(parent)) ||
		(is_AssignmentOrVariableDeclarator(parent) &&
			((is_CallExpression_or_CallLikeMacroInvocation(node.expression) && node.expression.arguments.length > 0) ||
				(is_PostfixExpression(node.expression) &&
					is_CallExpression_or_CallLikeMacroInvocation(node.expression.expression) &&
					node.expression.expression.arguments.length > 0) ||
				(objectDoc as any).label === "member-chain"));
	return shouldInline;
}

export function printAssignment(leftDoc: Doc, operator: string, rightPropertyName: string) {
	const assignmentNode = getNode();
	const rightNode = assignmentNode[rightPropertyName];

	if (!rightNode) return group(leftDoc);

	const layout = chooseLayout();
	const rightDoc = getPrintFn(assignmentNode)(rightPropertyName as any, { assignmentLayout: layout });
	const res = (function () {
		switch (layout) {
			case Layout["break-after-operator"]:
				return group([group(leftDoc), operator, group(indent([line, rightDoc]))]);
			case Layout["never-break-after-operator"]:
				return group([group(leftDoc), operator, " ", rightDoc]);
			case Layout["fluid"]: {
				const groupId = Symbol("assignment");
				return group([
					group(leftDoc),
					operator, //
					group(indent(line), { id: groupId }),
					lineSuffixBoundary,
					indentIfBreak(rightDoc, { groupId }),
				]);
			}
			case Layout["break-lhs"]:
				return group([leftDoc, operator, " ", group(rightDoc)]);
			case Layout["chain"]:
				return [group(leftDoc), operator, line, rightDoc];
			case Layout["chain-tail"]:
				return [group(leftDoc), operator, indent([line, rightDoc])];
			case Layout["chain-tail-arrow-chain"]:
				return [group(leftDoc), operator, rightDoc];
			default:
				exit.never();
		}
	})();

	return label(toLayout[layout], res);
	return res;

	function chooseLayout() {
		if (
			(is_ReassignmentExpression(assignmentNode) && is_printing_macro()) ||
			is_GenericParameterDeclaration(assignmentNode) ||
			is_TypeCallNamedArgument(assignmentNode)
		) {
			return Layout["never-break-after-operator"];
		}

		const isTail = !is_ReassignmentNode(rightNode);
		const shouldUseChainFormatting = getContext().path.match(
			is_ReassignmentNode,
			is_AssignmentOrVariableDeclarator,
			(node: Node) => !isTail || (!is_ExpressionStatement(node) && !is_VariableDeclarationNode(node))
		);

		if (shouldUseChainFormatting) {
			return !isTail
				? Layout["chain"]
				: is_ClosureFunctionExpression(rightNode) && is_ClosureFunctionExpression(rightNode.expression)
				? Layout["chain-tail-arrow-chain"]
				: Layout["chain-tail"];
		}

		const isHeadOfLongChain = !isTail && is_ReassignmentNode(rightNode.right);
		if (isHeadOfLongChain || hasLeadingOwnLineComment(rightNode)) {
			return Layout["break-after-operator"];
		}

		if (
			isComplexDestructuring(assignmentNode) ||
			hasComplexGenerics(assignmentNode) ||
			hasComplexTypeAnnotation(assignmentNode) ||
			(isArrowFunctionVariableDeclarator(assignmentNode) && canBreak(leftDoc))
		) {
			return Layout["break-lhs"];
		}

		const hasShortKey = isObjectPropertyWithShortKey(assignmentNode, leftDoc);

		if (pathCall(assignmentNode, rightPropertyName as never, (rightNode) => shouldBreakAfterOperator(rightNode, hasShortKey))) {
			return Layout["break-after-operator"];
		}

		if (hasShortKey || is_Literal(rightNode)) {
			return Layout["never-break-after-operator"];
		}

		return Layout["fluid"];
	}

	function shouldBreakAfterOperator(rightNode: Node, hasShortKey: boolean) {
		if (is_MemberExpression(rightNode) && shouldInlineMemberExpression(rightNode, getPrintFn(rightNode)("expression"))) {
			return false;
		}

		if (is_BinaryishExpression(rightNode) && !shouldInlineLogicalExpression(rightNode)) {
			return true;
		}

		if (is_IfBlockExpression(rightNode)) {
			return false;
		}

		if (hasShortKey) {
			return false;
		}

		return (function unwrap(node: Node) {
			if (is_UnaryExpression(node) || is_PostfixExpression(node)) {
				return pathCall(node, "expression", unwrap);
			}
			if (is_LiteralStringLike(node)) {
				return true;
			}
			return isPoorlyBreakableMemberOrCallChain(node);
		})(rightNode);

		function isPoorlyBreakableMemberOrCallChain(topNode: Node) {
			return (function unwrap(node: Node): boolean {
				if (is_MemberExpression(node) || is_PostfixExpression(node) || is_UnaryExpression(node)) {
					return pathCall(node, "expression", unwrap);
				}

				if (is_ExpressionPath(node)) {
					return pathCall(node, "namespace", (namespace) => !namespace || unwrap(namespace));
				}

				if (is_CallExpression_or_CallLikeMacroInvocation(node)) {
					const doc = printCallExpression(getPrintFn(), node);
					if ((doc as any).label === "member-chain") {
						return false;
					}

					const args = node.arguments;
					const isPoorlyBreakableCall = args.length === 0 || (args.length === 1 && isLoneShortArgument(args[0]));
					if (!isPoorlyBreakableCall) {
						return false;
					}

					if (hasComplexTypeArguments(node)) {
						return false;
					}

					return pathCall(node, "callee", unwrap);
				}

				return topNode === node ? false : is_Identifier(node);
			})(topNode);
		}
	}
}
function is_MemberExpression_with_RangeOrLiteral_Property(node: Node | undefined) {
	return (
		!!node && is_MemberExpression(node) && (node.computed ? is_Literal_or_SimpleRangeLiteral(node.property) : is_Literal(node.property))
	);
}
function is_Literal_or_SimpleRangeLiteral(node: Node) {
	return is_Literal(node)
		? true
		: is_RangeLiteral(node)
		? (!node.lower || is_Literal(node.lower)) && (!node.upper || is_Literal(node.upper))
		: false;
}
function printMemberLookup(print: print<MemberExpression>, node: MemberExpression) {
	return !node.computed
		? [".", print("property")]
		: is_Literal_or_SimpleRangeLiteral(node.property)
		? ["[", print("property"), "]"]
		: group(["[", indent([softline, print("property")]), softline, "]"]);
}

function shouldPrint_CallExpression_chain(node: CallExpression) {
	return is_MemberAccessLike(node.callee) || is_CallExpression_or_CallLikeMacroInvocation(node.callee);
}
export function is_MemberAccessLike(node: Node): node is ExpressionPath | MemberExpression {
	switch (node.nodeType) {
		case NodeType.ExpressionPath:
		case NodeType.MemberExpression:
			return true;
		default:
			return false;
	}
}
type ChainItem = { node: Node; printed: Doc; needsParens: boolean };
function printMemberChain(print: print<CallExpression>, node: CallExpression) {
	const parent = getParentNode();
	const isExpressionStatement = !parent || is_ExpressionStatement(parent);
	const { printedNodes, groups } = splitCallChains(node);

	const shouldMerge = groups.length >= 2 && !hasComment(groups[1][0].node) && shouldNotWrap(groups);

	const printedGroups = groups.map(printGroup);
	const oneLine = printedGroups;

	const cutoff = shouldMerge ? 3 : 2;

	const nodeHasComment =
		printedNodes.slice(1, -1).some(({ node }) => hasComment(node, CF.Leading)) ||
		printedNodes.slice(0, -1).some(({ node }) => hasComment(node, CF.Trailing)) ||
		(groups[cutoff] && hasComment(groups[cutoff][0].node, CF.Leading));

	if (groups.length <= cutoff && !nodeHasComment) {
		return isLongCurriedCallExpression(node) ? oneLine : group(oneLine);
	}

	const lastNodeBeforeIndent = last_of(groups[shouldMerge ? 1 : 0]).node;
	const shouldHaveEmptyLineBeforeIndent =
		!is_CallExpression_or_CallLikeMacroInvocation(lastNodeBeforeIndent) && shouldInsertEmptyLineAfter(lastNodeBeforeIndent);

	const expanded = [
		printGroup(groups[0]),
		shouldMerge ? groups.slice(1, 2).map(printGroup) : "",
		shouldHaveEmptyLineBeforeIndent ? hardline : "",
		printIndentedGroup(groups.slice(shouldMerge ? 2 : 1)),
	];

	const callExpressions = printedNodes.map(({ node }) => node).filter(is_CallExpression_or_CallLikeMacroInvocation);
	const result: Doc =
		nodeHasComment ||
		(callExpressions.length > 2 && callExpressions.some((expr) => expr.arguments.some((arg) => !isSimpleCallArgument(arg, 0)))) ||
		printedGroups.slice(0, -1).some(willBreak) ||
		lastGroupWillBreakAndOtherCallsHaveFunctionArguments()
			? group(expanded)
			: [shouldHaveEmptyLineBeforeIndent || willBreak(oneLine) ? breakParent : "", conditionalGroup([oneLine, expanded])];

	return label("member-chain", result);

	function shouldInsertEmptyLineAfter(node: Node) {
		let start = end(node);
		const last = getNextNonSpaceNonCommentCharacterIndex(node);
		const { originalText } = getContext().options;
		while (start < last) {
			if (originalText.charAt(start) === ")") {
				return isNextLineEmptyAfterIndex(start + 1);
			}
			start++;
		}
		return isNextLineEmpty(node);
	}

	function isFactory(name: string) {
		return /^[A-Z]|^[$_]+$/.test(name);
	}
	function isShort(name: string) {
		return name.length <= getContext().options.tabWidth;
	}

	function shouldNotWrap(groups: ChainItem[][]) {
		const hasComputed = groups[1].length > 0 && is_MemberExpression(groups[1][0].node) && groups[1][0].node.computed;

		if (groups[0].length === 1) {
			const firstNode = groups[0][0].node;
			return (
				is_Identifier(firstNode) && (isFactory(firstNode.name) || (isExpressionStatement && isShort(firstNode.name)) || hasComputed)
			);
		}

		const lastNode = last_of(groups[0]).node;
		const lastNodeLeft = is_ExpressionPath(lastNode)
			? lastNode.namespace
			: is_MemberExpression(lastNode)
			? lastNode.expression
			: undefined;
		return lastNodeLeft && is_Identifier(lastNodeLeft) && (isFactory(lastNodeLeft.name) || hasComputed);
	}

	function printGroup(g: ChainItem[]) {
		const printed: Doc[] = [];
		if (printedNodes[0] === g[0]) {
			for (const item of printedNodes) {
				if (item.needsParens) printed.unshift("(");
			}
		}
		for (const item of g) {
			printed.push(item.printed);
			if (item.needsParens) printed.push(")");
		}
		return printed;
	}

	function printIndentedGroup(groups: ChainItem[][]) {
		if (groups.length === 0) return "";
		return indent(group([hardline, join(hardline, groups.map(printGroup))]));
	}

	function lastGroupWillBreakAndOtherCallsHaveFunctionArguments() {
		const lastGroupNode = last_of(last_of(groups)).node;
		const lastGroupDoc = last_of(printedGroups);
		return (
			is_CallExpression_or_CallLikeMacroInvocation(lastGroupNode) &&
			willBreak(lastGroupDoc) &&
			callExpressions.slice(0, -1).some((node) => node.arguments.some(is_ClosureFunctionExpression))
		);
	}

	function splitCallChains(topNode: CallExpression | CallLikeMacroInvocation) {
		const printedNodes: ChainItem[] = [
			{
				node: topNode,
				needsParens: false,
				printed: print_CallExpression_end(print, node),
			},
		];

		pathCall(topNode, "callee", function READ_LEFT(node: Node) {
			if (is_CallExpression_or_CallLikeMacroInvocation(node) && shouldPrint_CallExpression_chain(node)) {
				__DEV__: Narrow<print<typeof node>>(print);

				unshift(print_CallExpression_end(print, node), shouldInsertEmptyLineAfter(node));
				pathCall(node, "callee", READ_LEFT);
			} else if (is_MemberExpression(node)) {
				__DEV__: Narrow<print<typeof node>>(print);

				unshift(printMemberLookup(print, node));
				pathCall(node, "expression", READ_LEFT);
			} else if (is_ExpressionPath(node)) {
				__DEV__: Narrow<print<typeof node>>(print);

				unshift(["::", print("segment")]);
				if (node.namespace) {
					pathCall(node, "namespace", READ_LEFT as any);
				}
			} else if (is_PostfixExpression(node)) {
				unshift(is_UnwrapExpression(node) ? "?" : ".await");
				pathCall(node, "expression", READ_LEFT);
			} else {
				printedNodes.unshift({ node, needsParens: false, printed: print() });
			}

			function unshift(printed: Doc, needsHardlineAfter = false) {
				printedNodes.unshift({
					node,
					needsParens: is_MemberAccessLike(node) && needsParens(node),
					printed: [withComments(node, printed), needsHardlineAfter ? hardline : ""],
				});
			}
		});

		const groups = spread(function* () {
			let i = 0;
			let currentItem = printedNodes[i];

			function testNextItem(fn: (item: ChainItem) => boolean) {
				return i + 1 < printedNodes.length && fn(printedNodes[i + 1]);
			}

			function readGroup(fn: () => Iterable<ChainItem>) {
				__DEV__: assert(i < printedNodes.length);
				return spread(function* () {
					for (var _item of fn()) {
						yield currentItem;
						if (++i < printedNodes.length) currentItem = printedNodes[i];
						else break;
					}
				});
			}

			function* loop(condition: (item: ChainItem) => boolean) {
				while (condition(currentItem)) yield currentItem;
			}
			function* until(condition: (item: ChainItem) => boolean) {
				while (!condition(currentItem)) yield currentItem;
			}

			yield readGroup(function* () {
				const isCallExpression = is_CallExpression_or_CallLikeMacroInvocation(currentItem.node);

				yield currentItem;
				yield* loop(
					({ node, needsParens }) =>
						is_PostfixExpression(node) ||
						is_CallExpression_or_CallLikeMacroInvocation(node) ||
						is_MemberExpression_with_RangeOrLiteral_Property(node) ||
						needsParens
				);

				if (!isCallExpression) {
					yield* loop(
						({ node, needsParens }) =>
							is_MemberAccessLike(node) && //
							testNextItem(({ node }) => is_MemberAccessLike(node))
					);
				}
			});

			while (i < printedNodes.length) {
				yield readGroup(function* () {
					let isCallExpression = false;

					yield* until(
						({ node }) =>
							(isCallExpression = is_CallExpression_or_CallLikeMacroInvocation(node)) || //
							hasComment(node, CF.Trailing)
					);
					yield currentItem;

					if (isCallExpression) {
						yield* loop(({ node }) => is_MemberExpression_with_RangeOrLiteral_Property(node));
						yield* until(
							({ node }) =>
								is_MemberAccessLike(node) || //
								hasComment(node, CF.Trailing)
						);
					}
				});
			}
		});
		return { printedNodes, groups };
	}
}

function isSimpleCallArgument(node: Node, depth: number) {
	if (depth >= 2) return false;

	if (is_IdentifierOrIndex(node)) {
		return true;
	}

	if (is_Literal(node)) {
		return !is_LiteralStringLike(node) || !node.value.includes("\n");
	}

	if (is_ArrayOrTupleLiteral(node)) {
		return node.items.every(isChildSimple);
	}

	if (is_StructLiteral(node)) {
		return (
			isSimpleCallArgument(node.struct, depth) &&
			node.properties.every((prop) =>
				is_StructLiteralPropertySpread(prop)
					? isChildSimple(prop.expression)
					: is_StructLiteralProperty(prop)
					? isChildSimple(prop.value)
					: true
			)
		);
	}

	if (is_CallExpression_or_CallLikeMacroInvocation(node)) {
		return (
			isSimpleCallArgument(node.callee, depth) &&
			(node.typeArguments ?? []).every(isChildSimple) &&
			node.arguments.every(isChildSimple)
		);
	}

	if (is_MemberExpression(node)) {
		return isSimpleCallArgument(node.expression, depth) && isSimpleCallArgument(node.property, depth);
	}

	if (is_ExpressionTypeCast(node)) {
		return isSimpleCallArgument(node.typeCallee, depth) && node.typeArguments.every(isChildSimple);
	}

	if (is_ExpressionPath(node)) {
		const namespace = node.namespace;
		return !namespace || isSimpleCallArgument(namespace, depth);
	}

	if (is_UnaryExpression(node) || is_PostfixExpression(node)) {
		return isSimpleCallArgument(node.expression, depth);
	}

	return false;

	function isChildSimple(child: Node) {
		return isSimpleCallArgument(child, depth + 1);
	}
}
function isLongCurriedCallExpression(node: Node) {
	const parent = getParentNode(node)!;
	return (
		is_CallExpression_or_CallLikeMacroInvocation(node) &&
		is_CallExpression_or_CallLikeMacroInvocation(parent) &&
		parent.callee === node &&
		node.arguments.length > parent.arguments.length &&
		parent.arguments.length > 0
	);
}

export function printTypeArguments<T extends Extract<Node, MaybeGenericArgsTarget>>(print: print<T>, node: T) {
	return !node.typeArguments
		? ""
		: node.typeArguments.length === 0
		? ["<", printDanglingCommentsForInline(node, DCM["typeArguments"]), ">"]
		: hasComplexTypeArguments(node)
		? group(
				[
					"<", //
					indent([softline, print.join("typeArguments", [",", line])]),
					softline,
					">",
				],
				{ id: getTypeParametersGroupId(node) }
		  )
		: ["<", print.join("typeArguments", ", "), ">"];
}

export function printLtParameters<T extends Extract<Node, ForLtParametersBody>>(print: print<T>, node: T) {
	return !node.ltParameters
		? ""
		: node.ltParameters.length === 0
		? ["for<", printDanglingCommentsForInline(node, DCM["ltParameters"]), "> "]
		: hasComplexLtParameters(node)
		? group(
				[
					"for<", //
					indent([softline, print.join("ltParameters", [",", line])]),
					softline,
					"> ",
				],
				{ id: getTypeParametersGroupId(node) }
		  )
		: ["for<", print.join("ltParameters", ", "), "> "];
}

export function printGenerics<T extends DeclarationNode>(print: print<T>, node: T) {
	return group(
		!node.generics
			? ""
			: hasComplexGenerics(node)
			? [
					"<",
					indent([softline, print.join("generics", [",", line])]), //
					hasMultipleHeritage(node) ? indent([softline, ">"]) : [softline, ">"],
			  ]
			: [
					"<",
					print.join("generics", ", "), //
					printDanglingCommentsForInline(node, DCM["generics"]),
					">",
			  ]
	);
}
function getPrintedTypeBounds<T extends Extract<Node, TypeBoundsConstaint>>(print: print<T>, node: T) {
	if (!hasTypeBounds(node) || node.typeBounds.length === 0) return "";
	if (node.typeBounds.length === 1) return print.map("typeBounds");
	// let shouldIndent = false;
	// const printed = print.map("typeBounds", (bound, i, arr) =>
	// 	0 === i
	// 		? print()
	// 		: true //isSimpleTypeBound(arr[i - 1]) && isSimpleTypeBound(bound)
	// 		? indent([" +", line, print()])
	// 		: [" + ", (shouldIndent ||= isSimpleTypeBound(arr[i - 1]) || isSimpleTypeBound(bound)) ? indent(print()) : print()]
	// );
	const printed = print.join("typeBounds", (_, __, prev) => (!prev ? " +" : [" +", line]));
	return [printed.shift()!, indent([line, printed])];
}
export function printTypeBounds<T extends Extract<Node, TypeBoundsConstaint>>(operator: "dyn" | "impl" | ":", print: print<T>, node: T) {
	if (!hasTypeBounds(node)) return "";
	const printed = getPrintedTypeBounds(print, node);
	return printed ? group([operator, " ", printed]) : operator;

	return !node.typeBounds
		? ""
		: hasComplexTypeBounds(node)
		? group(indent([ifBreak(line), operator, " ", join([line, "+ "], print.map("typeBounds"))]))
		: [operator, " ", join(" + ", print.map("typeBounds"))];
}

export function printLtBounds<T extends Extract<Node, MaybeHasLtBounds>>(left: Doc, print: print<T>, node: T) {
	return group(
		!node.ltBounds
			? ""
			: node.ltBounds.length === 0
			? [left, " "]
			: [left, " ", print.map("ltBounds", (typeBound, i) => (i === 0 ? print() : indent([line, "+ ", print()])))]
	);
}

function printWhereBounds<T extends DeclarationNode>(print: print<T>, node: T) {
	if (!node.whereBounds || node.whereBounds.length === 0) return "";
	return adjustDeclarationClause(
		node, //
		"where",
		print.join("whereBounds", [",", line])
	);
}
export function printDeclarationTypeBounds<T extends Extract<DeclarationNode, TypeBoundsConstaint>>(
	print: print<T>,
	node: T,
	operator: ":" | " ="
) {
	return hasTypeBounds(node) ? adjustDeclarationClause(node, operator, getPrintedTypeBounds(print, node)) : "";
}
export function printImplTraitForType(
	print: print<ImplDeclaration | NegativeImplDeclaration>,
	node: ImplDeclaration | NegativeImplDeclaration
) {
	return node.trait
		? [print("trait"), adjustDeclarationClause(node, "for", print("typeTarget"))] //
		: print("typeTarget");
}

function adjustDeclarationClause(node: DeclarationNode, clause: ":" | " =" | "for" | "->" | "where", content: Doc) {
	const isTypeBoundsClause = clause === ":" || clause === " =";
	return (clause === "where" || hasMultipleHeritage(node) ? indent : Identity)([
		clause === "->"
			? hasMultipleHeritage(node) && node.whereBounds!.length > 1
				? line
				: " "
			: clause === "where" //
			? line
			: isTypeBoundsClause
			? hasMultipleHeritage(node)
				? softline
				: ""
			: line,
		clause,
		content &&
			group(
				clause === "where"
					? indent([line, content]) //
					: clause === "->" || isTypeBoundsClause
					? [" ", content]
					: [line, content]
			),
	]);
}
function hasNonWhereHeritageClause(node: DeclarationNode) {
	AssertTypesEq<
		DeclarationNode,
		| FunctionDeclaration
		| StructDeclaration
		| TupleStructDeclaration
		| UnionDeclaration
		| TypeAliasDeclaration
		| TraitDeclaration
		| TraitAliasDeclaration
		| NegativeImplDeclaration
		| ImplDeclaration
		| EnumDeclaration
	>();
	switch (node.nodeType) {
		case NodeType.FunctionDeclaration:
			return !!node.returnType;
		case NodeType.StructDeclaration:
		case NodeType.TupleStructDeclaration:
		case NodeType.UnionDeclaration:
		case NodeType.EnumDeclaration:
			return false;
		case NodeType.TypeAliasDeclaration:
		case NodeType.TraitDeclaration:
		case NodeType.TraitAliasDeclaration:
			// case NodeType.AutoTraitDeclaration:
			return hasTypeBounds(node);
		case NodeType.ImplDeclaration:
		case NodeType.NegativeImplDeclaration:
			return !!node.trait;
		default:
			__DEV__: exit.never(node);
	}
}
function hasAnyHeritageClause(node: DeclarationNode) {
	return !!node.whereBounds || hasNonWhereHeritageClause(node);
}

function hasMultipleHeritage(node: DeclarationNode) {
	return !!node.whereBounds && hasNonWhereHeritageClause(node);
}

const getMacroGroupId = createGroupIdMapper("MacroGroup");
const getHeritageGroupId = createGroupIdMapper("heritageGroup");
const getWhereBoundsGroupId = createGroupIdMapper("where");
const getTypeParametersGroupId = createGroupIdMapper("typeParameters");
function createGroupIdMapper(description: string) {
	const groupIds = new WeakMap<Node, symbol>();
	return (node: Node) => Map_get(groupIds, node, () => Symbol(description));
}

export function printDanglingCommentsForInline(node: Node, marker?: DCM) {
	const hasOnlyBlockComments =
		!hasComment(node, CF.Line | CF.Dangling, (comment) => !marker || comment.marker === marker) || is_Program(node);
	const printed = printDanglingComments(node, hasOnlyBlockComments, marker);
	return (
		printed &&
		(hasOnlyBlockComments && !is_Program(node)
			? willBreak(printed)
				? [indent([hardline, printed]), hardline]
				: [printed]
			: [printed, hardline])
	);
}

function isFormatLikeCall(node: CallExpression | CallLikeMacroInvocation) {
	if (is_Identifier(node.callee) && !node.typeArguments) {
		const [first, ...rest] = node.arguments;
		if (is_Literal(first) && is_LiteralStringLike(first) && first.value.includes("{}") && rest.every(is_Identifier)) {
			return true;
		}
	}
	return false;
}

class ArgExpansionBailout extends Error {}

export function printCallArguments<T extends CallExpression | CallLikeMacroInvocation>(print: print<T>, node: T) {
	const args = node.arguments;
	const { left: LEFT, right: RIGHT } = getDelimChars(args);

	__DEV__: {
		// assert(args.length === 0 || !hasComment(node, CF.Dangling), "", node);
		if (is_MacroInvocation(node)) assert(args.every(is_ExpressionNode), "", node);
	}

	if (args.length === 0) return [LEFT, printDanglingCommentsForInline(node, DCM["arguments"]), RIGHT];

	// force inline format!(" {} ", ident)
	if (args.length === 2 && isFormatLikeCall(node)) {
		return [LEFT, print(["arguments", 0]), ", ", print(["arguments", 1]), RIGHT];
	}

	let anyArgEmptyLine = false;
	let hasEmptyLineFollowingFirstArg = false;
	const lastArgIndex = args.length - 1;
	const trailingComma = false ? "," : "";
	const printedArguments = print.map("arguments", (arg, index, arr) => {
		if (index === lastArgIndex) {
			return [print()];
		} else if (isNextLineEmpty(arg)) {
			if (index === 0) hasEmptyLineFollowingFirstArg = true;
			anyArgEmptyLine = true;
			return [print(), ",", hardline, hardline];
		} else {
			return [print(), ",", line];
		}
	});

	if (anyArgEmptyLine || isFunctionCompositionArgs(args)) {
		return allArgsBrokenOut();
	}

	const shouldGroupFirst = shouldGroupFirstArg(args);
	const shouldGroupLast = shouldGroupLastArg(args);
	if (shouldGroupFirst || shouldGroupLast) {
		if (shouldGroupFirst ? printedArguments.slice(1).some(willBreak) : printedArguments.slice(0, -1).some(willBreak)) {
			return allArgsBrokenOut();
		}
		let printedExpanded: Doc[] = [];
		const { path } = getContext();
		const stackBackup = [...path.stack];
		try {
			path_try(() => {
				getContext().path.each((p, i) => {
					if (shouldGroupFirst && i === 0) {
						printedExpanded = [
							[
								print([], { expandFirstArg: true }),
								printedArguments.length > 1 ? "," : "",
								hasEmptyLineFollowingFirstArg ? hardline : line,
								hasEmptyLineFollowingFirstArg ? hardline : "",
							],
							...printedArguments.slice(1),
						];
					}
					if (shouldGroupLast && i === lastArgIndex) {
						printedExpanded = [...printedArguments.slice(0, -1), print([], { expandLastArg: true })];
					}
				}, "arguments");
			});
		} catch (caught) {
			path.stack.length = 0;
			path.stack.push(...stackBackup);
			if (caught instanceof ArgExpansionBailout) return allArgsBrokenOut();
			throw caught;
		}

		return [
			printedArguments.some(willBreak) ? breakParent : "",
			conditionalGroup([
				[LEFT, ...printedExpanded, RIGHT],
				shouldGroupFirst
					? [LEFT, group(printedExpanded[0], { shouldBreak: true }), ...printedExpanded.slice(1), RIGHT]
					: [LEFT, ...printedArguments.slice(0, -1), group(printedExpanded[lastArgIndex], { shouldBreak: true }), RIGHT],
				allArgsBrokenOut(),
			]),
		];
	}

	const contents = [LEFT, indent([softline, ...printedArguments]), ifBreak(trailingComma), softline, RIGHT];

	return isLongCurriedCallExpression(node)
		? contents
		: group(contents, { shouldBreak: anyArgEmptyLine || printedArguments.some(willBreak) });

	function allArgsBrokenOut() {
		return group([LEFT, indent([line, ...printedArguments]), trailingComma, line, RIGHT], { shouldBreak: true });
	}
}

function shouldHugFunctionParameters(node: Extract<Node, FunctionLike>) {
	if (!node) return false;
	const parameters = getParameters(node);
	if (parameters.length !== 1) return false;
	const param = parameters[0];
	if (hasComment(param)) return false;
	switch (param.nodeType) {
		case NodeType.FunctionSelfParameterDeclaration:
		case NodeType.FunctionSpread:
		case NodeType.TypeFnPointerParameter:
		default:
			return false;
		case NodeType.FunctionParameterDeclaration:
		case NodeType.ClosureFunctionParameterDeclaration:
			return "items" in param.pattern || "properties" in param.pattern;
	}
}

function shouldGroupFunctionParameters(functionNode: FunctionDeclaration, returnTypeDoc: Doc) {
	const returnType = functionNode.returnType;
	const generics = functionNode.generics;
	const whereBounds = functionNode.whereBounds;
	if (!returnType) return false;
	if (generics) {
		if (generics.length > 1) return false;
		if (generics.length === 1 && !isShortGenericParameterDeclaration(generics[0])) return false;
	}
	if (whereBounds) {
		if (whereBounds.length > 1) return false;
	}
	return (
		getParameters(functionNode).length === 1 &&
		(willBreak(returnTypeDoc) || willBreak(printWhereBounds(getPrintFn(functionNode), functionNode)))
	);
}

export function printBlockBody<T extends NodeWithBodyOrCases | BlockLikeMacroInvocation>(print: print<T>, node: T): Doc {
	const body = printBodyOrCases(print, node);
	return [
		"{",
		body.length > 0
			? getBodyOrCases(node)?.length
				? canInlineBlockBody(node)
					? [indent([line, body]), line]
					: group([indent([line, body]), line], { shouldBreak: true })
				: body
			: emptyContent(node),
		"}",
	];
}

export function printMaybeBlockBody<T extends (NodeWithBodyOrCases | BlockLikeMacroInvocation) & { body: undefined | {} }>(
	print: print<T>,
	node: T
): Doc {
	return hasSemiNoBody(node) ? ";" : adjustClause(node, printBlockBody(print, node));
}

export function printArrowFunction<T extends ClosureFunctionExpression>(print: print<T>, node: T) {
	const signatures: Doc[] = [];
	const body: Doc[] = [];
	const { args, path } = getContext();
	let chainShouldBreak = false;

	let tailNode: ClosureFunctionExpression = node;
	(function rec(node: ClosureFunctionExpression) {
		tailNode = node;
		const doc = printArrowFunctionSignature(print, node);
		if (signatures.length === 0) {
			signatures.push(doc);
		} else {
			const { leading, trailing } = printCommentsSeparately();
			signatures.push([leading!, doc]);
			body.unshift(trailing!);
		}

		chainShouldBreak ||= !!node.returnType || !node.parameters.every((param) => isSimplePattern(param.pattern));

		if (!is_ClosureFunctionExpression(node.expression) || (args && args.expandLastArg)) {
			body.unshift(print("expression", args));
		} else {
			pathCall(node, "expression", rec);
		}
	})(node);

	if (signatures.length > 1) {
		return printArrowChain(signatures, chainShouldBreak, body, tailNode);
	} else {
		const printed = signatures[0];

		if (
			!hasLeadingOwnLineComment(node.expression) &&
			(is_ArrayOrTupleLiteral(node.expression) ||
				is_StructLiteral(node.expression) ||
				is_ExpressionWithBodyOrCases(node.expression) ||
				is_ClosureFunctionExpression(node.expression))
		) {
			return group([printed, " ", body]);
		}

		const shouldAddSoftLine = args && args.expandLastArg && !hasComment(node);
		const printTrailingComma = args && args.expandLastArg && false;
		const shouldAddParens = is_OrExpression(node.expression);

		return group([
			printed,
			group([
				indent(shouldAddParens ? [line, ifBreak("", "("), body, ifBreak("", ")")] : [line, body]),
				shouldAddSoftLine ? [ifBreak(printTrailingComma ? "," : ""), softline] : "",
			]),
		]);
	}
}

function printArrowChain(signatures: Doc[], shouldBreak: boolean, bodyDoc: Doc, tailNode: ClosureFunctionExpression) {
	const { args } = getContext();
	const parent = getParentNode()!;
	const isCallee = is_CallExpression_or_CallLikeMacroInvocation(parent) && parent.callee === getNode();
	const isAssignmentRhs = !!(args && args.assignmentLayout);
	const shouldPutBodyOnSeparateLine = !is_ExpressionWithBodyOrCases(tailNode.expression) && !is_StructLiteral(tailNode.expression);
	const shouldBreakBeforeChain =
		(isCallee && shouldPutBodyOnSeparateLine) || (args && args.assignmentLayout === Layout["chain-tail-arrow-chain"]);

	const groupId = Symbol("arrow-chain");

	return group([
		group(
			indent([isCallee || isAssignmentRhs ? softline : "", group(join(line, signatures), { shouldBreak })]), //
			{ id: groupId, shouldBreak: shouldBreakBeforeChain }
		),
		indentIfBreak(shouldPutBodyOnSeparateLine ? indent([line, bodyDoc]) : [" ", bodyDoc], { groupId }),
		isCallee ? ifBreak(softline, "", { groupId }) : "",
	]);
}

function printArrowFunctionSignature<T extends ClosureFunctionExpression>(print: print<T>, node: T) {
	const { args } = getContext();
	const expandArg = args && (args.expandLastArg || args.expandFirstArg);
	let returnTypeDoc: Doc = printReturnType(print, node);
	if (expandArg) {
		if (willBreak(returnTypeDoc)) throw new ArgExpansionBailout();
		else returnTypeDoc = group(removeLines(returnTypeDoc));
	}
	// const dangling = printDanglingComments(node, true, (comment) => node.parameters.loc.contains(comment));

	// if (dangling) {
	// 	parts.push(" ", dangling);
	// }
	return [
		print.b("static"),
		print.b("async"),
		print.b("move"), //
		group([printFunctionParameters(print, node, expandArg), returnTypeDoc]),
	];
}

export function printGenerics_x_whereBounds<T extends DeclarationNode>(print: print<T>, node: T, xDoc: Doc) {
	const generics: Doc = is_ImplDeclarationNode(node)
		? [printGenerics(print, node), " "]
		: [" ", print("id" as any), printGenerics(print, node)];

	const whereBoundsDoc = printWhereBounds(print, node);

	return is_TupleStructDeclaration(node)
		? [...generics, xDoc, group(whereBoundsDoc, { id: getHeritageGroupId(node) })]
		: [...generics, group([xDoc, whereBoundsDoc], { id: getHeritageGroupId(node) })];
}

export function adjustClause(
	node: DeclarationNode | ((NodeWithBodyOrCases | BlockLikeMacroInvocation) & { body: undefined | {} }),
	doc: Doc
) {
	return [
		"whereBounds" in node && (!!node.whereBounds || (hasTypeBounds(node) && node.typeBounds.length > 1)) && willBreak(doc)
			? ifBreak(hardline, " ", { groupId: getHeritageGroupId(node) })
			: " ",
		doc,
	];
}

export function printParametersAndReturnType(node: FunctionNode | TypeFunctionNode) {
	const parametersDoc = printFunctionParameters(getPrintFn(node), node);
	const returnTypeDoc = printReturnType(getPrintFn(node), node);
	return is_FunctionDeclaration(node) && shouldGroupFunctionParameters(node, returnTypeDoc)
		? group([group(parametersDoc), returnTypeDoc])
		: group([parametersDoc, returnTypeDoc]);
}

export function printFlowControlExpression<T extends ReturnExpression | BreakExpression | YieldExpression>(print: print<T>, node: T) {
	return !node.expression
		? ""
		: // : hasLeadingComment(node.expression)
		// ? [" (", indent([hardline, print("expression")]), hardline, ")"]
		is_BinaryishExpression(node.expression) && !flowControlExpressionNeedsOuterParens(node)
		? group([" ", ifBreak("("), indent([softline, print("expression")]), softline, ifBreak(")")])
		: [" ", print("expression")];

	function hasLeadingComment(node: Node) {
		if (hasLeadingOwnLineComment(node)) return true;
		if (hasNakedLeftSide(node)) {
			let leftMost = node;
			while ((leftMost = getLeftSide(leftMost))) {
				if (hasLeadingOwnLineComment(leftMost)) return true;
			}
		}
		return false;
	}
}
export function flowControlExpressionNeedsOuterParens(flow: ReturnExpression | BreakExpression | YieldExpression) {
	return (
		flow.expression &&
		(function hasLeadingComment(node: Node) {
			if (hasLeadingOwnLineComment(node)) return true;
			if (hasNakedLeftSide(node)) {
				let leftMost = node;
				while ((leftMost = getLeftSide(leftMost))) {
					if (hasLeadingOwnLineComment(leftMost)) return true;
				}
			}
			return false;
		})(flow.expression)
	);
}
function getLeftSide(node, includeAttributes = false) {
	let target: Node =
		(node as LeftRightExpression).left ??
		(node as CallExpression).callee ??
		(node as PathNode).namespace ??
		(node as ExpressionWithBody).label ??
		(node as RangeNode).lower ??
		(node as StructLiteral).struct ??
		(node as NodeWithCondition).condition ??
		(node as ExpressionBody).expression;
	if (target && includeAttributes && hasAttributes(node)) {
		(node.attributes as AttributeOrDocComment[]).forEach((attr) => {
			if (start(attr) < start(target)) target = attr;
		});
	}
	return target;
}
function hasNakedLeftSide(node: Node) {
	return (
		is_BinaryishExpression(node) ||
		is_ReassignmentNode(node) ||
		is_CallExpression_or_CallLikeMacroInvocation(node) ||
		is_MemberAccessLike(node) ||
		is_PostfixExpression(node) ||
		is_ExpressionAsTypeCast(node)
	);
}

export function printReturnType<T extends Extract<Node, MaybeReturnTypeConstraint>>(print: print<T>, node: T) {
	return node.returnType
		? is_FunctionDeclaration(node)
			? adjustDeclarationClause(node, "->", print("returnType"))
			: [" -> ", print("returnType")]
		: "";
}

function printFunctionParameters<T extends FunctionNode | TypeFunctionNode>(
	print: print<T>,
	node: T,
	expandArg = false,
	printTypeParams = false
) {
	const { left: leftDelim, right: rightDelim } = getDelimChars(node.parameters);
	const generics = printTypeParams && is_FunctionDeclaration(node) ? printGenerics(print as any, node) : "";

	if (!hasParameters(node)) {
		return [
			generics, //
			leftDelim,
			printDanglingCommentsForInline(node, DCM["parameters"]),
			rightDelim,
		];
	}

	const isParametersInTestCall = false;
	const shouldHugParameters = shouldHugFunctionParameters(node);
	const printed = print.join("parameters", sepFn);
	if (hasSelfParameter(node)) {
		printed.unshift(getContext().path.call(() => [print(), printed.length ? sepFn(node.parameters.self) : ""], "parameters", "self"));
	}

	if (expandArg) {
		if (willBreak(generics) || willBreak(printed)) throw new ArgExpansionBailout();
		return group([removeLines(generics), leftDelim, removeLines(printed), rightDelim]);
	} else if (shouldHugParameters || isParametersInTestCall) {
		return [generics, leftDelim, ...printed, rightDelim];
	} else {
		return [generics, leftDelim, indent([softline, ...printed]), softline, rightDelim];
	}
	function sepFn(param: Node) {
		return shouldHugParameters || isParametersInTestCall //
			? ", "
			: isNextLineEmpty(param)
			? [",", hardline, hardline]
			: [",", line];
	}
}

function path_try<T>(callback: () => T): T {
	const { stack } = getContext().path;
	const stackBackup = [...stack];
	try {
		return callback();
	} finally {
		stack.length = 0;
		stack.push(...stackBackup);
	}
}
function shouldGroupFirstArg(args: LocArray<ExpressionNode>): boolean {
	if (args.length !== 2) return false;
	const [firstArg, secondArg] = args;
	return (
		!hasComment(firstArg) &&
		is_ClosureFunctionExpression(firstArg) &&
		is_ExpressionWithBodyOrCases(firstArg.expression) &&
		!is_ClosureFunctionExpression(secondArg) &&
		!couldGroupArg(secondArg)
	);
}
function shouldGroupLastArg(args: LocArray<ExpressionNode>): boolean {
	const lastArg = last_of(args);
	const preLastArg = args[args.length - 2];
	return (
		!hasComment(lastArg, CF.Leading) &&
		!hasComment(lastArg, CF.Trailing) &&
		couldGroupArg(lastArg) &&
		(!preLastArg || preLastArg.nodeType !== lastArg.nodeType) &&
		(args.length !== 2 || !is_ClosureFunctionExpression(preLastArg) || !is_ArrayOrTupleLiteral(lastArg)) &&
		!(args.length > 1 && is_ArrayOrTupleLiteral(lastArg) && isConciselyPrintedArray(lastArg)) &&
		(args.length !== 1 || !is_IfBlockExpression(lastArg))
	);
}
function couldGroupArg(arg: Node, arrowChainRecursion = false): boolean {
	return (
		(is_StructLiteral(arg) && (arg.properties.length > 0 || hasComment(arg))) ||
		(is_ArrayOrTupleLiteral(arg) && (arg.items.length > 0 || hasComment(arg))) ||
		(is_ExpressionAsTypeCast(arg) && couldGroupArg(arg.expression)) ||
		(is_ClosureFunctionExpression(arg) &&
			(!arg.returnType || is_Identifier(arg.returnType) || !isNonEmptyBlockStatement(arg.expression)) &&
			(isNonEmptyBlockStatement(arg.expression) ||
				(is_ClosureFunctionExpression(arg.expression) && couldGroupArg(arg.expression, true)) ||
				is_StructLiteral(arg.expression) ||
				is_ArrayOrTupleLiteral(arg.expression) ||
				(!arrowChainRecursion && is_CallExpression_or_CallLikeMacroInvocation(arg.expression)))) ||
		is_ExpressionWithBodyOrCases(arg) //&& isNonEmptyBlockStatement(arg)
	);
}
function isNonEmptyBlockStatement(node: Node) {
	if (is_MatchExpression(node)) return node.cases.length > 0;
	return is_ExpressionWithBodyOrCases(node) && node.body.length > 0;
}
function isFunctionCompositionArgs(args) {
	if (args.length <= 1) {
		return false;
	}
	let count = 0;
	for (const arg of args) {
		if (is_ClosureFunctionExpression(arg)) {
			if (++count > 1) return true;
		} else if (is_CallExpression_or_CallLikeMacroInvocation(arg)) {
			for (const childArg of arg.arguments) {
				if (is_ClosureFunctionExpression(childArg)) {
					return true;
				}
			}
		}
	}
	return false;
}

export function printBinaryishExpression<T extends AndExpression | OrExpression | ComparisonExpression | OperationExpression>(
	print: print<T>,
	node: T
) {
	const parent = getParentNode()!;
	const grandParent = getGrandParentNode();
	const isInsideParenthesis = ("condition" in parent && parent.condition === node) || is_MatchExpression(parent);

	const parts = printBinaryishExpressions(false, isInsideParenthesis);
	if (isInsideParenthesis) return parts;
	if (
		(is_CallExpression_or_CallLikeMacroInvocation(parent) && parent.callee === node) || //
		is_UnaryExpression(parent) ||
		is_MemberExpression(parent)
	) {
		return group([indent([softline, ...parts]), softline]);
	}

	const shouldNotIndent =
		is_FlowControlExpression(parent) ||
		(is_ClosureFunctionExpression(parent) && parent.expression === node) ||
		is_ExpressionWithBodyOrCases(parent);

	const shouldIndentIfInlining =
		is_ReassignmentNode(parent) || is_VariableDeclarationNode(parent) || is_StructLiteral(parent) || is_StructLiteral(grandParent);

	const samePrecedenceSubExpression = is_BinaryishExpression(node.left) && shouldFlatten(node, node.left);

	if (
		shouldNotIndent ||
		(shouldInlineLogicalExpression(node) && !samePrecedenceSubExpression) ||
		(!shouldInlineLogicalExpression(node) && shouldIndentIfInlining)
	) {
		return group(parts);
	}

	if (parts.length === 0) return "";
	const firstGroupIndex = parts.findIndex((part) => typeof part !== "string" && !Array.isArray(part) && part.type === "group");
	const leading = parts.slice(0, firstGroupIndex === -1 ? 1 : firstGroupIndex + 1);
	return group([...leading, indent(parts.slice(leading.length))], { id: Symbol("logicalChain") });

	function printBinaryishExpressions(isNested: boolean, isInsideParenthesis: boolean): Doc[] {
		const { path, print, options } = getContext();
		const node = path.getValue();

		if (!is_BinaryishExpression(node)) {
			return [group(print())];
		}

		const parts: Doc[] = [];

		if (shouldFlatten(node, node.left)) {
			parts.push(...pathCall(node, "left", () => printBinaryishExpressions(true, isInsideParenthesis)));
		} else {
			parts.push(group(print("left")));
		}

		const shouldInline = shouldInlineLogicalExpression(node);
		const operator = node.kind;

		const right = [
			operator,
			shouldInline ? " " : line,
			// this is a hack (should always be 'print("right")')
			!shouldInline && is_LogicalExpression(node.right) && shouldFlatten(node.right, node)
				? pathCall(node, "right", () => printBinaryishExpressions(true, isInsideParenthesis))
				: print("right"),
		];

		const shouldBreak = hasComment(node.left, CF.Trailing | CF.Line);
		const shouldGroup =
			shouldBreak ||
			(!(isInsideParenthesis && is_LogicalExpression(node)) &&
				path.getParentNode()!.nodeType !== node.nodeType &&
				node.left.nodeType !== node.nodeType &&
				node.right.nodeType !== node.nodeType);

		parts.push(" ", shouldGroup ? group(right, { shouldBreak }) : right);

		if (isNested && hasComment(node)) {
			const printed = cleanDoc(withComments(node, parts));
			if (isConcat(printed) || (printed as any).type === "fill") {
				return getDocParts(printed) as Doc[];
			}

			return [printed];
		}

		return parts;
	}
}

// export function printLogicalExpression<T extends AndExpression | OrExpression>(print: print<T>, node: T) {
// 	if (!is_insideScrutinee(node)) return printBinaryishExpression(print, node);

// }

function shouldInlineLogicalExpression(node: Node) {
	if (is_LogicalExpression(node)) {
		if (is_StructLiteral(node.right)) return node.right.properties.length > 0;
		if (is_ArrayOrTupleLiteral(node.right)) return node.right.items.length > 0;
	}
	return false;
}

export function printUnaryExpression<T extends UnaryExpression>(leftDoc: Doc, node: T) {
	__DEV__: assert(is_UnaryExpression(node));
	// if (unaryNeedsOuterParens(node)) {
	// 	return [leftDoc, group(["(", indent([softline, getPrintFn<UnaryExpression>()("expression")]), softline, ")"])];
	// } else {
	const printed = getPrintFn(node)("expression");

	return group([leftDoc, printed]);
	// }
}

export function printIfBlock<T extends IfBlockExpression>(print: print<T>, node: T) {
	let printed: Doc = [
		printIfBlockCondition(print, node), //
		printBlockBody(print, node),
		f` else ${print("else")}`,
	];

	const parent = getParentNode()!;

	if (is_ClosureBlock(node, parent)) {
		printed = parenthesize_if_break([indent([softline, printed]), softline]);
	} else if (!is_ElseBlock(node, parent)) {
		// if (needsParens(node)) {
		// 	printed = group(printed);
		// }
		// if (needsParens(node)) {
		// 	printed = group([indent([softline, printed]), softline]);
		// } else {
		// printed = group(printed);
		// }
	}

	return printed;
}

export function printIfBlockCondition<T extends IfBlockExpression | MatchExpressionCase>(print: print<T>, node: T) {
	if (!hasCondition(node)) return "";
	// const id = Symbol("if");
	// return ["if ", group(printCondition(print, node), { id }), ifBreak("", " ", { groupId: id })];
	return f`if ${printCondition(print, node)}`;
}

// export function printMatchExpressionExpression<T extends MatchExpression>(print: print<T>, node: T) {
// 	return ["match ", maybe_parenthesize_condition(print, node), " "];
// }

export function printCondition<T extends IfBlockExpression | MatchExpressionCase | WhileBlockExpression>(print: print<T>, node: T) {
	return pathCall(node, "condition", (condition) => {
		if (!condition) return "";
		if (needsParens(condition)) return [print(), " "];
		const id = Symbol("condition");
		const softlineStart = true; //!is_LetScrutinee(condition); //!is_LetScrutinee(getLeftMostCondition(condition));
		const printed = [indent([softlineStart ? softline : "", print()]), softline];
		return [group(printed, { id }), ifBreak("", " ", { groupId: id })];
		return hasLetScrutineeCondition(node)
			? printed // parenthesizing nested "let" throws 'unused_parens'
			: parenthesize_if_break(printed);
	});
}
export function parenthesize_if_break(doc: Doc) {
	return conditionalGroup([doc, ["(", doc, ")"]], { shouldBreak: willBreak(doc) });
}
function unwrapParenthesized(doc: Doc) {
	__DEV__: {
		const check = (doc: Doc) => Array.isArray(doc) && doc.length === 3 && doc[0] === "(" && doc[2] === ")";
		assert(Array.isArray(doc) && check(doc.length === 1 ? doc[0] : doc), "Expected ['(', Doc, ')']", doc);
	}
	return doc.length === 1 ? doc[0][1] : doc[1];
}
export function withParensIdentOnBreak(node: Node, printed: Doc) {
	return printed;
	const parens = needsParens(node);
	const grouped = group([indent([softline, parens ? unwrapParenthesized(printed) : printed]), softline]);
	return conditionalGroup([printed, parens ? ["(", grouped, ")"] : grouped], { shouldBreak: willBreak(printed) });
}
function isSimplePattern(node: PatternNode | undefined) {
	if (!node) return false;
	switch (node.nodeType) {
		case NodeType.MacroInvocation:
			return false;
		case NodeType.ExpressionTypeCast:
			return isSimplePattern(node.typeCallee) && !hasComplexTypeArguments(node);
		case NodeType.ExpressionTypeSelector:
			return is_Identifier(node.typeTarget) && (!node.typeExpression || is_Identifier(node.typeExpression));
		case NodeType.ExpressionPath:
			return !node.namespace || isSimplePattern(node.namespace);
		case NodeType.RangePattern:
			return (!node.lower || isSimplePattern(node.lower)) && (!node.upper || isSimplePattern(node.upper));
		case NodeType.PatternVariableDeclaration:
		case NodeType.ReferencePattern:
		case NodeType.BoxPattern:
		case NodeType.MinusPattern:
			return isSimplePattern(node.pattern);
		case NodeType.Identifier:
		case NodeType.Literal:
		case NodeType.RestPattern:
		case NodeType.WildcardPattern:
			return true;
		default:
			return false;
	}
}

export function printUnionPattern<T extends UnionPattern>(print: print<T>, node: T) {
	if (node.patterns.length === 1) return print.map("patterns");
	const parent = getParentNode();
	const prebreak = parent && (is_VariableDeclarationNode(parent) || is_LetScrutinee(parent)) && !needsParens(node);
	return group([
		prebreak ? softline : "",
		print.map("patterns", (node, i, arr) => [
			withComments(node, [
				i === 0 //
					? ifBreak("| ")
					: "| ",
				align(2, print()),
			]),
			i === arr.length - 1 ? "" : line,
		]),
	]);
}

export function printArrayLike<T extends ArrayLikeNode>(print: print<T>, node: T) {
	const delims = getDelimChars(node.items);
	if (node.items.length === 0) {
		const comments = printDanglingCommentsForInline(node, DCM["items"]);
		return comments ? group([delims.left, comments, delims.right]) : delims.left + delims.right;
	}

	const groupId = Symbol("array");
	const shouldBreak =
		// is_TupleStructDeclaration(node) ||
		!is_TupleNode(node) &&
		node.items.length > 1 &&
		(node.items as (ExpressionNode | PatternNode)[]).every((item, i) => {
			const next = node.items[i + 1];
			return (
				((hasProperties(item) && item.properties.length > 1) || (hasItems(item) && item.items.length > 1)) &&
				(!next || item.nodeType === next.nodeType)
			);
		});
	const shouldUseConciseFormatting = isConciselyPrintedArray(node);

	const parent = getParentNode(node)!;
	const needsForcedTrailingComma =
		node.items.length === 1
			? is_TupleLiteral(node)
				? is_RangeLiteral(node.items[0])
					? !(is_ReassignmentExpression(parent) && parent.left === node)
					: true
				: is_TuplePattern(node)
				? !node.struct && !is_RangePattern(node.items[0]) && !is_RestPattern(node.items[0])
				: is_TypeTuple(node)
				? true
				: false
			: false;
	const trailingComma: Doc = needsForcedTrailingComma
		? "," //
		: shouldUseConciseFormatting
		? ifBreak(",", "", { groupId })
		: ifBreak(",");

	const printed = shouldUseConciseFormatting //
		? fill(
				print.join(
					"items",
					(item, next) =>
						isNextLineEmpty(item)
							? [",", hardline, hardline]
							: hasComment(next, CF.Leading, (comment) => is_LineCommentNode(comment) || comment.placement === "ownLine")
							? [",", hardline]
							: [",", line],
					trailingComma
				)
		  )
		: print.map_join(
				"items",
				() => group(print()),
				(item) => (isNextLineEmpty(item) ? [",", line, softline] : [",", line]),
				trailingComma
		  );

	return group([delims.left, indent([softline, printed]), printDanglingComments(node, true, DCM["items"]), softline, delims.right], {
		shouldBreak,
		id: groupId,
	});
}

export function printObject<T extends ObjectNode>(print: print<T>, node: T): Doc {
	if (hasSemiNoProperties(node)) {
		return ";";
	}
	if (!hasProperties(node)) {
		return [" {", printDanglingCommentsForInline(node, DCM["properties"]) || emptyContent(node), "}"];
	}

	const firstProperty = node.properties[0];

	const parent = getParentNode()!;

	const shouldBreak = is_StructPattern(node)
		? false
		: is_UnionDeclaration(node) ||
		  is_StructDeclaration(node) ||
		  is_EnumMemberStructDeclaration(node) ||
		  hasNewlineInRange(start(node), start(firstProperty));

	const content = [
		" {",
		indent([
			line,
			...print.join(
				"properties", //
				(node) => (isNextLineEmpty(node) ? [",", hardline, hardline] : [",", line]),
				(node) => (is_StructSpread(node) ? "" : ifBreak(","))
			),
		]),
		line,
		"}",
	];

	const grandparent = getGrandParentNode();
	if (
		grandparent &&
		(is_FunctionDeclaration(grandparent) || is_ClosureFunctionExpression(grandparent)) &&
		getParameters(grandparent)[0] === parent
	) {
		return content;
	}

	if (
		(is_StructLiteral(node) && is_ReassignmentNode(parent) && parent.left === node) ||
		(is_StructPattern(node) &&
			(is_VariableDeclarationNode(parent) || is_MatchExpressionCase(parent) || is_FunctionParameterDeclaration(parent)) &&
			parent.pattern === node)
	) {
		return content;
	}

	return group(content, { shouldBreak });
}

export function printEnumBody<T extends EnumDeclaration>(print: print<T>, node: T): Doc {
	const printed = print.join("members", (member) => [",", maybeEmptyLine(member)], ",");
	return [
		" {",
		printed.length === 0
			? printDanglingCommentsForInline(node, DCM["members"]) || emptyContent(node)
			: [indent([hardline, ...printed]), hardline],
		"}",
	];
}
