import {
	ClosureFunctionExpression,
	ComparisonExpression,
	ConditionExpression,
	EnumDeclaration,
	EnumMemberStructDeclaration,
	ExpressionAsTypeCast,
	ExpressionNode,
	ExpressionStatement,
	ExpressionWithBody,
	LeftRightExpression,
	LogicalExpression,
	MacroDeclaration,
	MacroRulesDeclaration,
	MissingNode,
	Node,
	NodeType,
	NodeWithBody,
	NodeWithBodyOrCases,
	OperationExpression,
	PRCD,
	StructDeclaration,
	StructLiteral,
	StructPattern,
	TK,
	UnionDeclaration,
} from "jinx-rust";
import {
	can_have_OuterAttributes,
	getAstPath,
	getPrecedence,
	hasAttributes,
	hasBody,
	hasCondition,
	hasItems,
	hasLetScrutineeCondition,
	hasOuterAttributes,
	hasProperties,
	is_Attribute,
	is_AttributeOrDocComment,
	is_AwaitExpression,
	is_BitwiseOperator,
	is_CallExpression,
	is_ClosureFunctionExpression,
	is_ComparisonExpression,
	is_DocCommentAttribute,
	is_ElseBlock,
	is_EnumMemberDeclaration,
	is_EqualityOperator,
	is_ExpressionAsTypeCast,
	is_ExpressionStatement,
	is_ExpressionWithBody,
	is_ExpressionWithBodyOrCases,
	is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation,
	is_FlowControlExpression,
	is_FlowControlMaybeValueExpression,
	is_ForInBlockExpression,
	is_Identifier,
	is_IfBlockExpression,
	is_ImplicitReturnAbleNode,
	is_LeftRightExpression,
	is_LetScrutinee,
	is_Literal,
	is_LiteralNumberLike,
	is_LogicalExpression,
	is_LoopBlockExpression,
	is_MatchExpression,
	is_MatchExpressionCase,
	is_MemberExpression,
	is_NodeWithBodyNoBody,
	is_NodeWithMaybePatternNoUnionBody,
	is_OperationExpression,
	is_ParenthesizedNode,
	is_PatternVariableDeclaration,
	is_PostfixExpression,
	is_RangeLiteral,
	is_ReassignmentNode,
	is_ReturnExpression,
	is_StatementNode,
	is_StructLiteral,
	is_StructLiteralProperty,
	is_StructPatternProperty,
	is_StructPropertyDeclaration,
	is_TypeBoundsStandaloneNode,
	is_TypeFunctionNode,
	is_TypeTraitBound,
	is_UnaryExpression,
	is_UnaryType,
	is_UnionPattern,
	is_WhileBlockExpression,
	is_YieldExpression,
	is_bitshiftOperator,
	is_multiplicativeOperator,
} from "jinx-rust/utils";
import { BlockLikeMacroInvocation, is_BlockLikeMacroInvocation, is_CallExpression_or_CallLikeMacroInvocation } from "../transform";
import { exit, last_of } from "../utils/common";
import { CF, hasBreaklineAfter, hasComment } from "./comments";
import { flowControlExpressionNeedsOuterParens } from "./core";
import { Doc, hardline, softline, willBreak } from "./external";
import {
	assertPathAtNode,
	getContext,
	getGrandParentNode,
	getNode,
	getOptions,
	getParentNode,
	getPrintFn,
	is_printing_macro,
	pathCallAtParent,
	pathCallParentOf,
	stackIncludes,
} from "./plugin";

export function needsOuterSoftbreakParens(node: Node) {
	const parent = getParentNode(node);

	if (!parent) return false;

	if (is_ExpressionAsTypeCast(node)) {
		return precedenceNeedsParens(node, parent);
	}

	if (
		is_FlowControlMaybeValueExpression(parent) && //
		parent.expression === node &&
		flowControlExpressionNeedsOuterParens(parent)
	) {
		return true;
	}
	if (
		is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node) &&
		(false ||
			(is_MemberExpression(parent) && parent.expression === node) ||
			(is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(parent) && !is_ElseBlock(node, parent)))
	) {
		return true;
	}

	if (is_UnionPattern(node) && is_NodeWithMaybePatternNoUnionBody(parent)) {
		return true;
	}

	if (hasComment(node)) {
		if (is_UnaryExpression(parent)) {
			return true;
		}

		if (hasComment(node, CF.Line)) {
			if (is_ReturnExpression(parent) || (is_YieldExpression(parent) && parent.expression === node)) {
				return true;
			}
		}

		if (
			hasComment(node, CF.Leading, (comment) => is_Attribute(comment) && !comment.inner) &&
			!can_have_OuterAttributes(node, parent, true)
		) {
			return true;
		}
	}

	return false;
}

export function needsInnerParens(node: Node) {
	if (needsOuterSoftbreakParens(node)) {
		return false;
	}

	const parent = getParentNode(node);

	if (!parent) {
		return false;
	}

	if (is_Identifier(node)) {
		return false;
	}

	if (is_Literal(node)) {
		return is_LiteralNumberLike(node) && is_MemberExpression(parent) && node === parent.expression;
	}

	if (is_CallExpression(parent) && parent.callee === node && is_MemberExpression(node)) {
		return !getOptions().actuallyMethodNodes.has(node);
	}

	if (is_ReassignmentNode(node)) {
		if (is_printing_macro()) {
			return false;
		}

		if (is_ClosureFunctionExpression(parent) && node === parent.expression) {
			return true;
		}

		if (is_ExpressionStatement(parent)) {
			return is_StructLiteral(node.left);
		}

		if (is_ReassignmentNode(parent)) {
			return false;
		}

		return true;
	}

	if (is_ParenthesizedNode(parent)) {
		return false;
	}

	if (is_ExpressionStatement(parent)) {
		return false;
	}

	if (is_RangeLiteral(node)) {
		return (
			is_ExpressionAsTypeCast(parent) ||
			is_LogicalExpression(parent) ||
			is_UnaryExpression(parent) ||
			is_PostfixExpression(parent) ||
			(is_MemberExpression(parent) && node === parent.expression) ||
			(is_CallExpression(parent) && node === parent.callee) ||
			is_OperationExpression(parent) ||
			is_ComparisonExpression(parent)
		);
	}

	if (is_LetScrutinee(parent) && is_LogicalExpression(node) && parent.expression === (node as any)) {
		return true;
	}

	if (is_UnaryExpression(node)) {
		switch (parent.nodeType) {
			case NodeType.MemberExpression:
			case NodeType.AwaitExpression:
				return node === parent.expression;
			case NodeType.CallExpression:
				return node === parent.callee;
			default:
				return false;
		}
	}

	if (is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node)) {
		if (is_ExpressionWithBodyOrCases(parent)) {
			return !is_ElseBlock(node, parent);
		}
		if (is_LetScrutinee(parent) && parent.expression === node && is_ExpressionWithBodyOrCases(getGrandParentNode())) {
			return true;
		}
		return (
			is_ExpressionAsTypeCast(parent) ||
			is_LogicalExpression(parent) ||
			is_UnaryExpression(parent) ||
			is_PostfixExpression(parent) ||
			(is_MemberExpression(parent) && node === parent.expression) ||
			(is_CallExpression(parent) && node === parent.callee) ||
			is_OperationExpression(parent) ||
			is_ComparisonExpression(parent) ||
			is_RangeLiteral(parent)
		);
	}

	if (is_StructLiteral(node)) {
		if (is_ExpressionWithBodyOrCases(parent)) {
			return true;
		}

		if (is_LetScrutinee(parent) && parent.expression === node && is_ExpressionWithBodyOrCases(getGrandParentNode())) {
			return true;
		}
		if (is_UnaryExpression(parent) || is_PostfixExpression(parent) || is_MemberExpression(parent)) {
			return parent.expression === node;
		}
		if (is_CallExpression(parent)) {
			return parent.callee === node;
		}
	}

	if (is_LogicalExpression(node) || is_OperationExpression(node) || is_ComparisonExpression(node) || is_ClosureFunctionExpression(node)) {
		return precedenceNeedsParens(node, parent);
	}

	if (is_TypeFunctionNode(node)) {
		const gp = getGrandParentNode();
		if (node.returnType && is_TypeTraitBound(parent) && is_TypeBoundsStandaloneNode(gp) && last_of(gp.typeBounds) !== parent) {
			return true;
		}
	}

	if (is_TypeBoundsStandaloneNode(node)) {
		return (
			(is_UnaryType(parent) && node.typeBounds.length > 1) ||
			is_TypeBoundsStandaloneNode(parent) ||
			is_TypeTraitBound(parent) ||
			(is_TypeFunctionNode(parent) && parent.returnType === node)
		);
	}

	if (is_PatternVariableDeclaration(parent)) {
		return is_UnionPattern(node);
	}

	return false;
}

function precedenceNeedsParens(node: LeftRightExpression | ClosureFunctionExpression | ExpressionAsTypeCast, parent: Node) {
	if (is_UnaryExpression(parent) || is_PostfixExpression(parent)) return true;
	if (is_ReassignmentNode(parent)) return parent.left === node;
	if (is_MemberExpression(parent)) return parent.expression === node;
	if (is_CallExpression(parent)) return parent.callee === node;
	if (is_ExpressionAsTypeCast(parent)) return !is_ExpressionAsTypeCast(node);
	if (is_LogicalExpression(parent)) return is_LogicalExpression(node) ? parent.nodeType !== node.nodeType : evalPrecedence(node, parent);
	if (is_OperationExpression(parent) || is_ComparisonExpression(parent)) return evalPrecedence(node, parent);
	return false;
	function evalPrecedence(
		child: LeftRightExpression | ClosureFunctionExpression | ExpressionAsTypeCast,
		parent: ComparisonExpression | OperationExpression | LogicalExpression
	) {
		if (is_ExpressionAsTypeCast(child) || is_ClosureFunctionExpression(child)) {
			return true;
		}
		function getPrec(node, bool) {
			// if (is_EqualityOperator(node.tk)) {
			// 	return 11.3;
			// }
			// if (is_LargerLesserOperator(node.tk)) {
			// 	return 11.6;
			// }
			return getPrecedence(node, bool);
		}

		const childPRCD = getPrec(child, is_insideScrutinee(child));
		const parentPRCD = getPrec(parent, is_insideScrutinee(parent));

		if (parentPRCD > childPRCD) {
			return true;
		}

		if (parentPRCD === childPRCD && parent.right === child) {
			return true;
		}

		if (parentPRCD === childPRCD && !shouldFlatten(parent, child)) {
			return true;
		}

		if (parentPRCD < childPRCD && child.tk === TK["%"]) {
			return parentPRCD === PRCD["+-"];
		}

		if (is_BitwiseOperator(parent.tk) || (is_BitwiseOperator(child.tk) && is_EqualityOperator(parent.tk))) {
			return true;
		}

		return false;
	}
}

export function shouldFlatten(parent: ExpressionNode | ConditionExpression, node: ExpressionNode | ConditionExpression) {
	if (getPrecedence(node, is_insideScrutinee(node)) !== getPrecedence(parent, is_insideScrutinee(parent))) return false;
	if (is_ComparisonExpression(parent) && is_ComparisonExpression(node)) return false;
	if (is_OperationExpression(parent) && is_OperationExpression(node)) {
		if (
			(node.tk === TK["%"] && is_multiplicativeOperator(parent.tk)) ||
			(parent.tk === TK["%"] && is_multiplicativeOperator(node.tk)) ||
			(node.tk !== parent.tk && is_multiplicativeOperator(node.tk) && is_multiplicativeOperator(parent.tk)) ||
			(is_bitshiftOperator(node.tk) && is_bitshiftOperator(parent.tk))
		)
			return false;
	}
	return true;
}

export function needsParens(node: Node) {
	return needsOuterSoftbreakParens(node) || needsInnerParens(node);
}

export function stmtNeedsSemi(stmt: ExpressionStatement, disregardExprType = false) {
	return pathCallParentOf(stmt, (parent) => needsSemi(parent as any, stmt, disregardExprType));
}

const NoNode = { nodeType: 0 } as MissingNode;

export function needsSemi(parent: NodeWithBody, stmt: ExpressionStatement, disregardExprType = false) {
	const expr = disregardExprType ? NoNode : stmt.expression!;
	const hadSemi = !disregardExprType && stmt.semi;

	return (
		!!expr &&
		(forcePreserveSemi()
			? true
			: shouldNeverSemi()
			? false
			: shouldPreserveSemi()
			? hadSemi || shouldAlwaysSemi() || canAutoCompleteSemi()
			: true)
	);

	function forcePreserveSemi() {
		/** Rust Compiler bug (preserve optional semicolon) */
		// rust-lang/rust#70844 https://github.com/rust-lang/rust/issues/70844
		// issue#22 https://github.com/jinxdash/prettier-plugin-rust/issues/22
		return (
			hadSemi &&
			stmt === last_of(parent.body!) &&
			((is_IfBlockExpression(expr) &&
				hasLetScrutineeCondition(expr) &&
				!(is_LetScrutinee(expr.condition) && is_Identifier(expr.condition.expression))) ||
				(is_MatchExpression(expr) && !is_Identifier(expr.expression)))
		);
	}
	function shouldNeverSemi() {
		return is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(expr);
	}
	function shouldPreserveSemi() {
		return stmt === last_of(parent.body!) && (is_ImplicitReturnAbleNode(parent) || is_BlockLikeMacroInvocation(parent));
	}
	function shouldAlwaysSemi() {
		return is_FlowControlExpression(expr) || is_ReassignmentNode(expr);
	}
	function canAutoCompleteSemi() {
		return withPathAt(parent, function checkParent(child: NodeWithBodyOrCases): boolean {
			return pathCallParentOf(child, (parent) => {
				if (is_IfBlockExpression(parent) && parent.else === child) {
					// if ... { ... } else if { ... } ...
					// ^ ------------------------------- parent
					//                     ^ ----------- child
					return checkParent(parent);
				}
				if (is_ExpressionStatement(parent)) {
					// { .... { ... } ... }
					// ^ -----------------^ grandparent
					//        ^ --- ^       ExpressionStatement<child>
					if (hasOuterAttributes(parent)) return false;
					return stmtNeedsSemi(parent, true);
				}
				if (is_MatchExpressionCase(parent) && parent.expression === child) {
					return pathCallParentOf(parent, checkParent);
				}
				return false;
			});
		});
	}
}

export function canInlineBlockBody(node: NodeWithBodyOrCases | BlockLikeMacroInvocation): boolean {
	if (!is_ExpressionWithBody(node)) {
		return false;
	}
	const body = node.body;

	if (body.length === 0) {
		return canInlineInlineable(node);
	}

	if (body.length === 1) {
		const stmt = body[0];
		if (is_AttributeOrDocComment(stmt)) {
			return true;
		}
		if (is_ExpressionStatement(stmt) && !needsSemi(node, stmt)) {
			/**
			 * parent ( ExpressionStatement | StructLiteralProperty | LetVariableDeclaration | ... )
			 *   ...
			 *   node {
			 *     expr
			 *   }
			 *   ...
			 *
			 *
			 * Q: Can you inline "node { expr }" ?
			 */
			const expr = stmt.expression!;

			if (
				is_FlowControlExpression(expr) || //
				is_ClosureFunctionExpression(expr) ||
				is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(expr)
			) {
				return false;
			}

			return canInlineInlineable(node);
		}
	}
	return false;
}

// function q(node: ExpressionWithBody) {
// 	pathCallTopMostIfBlockExpression(node, (node) => {});
// }

function canInlineInlineable(node: ExpressionWithBody) {
	if (is_ForInBlockExpression(node) || is_LoopBlockExpression(node)) {
		return false;
	}
	if (is_WhileBlockExpression(node)) {
		return true;
	}

	const parent = getParentNode(node)!;

	if (
		is_ExpressionStatement(parent) &&
		(!is_ImplicitReturnAbleNode(node) || pathCallAtParent(parent, (parent) => stmtNeedsSemi(parent, true)))
	) {
		return false;
	}

	if (is_ElseBlock(node, parent)) {
		return pathCallAtParent(parent, canInlineBlockBody);
	}
	// if (is_CaseBlock(node, parent)) {
	// 	return false;
	// }
	if (is_IfBlockExpression(node)) {
		if (
			!node.else ||
			// hasLetScrutineeCondition(node) ||
			is_ExpressionWithBodyOrCases_or_BlockLikeMacroInvocation(node.condition) ||
			willBreak(getPrintFn(node)("condition"))
		) {
			return false;
		}

		const grandparent = getGrandParentNode();
		if (is_ExpressionStatement(parent) && hasBody(grandparent) && grandparent.body.length > 1) {
			return false;
		}
	}
	return true;
	return (
		is_CallExpression_or_CallLikeMacroInvocation(parent) ||
		hasItems(parent) ||
		hasProperties(parent) ||
		is_ClosureFunctionExpression(parent) ||
		is_MemberExpression(parent) ||
		is_AwaitExpression(parent) ||
		is_LeftRightExpression(parent)
	);
}

type NodeWithBracketContent =
	| NodeWithBodyOrCases
	| BlockLikeMacroInvocation
	| EnumDeclaration
	| StructDeclaration
	| StructLiteral
	| StructPattern
	| EnumMemberStructDeclaration
	| UnionDeclaration
	| MacroRulesDeclaration
	| MacroDeclaration;

export function emptyContent(node: NodeWithBracketContent): Doc {
	switch (node.nodeType) {
		case NodeType.Program:
		case NodeType.MacroRulesDeclaration:
		case NodeType.MacroDeclaration:
		case NodeType.ExternBlockDeclaration:
		case NodeType.ModuleDeclaration:
		case NodeType.TraitDeclaration:
		case NodeType.StructDeclaration:
		case NodeType.MacroInvocation:
		case NodeType.FunctionDeclaration:
		case NodeType.ImplDeclaration:
		case NodeType.UnionDeclaration:
		case NodeType.EnumDeclaration:
		case NodeType.EnumMemberStructDeclaration:
		case NodeType.StructLiteral:
		case NodeType.StructPattern:
			// case NodeType.MatchExpression:
			return "";
		case NodeType.BlockExpression:
		case NodeType.WhileBlockExpression:
		case NodeType.ForInBlockExpression:
		case NodeType.TryBlockExpression:
		case NodeType.IfBlockExpression:
			return canInlineInlineable(node)
				? is_IfBlockExpression(node) || is_ElseBlock(node, getParentNode()!)
					? softline
					: ""
				: hardline;
		case NodeType.LoopBlockExpression:
		case NodeType.MatchExpression:
			return hardline;
		default:
			if (is_NodeWithBodyNoBody(node)) {
				return "";
			}
			__DEV__: exit.never(node);
			return "";
	}
}

export function is_insideScrutinee(target: Node) {
	return withPathAt(target, (n) => stackIncludes("condition") && r(n));
	function r(CHILD: Node) {
		switch (CHILD.nodeType) {
			case NodeType.OrExpression:
			case NodeType.AndExpression:
				return pathCallParentOf(CHILD, (PARENT) =>
					hasCondition(PARENT) && PARENT.condition === CHILD //
						? hasLetScrutineeCondition(PARENT)
						: r(PARENT)
				);
			case NodeType.LetScrutinee:
				return true;
			default:
				return false;
		}
	}
}

function withPathAt<T extends Node, R>(target: T, callback: (target: T) => R): R {
	if (target === getNode()) return callback(target);
	if (target === getParentNode()) return pathCallAtParent(target, () => callback(target));
	if (stackIncludes(target)) return pathCallAtParent(getParentNode()!, () => withPathAt(target, callback));
	return getContext().path.call(() => {
		__DEV__: assertPathAtNode("withPathAt", target);
		return callback(target); // @ts-ignore
	}, ...getAstPath(getNode(), target));
}
export function shouldPrintOuterAttributesAbove(node: Node) {
	return (
		is_StatementNode(node) ||
		is_MatchExpressionCase(node) ||
		(hasAttributes(node) &&
			node.attributes.some(
				canInlineOuterAttribute(node)
					? (attr) => is_DocCommentAttribute(attr) || hasBreaklineAfter(attr) //
					: is_DocCommentAttribute
			))
	);
	function canInlineOuterAttribute(node: Node) {
		return (
			is_EnumMemberDeclaration(node) ||
			is_StructPropertyDeclaration(node) ||
			is_StructLiteralProperty(node) ||
			is_StructPatternProperty(node)
		);
	}
}
