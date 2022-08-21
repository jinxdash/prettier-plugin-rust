import { CommentOrDocComment, Node, NodeType, NodeWithBodyOrCases } from "jinx-rust";
import {
	end,
	getBodyOrCases,
	getLastParameter,
	hasOuterAttributes,
	isInner,
	is_Attribute,
	is_AttributeOrDocComment,
	is_BlockCommentKind,
	is_BlockCommentNode,
	is_Comment,
	is_CommentOrDocComment,
	is_ExpressionWithBodyOrCases,
	is_ExternSpecifier,
	is_FlowControlExpression,
	is_FunctionDeclaration,
	is_FunctionNode,
	is_IfBlockExpression,
	is_LineCommentKind,
	is_LineCommentNode,
	is_MacroRule,
	is_NodeWithBodyOrCases,
	is_ReassignmentNode,
	is_StatementNode,
	is_StructLiteralProperty,
	is_StructLiteralPropertySpread,
	nisAnyOf,
	ownStart,
	start,
} from "jinx-rust/utils";
import { assert, exit, iLast, last_of, maybe_last_of, Narrow } from "../utils/common";
import { is_MemberAccessLike, is_xVariableEqualishLike } from "./core";
import {
	AnyComment,
	breakParent,
	cursor,
	CustomOptions,
	DCM,
	Doc,
	hardline,
	indent,
	join,
	line,
	lineSuffix,
	literalline,
	MutatedAttribute,
	NodeWithComments,
	PrettierCommentInfo,
} from "./external";
import { assertPathAtNode, getAllComments, getContext, getNode, getOptions, pathCallEach } from "./plugin";
import { shouldPrintOuterAttributesAbove } from "./styling";
import { is_CallExpression_or_CallLikeMacroInvocation } from "./transform";

function addCommentHelper(node: Node, comment: AnyComment, leading = false, trailing = false) {
	__DEV__: assert(!handled(comment));
	((node as NodeWithComments<Node>).comments ??= []).push(comment);
	(comment.leading = leading), (comment.trailing = trailing), (comment.printed = false);
}

function addLeadingComment(node: Node, comment: AnyComment) {
	addCommentHelper(node, comment, true);
}
function addDanglingComment(node: Node, comment: AnyComment, marker: DCM) {
	addCommentHelper(node, comment);
	comment.marker = marker;
}
function addTrailingComment(node: Node, comment: AnyComment) {
	addCommentHelper(node, comment, false, true);
}
export function setPrettierIgnoreTarget(node: Node, comment: AnyComment) {
	__DEV__: Narrow<Node & { prettierIgnore?: true }>(node), assert(isPrettierIgnoreComment(comment) || isPrettierIgnoreAttribute(comment));
	comment.unignore = true;
	node.prettierIgnore = true;
}

function hasComments<T extends Node>(node: T): node is NodeWithComments<T> {
	return "comments" in node && node.comments.length > 0;
}

export function printDanglingComments(enclosingNode: Node, sameIndent: boolean, marker?: DCM) {
	if (hasComments(enclosingNode)) {
		const printed: Doc[] = [];
		pathCallEach(enclosingNode, "comments", (comment) => {
			if (isDangling(comment) && (!marker || comment.marker === marker)) {
				printed.push(printComment(comment));
			}
		});
		if (printed.length > 0) {
			return sameIndent //
				? join(hardline, printed)
				: indent([hardline, join(hardline, printed)]);
		}
	}
	return "";
}

export function needsHardlineAfterDanglingComment(node: Node) {
	if (!hasComment(node)) return false;
	const lastDanglingComment = maybe_last_of(getComments(node, CF.Dangling));
	return lastDanglingComment && is_LineCommentNode(lastDanglingComment);
}
export function setDidPrintComment(comment: AnyComment) {
	comment.printed = true;
}

function printComment(comment: AnyComment) {
	__DEV__: assertPathAtNode("printComment", comment);
	__DEV__: assert(handled(comment), `Assertion failed: Comment was not printed at ${comment.loc.url()}`, comment);
	setDidPrintComment(comment);
	return getContext().options.printer.printComment!(getContext().path as any, getOptions());
}

export function isPreviousLineEmpty(node: Node) {
	let index = start(node) - 1;
	index = skipSpaces(index, true) as number;
	index = skipNewline(index, true) as number;
	index = skipSpaces(index, true) as number;
	return index !== skipNewline(index, true);
}
export function hasBreaklineBefore(node: Node) {
	return hasNewline(start(node) - 1, true);
}

export function hasBreaklineAfter(node: Node) {
	return hasNewline(end(node));
}

export function printCommentsSeparately(ignored?: Set<AnyComment>) {
	const node = getNode();
	__DEV__: Narrow<Node & { comments?: AnyComment[] }>(node);

	const leading: Doc[] = [];
	const trailing: Doc[] = [];
	let hasTrailingLineComment = false;
	let hadLeadingBlockComment = false;

	if ("comments" in node) {
		pathCallEach(node, "comments", (comment) => {
			if (ignored?.has(comment)) {
				return;
			} else if (isLeading(comment)) {
				leading.push(printLeadingComment(comment));
			} else if (isTrailing(comment)) {
				trailing.push(printTrailingComment(comment));
			}
		});
	}

	if (node === getOptions().cursorNode) {
		leading.unshift(cursor);
		trailing.push(cursor);
	}

	return (leading.length | trailing.length) > 0 ? { leading, trailing } : ({ leading: "", trailing: "" } as const);

	function printLeadingComment(comment: AnyComment) {
		if (is_Attribute(comment) && !comment.inner) {
			const printed = printComment(comment);
			return [printed, " "];
		}
		hadLeadingBlockComment ||= is_BlockCommentKind(comment) && hasBreaklineBefore(comment);
		return [
			printComment(comment),
			is_BlockCommentKind(comment)
				? hasBreaklineAfter(comment) //
					? hadLeadingBlockComment
						? hardline
						: line
					: " "
				: hardline,
			hasNewline(skipNewline(skipSpaces(end(comment)))) ? hardline : "",
		];
	}

	function printTrailingComment(comment: AnyComment) {
		const printed = printComment(comment);
		return hasBreaklineBefore(comment)
			? lineSuffix([hardline, isPreviousLineEmpty(comment) ? hardline : "", printed])
			: is_BlockCommentNode(comment)
			? [" ", printed]
			: lineSuffix([" ", printed, hasTrailingLineComment === (hasTrailingLineComment = true) ? hardline : breakParent]);
	}
}

export function getPostLeadingComment(comment: AnyComment) {
	// console.log(comment.loc.url());
	// is_BlockCommentKind(comment)
	// 	? hasBreaklineAfter(comment) //
	// 		? hasBreaklineBefore(comment)
	// 			? hardline
	// 			: line
	// 		: " "
	// 	: hardline,
	return hasNewline(skipNewline(skipSpaces(end(comment)))) ? hardline : "";
}

export function withComments<D extends Doc>(node: Node, printed: D, ignored?: Set<AnyComment>): D | Doc[] {
	__DEV__: assertPathAtNode("withComments", node);
	const { leading, trailing } = printCommentsSeparately(ignored);
	return leading || trailing ? [...leading!, printed, ...trailing!] : printed;
	// return needsOuterParens(node) ? group(["(", indent([softline, parts]), softline, ")"]) : parts;
	// return parts;
}
export function getComments(node: Node, ...args: Parameters<typeof getCommentTestFunction>): AnyComment[] {
	__DEV__: Narrow<Node & { comments?: AnyComment[] }>(node);
	// if (!node || !node.comments) return [];
	// if (args.length === 0) return node.comments;
	// return args.length > 0 ? node.comments.filter(getCommentTestFunction(...args)) : node.comments;
	return node && node.comments //
		? args.length > 0
			? node.comments.filter(getCommentTestFunction(...args))
			: node.comments
		: [];
}

export function getFirstComment(node: Node, flags: CF, fn?: (comment: AnyComment) => boolean): AnyComment | undefined {
	const r = getComments(node, flags | CF.First, fn);
	return r.length === 0 ? undefined : r[0];
}

export function escapeComments(flags: number, fn?: (comment: AnyComment) => boolean) {
	const comments = getAllComments().filter(getCommentTestFunction(flags, fn)) as AnyComment[];
	comments.forEach(setDidPrintComment);
	return new Set(comments);
}

export const enum CF {
	Leading = 1 << 1,
	Trailing = 1 << 2,
	Dangling = 1 << 3,
	Block = 1 << 4,
	Line = 1 << 5,
	PrettierIgnore = 1 << 6,
	First = 1 << 7,
	Last = 1 << 8,
}
export function isPrettierIgnoreComment(comment: AnyComment) {
	return is_Comment(comment) && /^\s*prettier-ignore\s*/.test(comment.value) && !comment.unignore;
}
export function isPrettierIgnoreAttribute(node: Node): node is MutatedAttribute {
	return is_Attribute(node) && /^\s*rustfmt::skip\s*$/.test(node.value);
}
function getCommentTestFunction(flags: CF, fn?: (comment: AnyComment) => boolean) {
	return function (comment: AnyComment, index: number, comments: AnyComment[]) {
		__DEV__: Narrow<number>(flags), assert(handled(comment));
		return !(
			(flags & CF.Leading && !isLeading(comment)) ||
			(flags & CF.Trailing && !isTrailing(comment)) ||
			(flags & CF.Dangling && !isDangling(comment)) ||
			(flags & CF.Block && !is_BlockCommentKind(comment)) ||
			(flags & CF.Line && !is_LineCommentKind(comment)) ||
			(flags & CF.First && index !== 0) ||
			(flags & CF.Last && !iLast(index, comments)) ||
			(flags & CF.PrettierIgnore && !(isPrettierIgnoreComment(comment) || isPrettierIgnoreAttribute(comment))) ||
			(fn && !fn(comment))
		);
	};
}

export function hasComment(node: Node, flags: number = 0, fn?: (comment: AnyComment) => boolean) {
	if ("comments" in node && node.comments!.length > 0) {
		return flags || fn ? (node.comments as AnyComment[]).some(getCommentTestFunction(flags, fn)) : true;
	}
	return false;
}
export function hasNewlineInRange(leftIndex: number, rightIndex: number) {
	__DEV__: assert(leftIndex <= rightIndex);
	const text = getContext().options.originalText;
	for (var i = leftIndex; i < rightIndex; ++i) if (text.charCodeAt(i) === 10) return true;
	return false;
}
export function isNextLineEmpty(node: Node) {
	return isNextLineEmptyAfterIndex(end(node));
}
export function isNextLineEmptyAfterIndex(index: number | false) {
	let oldIdx: number | false = -1;
	let idx: number | false = index;
	while (idx !== oldIdx) {
		oldIdx = idx;
		idx = skipToLineEnd(idx);
		idx = skipBlockComment(idx);
		idx = skipSpaces(idx);
		idx = skipParens(idx);
	}
	idx = skipLineComment(idx);
	idx = skipParens(idx);
	idx = skipNewline(idx);
	idx = skipParens(idx);
	return idx !== false && hasNewline(idx);
}
export function hasNewline(index: number | false, backwards = false) {
	if (index === false) return false;
	const i = skipSpaces(index, backwards);
	return i !== false && i !== skipNewline(i, backwards);
}
function skipLineComment(index: number | false) {
	if (index === false) return false;
	const { commentSpans, originalText } = getContext().options;
	if (commentSpans.has(index) && originalText.charCodeAt(index + 1) === 47 /** "/" */)
		return skipEverythingButNewLine(commentSpans.get(index)!);
	return index;
}
function skipBlockComment(index: number | false) {
	if (index === false) return false;
	const { commentSpans, originalText } = getContext().options;
	if (commentSpans.has(index) && originalText.charCodeAt(index + 1) === 42 /** "*" */) return commentSpans.get(index)!;
	return index;
}
const [skipSpaces, skipToLineEnd, skipEverythingButNewLine] = [/[ \t]/, /[,; \t]/, /[^\r\n]/].map(function (re) {
	return function (index: number | false, backwards = false) {
		if (index === false) return false;
		const { originalText: text } = getContext().options;
		let cursor = index;
		while (cursor >= 0 && cursor < text.length) {
			if (re.test(text.charAt(cursor))) backwards ? cursor-- : cursor++;
			else return cursor;
		}
		return cursor === -1 || cursor === text.length ? cursor : false;
	};
});

function skipNewline(index: number | false, backwards = false) {
	if (index === false) return false;
	const { originalText } = getContext().options;
	const atIndex = originalText.charCodeAt(index);
	if (backwards) {
		if (originalText.charCodeAt(index - 1) === 13 && atIndex === 10) return index - 2;
		if (atIndex === 10) return index - 1;
	} else {
		if (atIndex === 13 && originalText.charCodeAt(index + 1) === 10) return index + 2;
		if (atIndex === 10) return index + 1;
	}
	return index;
}

function skipParens(index: number | false, backwards = false) {
	return index;
	// if (index === false) return false;
	// const { parensPositions } = getContext().options;
	// while (parensPositions.has(index)) backwards ? index-- : index++;
	// return index;
}

export function getNextNonSpaceNonCommentCharacterIndex(node: Node) {
	return getNextNonSpaceNonCommentCharacterIndexWithStartIndex(end(node));
}
function getNextNonSpaceNonCommentCharacterIndexWithStartIndex(i: number) {
	let oldIdx = -1;
	let nextIdx = i;
	while (nextIdx !== oldIdx) {
		oldIdx = nextIdx;
		nextIdx = skipSpaces(nextIdx) as number;
		nextIdx = skipBlockComment(nextIdx) as number;
		nextIdx = skipLineComment(nextIdx) as number;
		nextIdx = skipNewline(nextIdx) as number;
		nextIdx = skipParens(nextIdx) as number;
	}
	return nextIdx;
}
export function getNextNonSpaceNonCommentCharacter(node: Node) {
	return getContext().options.originalText.charAt(getNextNonSpaceNonCommentCharacterIndex(node));
}

interface CommentContext {
	comment: AnyComment;
	precedingNode: Node | undefined;
	enclosingNode: Node | undefined;
	followingNode: Node | undefined;
	text: string;
	options: CustomOptions;
	ast: Node;
	isLastComment: boolean;
}

function handled(comment: AnyComment) {
	return "printed" in comment;
}
function handleCommon(ctx: CommentContext): boolean {
	{
		const { comment, precedingNode, enclosingNode, followingNode } = ctx;
		if (!enclosingNode) {
			ctx.enclosingNode = ctx.comment.loc.src.program;
		} else if (enclosingNode && is_NodeWithBodyOrCases(enclosingNode)) {
			const body = getBodyOrCases(enclosingNode);
			if (body) {
				if (is_ExpressionWithBodyOrCases(enclosingNode) && enclosingNode.label) {
					if (ctx.precedingNode === enclosingNode.label) {
						ctx.precedingNode = undefined;
					}
					if (followingNode === enclosingNode.label) {
						ctx.followingNode = undefined;
					}
				}
				if (comment.loc.isBefore(body)) {
					if (followingNode && body.loc.contains(followingNode)) {
						ctx.followingNode = undefined;
					}
					if (!ctx.precedingNode && !ctx.followingNode) {
						addLeadingComment(enclosingNode, comment);
						return true;
					}
				} else if (comment.loc.isAfter(body)) {
					if (precedingNode && body.loc.contains(precedingNode)) {
						ctx.precedingNode = undefined;
					}
					if (!ctx.precedingNode && !ctx.followingNode) {
						addTrailingComment(enclosingNode, comment);
						return true;
					}
				} else if (body.loc.contains(comment)) {
					if (precedingNode && !body.loc.contains(precedingNode)) {
						ctx.precedingNode = undefined;
					}
					if (followingNode && !body.loc.contains(followingNode)) {
						ctx.followingNode = undefined;
					}
				}
			}
		}
	}
	for (const fn of [
		handleMixedInOuterAttributeComments,
		handleAttributeComments,
		handleDanglingComments,
		handleFunctionComments,
		handleMacroRuleComments,
		handleStructLiteralComments,
		handleVariableDeclaratorComments,
		handleIfBlockExpressionComments,
		handleMemberExpressionComments,
		handleStatementComments,
		handleFlowControlComments,
		handleBadComments,
	]) {
		fn(ctx);
		if (handled(ctx.comment)) {
			// console.log(ctx.comment.loc.url(), fn.name);
			return true;
		}
	}

	const { precedingNode, followingNode, comment } = ctx;

	if (isStartOfLine(comment)) {
		if (followingNode) {
			addLeadingComment(followingNode, comment);
		} else if (precedingNode) {
			addTrailingComment(precedingNode, comment);
		} else {
			exit.never(ctx);
		}
	} else if (isEndOfLine(comment)) {
		if (precedingNode) {
			addTrailingComment(precedingNode, comment);
		} else if (followingNode) {
			addLeadingComment(followingNode, comment);
		} else {
			exit.never(ctx);
		}
	} else {
		if (precedingNode && followingNode) {
			return false;
		} else if (precedingNode) {
			addTrailingComment(precedingNode, comment);
		} else if (followingNode) {
			addLeadingComment(followingNode, comment);
		} else {
			exit.never(ctx);
		}
	}
	return handled(ctx.comment);
}
export function handleOwnLineComment(ctx: CommentContext) {
	return handleCommon(ctx);
}
export function handleEndOfLineComment(ctx: CommentContext) {
	const { precedingNode, enclosingNode, followingNode, comment } = ctx;
	if (
		// handleCallExpressionComments
		precedingNode &&
		enclosingNode &&
		is_CallExpression_or_CallLikeMacroInvocation(enclosingNode) &&
		enclosingNode.arguments.length > 0 &&
		precedingNode === (enclosingNode.typeArguments ? last_of(enclosingNode.typeArguments) : enclosingNode.callee)
	) {
		addLeadingComment(enclosingNode.arguments[0], comment);
		return true;
	} else if (
		// handlePropertyComments
		enclosingNode &&
		is_StructLiteralProperty(enclosingNode)
	) {
		addLeadingComment(enclosingNode, comment);
		return true;
	} else {
		return handleCommon(ctx);
	}
}

export function handleRemainingComment(ctx: CommentContext) {
	return handleCommon(ctx);
}

function handleStructLiteralComments({ precedingNode, enclosingNode, followingNode, comment, ast }: CommentContext) {
	if (enclosingNode && is_StructLiteralPropertySpread(enclosingNode) && followingNode === enclosingNode.expression) {
		addLeadingComment(enclosingNode, comment);
	}
}

function handleVariableDeclaratorComments({ precedingNode, enclosingNode, followingNode, comment, ast }: CommentContext) {
	if (
		enclosingNode &&
		(is_xVariableEqualishLike(enclosingNode) || is_ReassignmentNode(enclosingNode)) &&
		followingNode &&
		(is_BlockCommentKind(comment) ||
			nisAnyOf(followingNode, [
				NodeType.StructLiteral,
				NodeType.StructPattern,
				NodeType.TupleLiteral,
				NodeType.TypeTuple,
				NodeType.TuplePattern,
				NodeType.ArrayLiteral,
				NodeType.ArrayPattern,
				NodeType.SizedArrayLiteral,
				NodeType.TypeSizedArray,
			]))
	) {
		addLeadingComment(followingNode, comment);
	}
}

function handleMixedInOuterAttributeComments({ precedingNode, enclosingNode, followingNode, comment, ast }: CommentContext) {
	if (enclosingNode && hasOuterAttributes(enclosingNode) && end(comment) <= ownStart(enclosingNode)) {
		if (isPrettierIgnoreComment(comment) || isPrettierIgnoreAttribute(comment)) {
			setPrettierIgnoreTarget(enclosingNode, comment);
		}
		if (isEndOfLine(comment)) {
			__DEV__: assert(!!precedingNode && is_Attribute(precedingNode), "", precedingNode);
			if (shouldPrintOuterAttributesAbove(enclosingNode)) {
				// #[attr] // comment
				// node
				addTrailingComment(precedingNode, comment);
			} else {
				// #[attr] /* comment */ node
				addLeadingComment(followingNode || enclosingNode, comment);
			}
		} else {
			// __DEV__: assert(isStartOfLine(comment));
			if (followingNode && end(followingNode) <= ownStart(enclosingNode)) {
				addLeadingComment(followingNode, comment);
			} else if (precedingNode && enclosingNode.loc.contains(precedingNode)) {
				addTrailingComment(precedingNode, comment);
			} else {
				addLeadingComment(enclosingNode, comment);
			}
		}
	}
}
function handleAttributeComments({ precedingNode, enclosingNode, followingNode, comment, ast }: CommentContext) {
	if (is_AttributeOrDocComment(comment)) {
		if (comment.inner) {
			if (
				enclosingNode &&
				is_FunctionDeclaration(enclosingNode) &&
				(!followingNode || !is_StatementNode(followingNode)) &&
				(!precedingNode || !is_StatementNode(precedingNode))
			) {
				if (enclosingNode.body) {
					if (enclosingNode.body.length === 0) {
						addDanglingComment(enclosingNode, comment, DCM["body"]);
					} else {
						addLeadingComment(enclosingNode.body[0], comment);
					}
				} else {
					addLeadingComment(enclosingNode, comment);
				}
			} else {
				if (followingNode) {
					addLeadingComment(followingNode, comment);
				} else {
					addDanglingComment(enclosingNode || ast, comment, DCM["body"]);
				}
			}
		} else {
			// if (comment.loc.url().startsWith("tests/samples/macro/attr.rs") && getContext().options.danglingAttributes.includes(comment)) {
			// 	// debugger;
			// 	console.log({
			// 		comment: comment.loc.url(),
			// 		precedingNode: precedingNode?.loc.url(),
			// 		enclosingNode: enclosingNode?.loc.url(),
			// 		followingNode: followingNode?.loc.url(),
			// 	});
			// }
			if (followingNode) {
				addLeadingComment(followingNode, comment);
			} else {
				addDanglingComment(enclosingNode || ast, comment, DCM["body"]);
			}
		}
	}
}

function handleBadComments({ precedingNode, enclosingNode, followingNode, ast, comment }: CommentContext) {
	if (!enclosingNode) {
		// console.log(comment.loc.url());
		if (followingNode) {
			addLeadingComment(followingNode, comment);
		} else if (precedingNode) {
			addTrailingComment(precedingNode, comment);
		} else {
			addDanglingComment(enclosingNode || ast, comment, DCM["body"]);
		}
	} else if (!precedingNode && !followingNode) {
		if (enclosingNode && enclosingNode !== ast) {
			addLeadingComment(enclosingNode, comment);
		} else {
			addDanglingComment(ast, comment, DCM["body"]);
		}
	}
}
function is_ABI_Comment({ precedingNode, enclosingNode, comment }: CommentContext) {
	return (
		is_CommentOrDocComment(comment) &&
		((precedingNode && is_ExternSpecifier(precedingNode)) || (enclosingNode && is_ExternSpecifier(enclosingNode)))
	);
}
function handleFlowControlComments({ precedingNode, enclosingNode, followingNode, comment }: CommentContext) {
	if (enclosingNode && is_FlowControlExpression(enclosingNode)) {
		if (!precedingNode && (isOwnLine(comment) || isEndOfLine(comment)) && !followingNode) {
			addLeadingComment(enclosingNode, comment);
		}
	}
}
function handleFunctionComments(ctx: CommentContext) {
	const { precedingNode, enclosingNode, followingNode, comment } = ctx;
	if (enclosingNode && is_FunctionNode(enclosingNode)) {
		if (
			is_FunctionDeclaration(enclosingNode) &&
			((!is_ABI_Comment(ctx) && comment.loc.isBefore(enclosingNode.generics || enclosingNode.id)) ||
				(enclosingNode.generics && comment.loc.isBetween(enclosingNode.generics, enclosingNode.parameters)))
		) {
			addLeadingComment(enclosingNode, comment);
		} else if (
			!enclosingNode.returnType &&
			comment.loc.isBetween(
				enclosingNode.parameters,
				is_FunctionDeclaration(enclosingNode) ? enclosingNode.body! : enclosingNode.expression
			)
		) {
			if (is_FunctionDeclaration(enclosingNode)) {
				addCommentToBlock(enclosingNode, comment);
			} else {
				addLeadingComment(enclosingNode.expression, comment);
			}
		} else if (
			precedingNode && //
			enclosingNode.parameters.loc.contains(comment)
		) {
			if (precedingNode === getLastParameter(enclosingNode)) {
				addTrailingComment(precedingNode, comment);
			}
		} else if (
			followingNode &&
			isStartOfLine(comment) &&
			comment.loc.isAfter(enclosingNode.parameters) &&
			(!is_FunctionDeclaration(enclosingNode) || !enclosingNode.whereBounds || comment.loc.isAfter(enclosingNode.whereBounds!)) &&
			(!enclosingNode.returnType || comment.loc.isAfter(enclosingNode.returnType)) &&
			followingNode === (is_FunctionDeclaration(enclosingNode) ? enclosingNode.body?.[0] : enclosingNode.expression)
		) {
			addLeadingComment(followingNode, comment);
		}
	}
}
function handleMacroRuleComments(ctx: CommentContext) {
	const { precedingNode, enclosingNode, followingNode, comment } = ctx;
	if (enclosingNode && is_MacroRule(enclosingNode)) {
		if (enclosingNode.transform.loc.contains(comment)) {
			__DEV__: assert(enclosingNode.transform.length > 0);
			if (!precedingNode || !enclosingNode.transform.loc.contains(precedingNode)) {
				__DEV__: assert(!!followingNode && enclosingNode.transform.loc.contains(followingNode));
				addLeadingComment(followingNode, comment);
			}
		} else if (enclosingNode.match.loc.contains(comment)) {
			__DEV__: assert(enclosingNode.match.length > 0);
			if (!followingNode || !enclosingNode.match.loc.contains(followingNode)) {
				__DEV__: assert(!!precedingNode && enclosingNode.match.loc.contains(precedingNode));
				addTrailingComment(precedingNode!, comment);
			}
		}
	}
}

function handleStatementComments(ctx: CommentContext) {
	const { precedingNode, enclosingNode, followingNode, comment } = ctx;
	if (isEndOfLine(comment) && precedingNode && (is_StatementNode(precedingNode) || precedingNode.loc.sliceText().endsWith(";"))) {
		addTrailingComment(precedingNode, comment);
	}
}

function addCommentToBlock(block: NodeWithBodyOrCases, comment: AnyComment) {
	const body = getBodyOrCases(block);
	__DEV__: assert(!!body);
	if (body.length > 0) {
		addLeadingComment(body![0], comment);
	} else {
		addDanglingComment(block, comment, DCM["body"]);
	}
}

function handleIfBlockExpressionComments(ctx: CommentContext) {
	const { comment, precedingNode, enclosingNode, followingNode } = ctx;
	if (enclosingNode && is_IfBlockExpression(enclosingNode)) {
		const { condition, body, else: else_ } = enclosingNode;
		if (comment.loc.isBefore(condition)) {
			addLeadingComment(condition, comment);
		} else if (comment.loc.isBetween(condition, body)) {
			addTrailingComment(condition, comment);
		} else if (else_ && comment.loc.isBetween(body, else_)) {
			if (is_IfBlockExpression(else_)) {
				addLeadingComment(else_.condition, comment);
			} else {
				addCommentToBlock(else_, comment);
			}
		}
	}
}

function handleMemberExpressionComments({ comment, precedingNode, enclosingNode, followingNode }: CommentContext) {
	if (enclosingNode && is_MemberAccessLike(enclosingNode)) {
		if (isStartOfLine(comment) || !precedingNode) addLeadingComment(enclosingNode, comment);
		else addTrailingComment(precedingNode, comment);
		return true;
	}

	return false;
}

function handleDanglingComments({ comment, precedingNode, enclosingNode, followingNode }: CommentContext) {
	if (enclosingNode) {
		for (var key in DCM) {
			if (key in enclosingNode && enclosingNode[key]?.length === 0 && enclosingNode[key].loc.contains(comment)) {
				addDanglingComment(enclosingNode, comment, key as DCM);
				return;
			}
		}
	}
}

function isOwnLine(comment: AnyComment) {
	return isStartOfLine(comment) && hasBreaklineAfter(comment);
}
function isStartOfLine(comment: AnyComment) {
	return comment.placement === "ownLine";
}
function isEndOfLine(comment: AnyComment) {
	return comment.placement === "endOfLine";
}
export function isDangling(comment: AnyComment) {
	__DEV__: assert(handled(comment));
	return !comment.leading && !comment.trailing;
}
export function isLeading(comment: AnyComment) {
	__DEV__: assert(handled(comment));
	return comment.leading && !comment.trailing;
}
export function isTrailing(comment: AnyComment) {
	__DEV__: assert(handled(comment));
	return !comment.leading && comment.trailing;
}

export function print_comment(comment: CommentOrDocComment) {
	__DEV__: Narrow<PrettierCommentInfo>(comment);

	const doc = is_BlockCommentNode(comment)
		? isIndentableBlockComment(comment.value)
			? [
					(!handled(comment) || isTrailing(comment)) && !hasBreaklineBefore(comment) ? hardline : "",
					getCommentStart(comment),
					...comment.value.split(/\n/g).map((line, i, a) =>
						i === 0 //
							? [line.trimEnd(), hardline]
							: !iLast(i, a)
							? [" " + line.trim(), hardline]
							: " " + line.trimStart()
					),
					"*/",
			  ]
			: [
					getCommentStart(comment), //
					join(literalline, comment.value.split(/\n/g)),
					"*/",
			  ]
		: [getCommentStart(comment), comment.value.trimEnd()];

	return handled(comment) && isDangling(comment) //
		? [doc, getPostLeadingComment(comment)]
		: doc;

	function getCommentStart(comment: CommentOrDocComment) {
		return is_Comment(comment)
			? is_BlockCommentKind(comment)
				? "/*"
				: "//"
			: is_BlockCommentKind(comment)
			? isInner(comment)
				? "/*!"
				: "/**"
			: isInner(comment)
			? "//!"
			: "///";
	}
	function isIndentableBlockComment(value: string) {
		const lines = `*${value}*`.split(/\n/g);
		return lines.length > 1 && lines.every((line) => /^\s*\*/.test(line));
	}
}
