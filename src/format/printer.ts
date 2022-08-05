import { DelimKind, Node, NodeType, NTMap } from "jinx-rust";
import {
	getDelimChars,
	hasSuffix,
	is_ArrayOrTupleLiteral,
	is_BlockExpression,
	is_ClosureFunctionExpression,
	is_Identifier,
	is_IfBlockExpression,
	is_LiteralNumberLike,
	is_StructLiteral,
	start,
} from "jinx-rust/utils";
import { exit } from "../utils/common";
import { hasComment, print_comment } from "./comments";
import { isSimpleType } from "./complexity";
import {
	adjustClause,
	parenthesize_if_break,
	printAnnotatedPattern,
	printArrayLike,
	printArrowFunction,
	printAssignment,
	printBinaryishExpression,
	printBlockBody,
	printBodyOrCases,
	printCallArguments,
	printCallExpression,
	printCondition,
	printDanglingCommentsForInline,
	printDeclarationTypeBounds,
	printEnumBody,
	printFlowControlExpression,
	printGenerics_x_whereBounds,
	printIfBlock,
	printIfBlockCondition,
	printImplTraitForType,
	printLtBounds,
	printLtParameters,
	printMacroRules,
	printMaybeBlockBody,
	printMemberExpression,
	printNumber,
	printObject,
	printParametersAndReturnType,
	printRuleMatch,
	printRuleTransform,
	printTypeAnnotation,
	printTypeArguments,
	printTypeBounds,
	printUnaryExpression,
	printUnionPattern,
} from "./core";
import { DCM, Doc, group, hardline, ifBreak, indent, join, line, softline, willBreak } from "./external";
import { f, getOptions, getParentNode, pathCall, print, sg_duo, sg_single } from "./plugin";
import { needsParens, stmtNeedsSemi } from "./styling";
import {
	BlockLikeMacroInvocation,
	CallLikeMacroInvocation,
	isTransformed,
	is_BlockLikeMacroInvocation,
	is_CallLikeMacroInvocation,
} from "./transform";

type nPrint<T extends Node> = (print: print<T>, node: T) => Doc | never;

export const printer: { [K in NodeType]: nPrint<Extract<NTMap[K], Node>> } = {
	[NodeType.MissingNode](print, node) {
		return "";
	},
	[NodeType.SourceFile](print, node) {
		return [
			print.b("UTF8BOM", "\uFEFF"), //
			print("shebang"),
			print("program"),
		];
	},
	[NodeType.Shebang](print, node) {
		return [`#!${node.value}`, hardline];
	},
	[NodeType.Program](print, node) {
		return printBodyOrCases(print, node);
	},
	[NodeType.Snippet](print, node) {
		exit.never();
	},
	[NodeType.Identifier](print, node) {
		return node.name;
	},
	[NodeType.Index](print, node) {
		return node.name;
	},
	[NodeType.LbIdentifier](print, node) {
		return node.name;
	},
	[NodeType.McIdentifier](print, node) {
		return node.name;
	},
	[NodeType.LtIdentifier](print, node) {
		return node.name;
	},
	[NodeType.PunctuationToken](print, node) {
		return node.token;
	},
	[NodeType.DelimGroup](print, node) {
		return node.loc.getOwnText();
	},
	[NodeType.Literal](print, node) {
		let { value } = node;
		if (is_LiteralNumberLike(node)) value = printNumber(value);
		return hasSuffix(node) ? [value, print("suffix")] : value;
	},
	[NodeType.ItemPath](print, node) {
		return [print("namespace"), "::", print("segment")];
	},
	[NodeType.ExpressionPath](print, node) {
		return [print("namespace"), "::", print("segment")];
	},
	[NodeType.TypePath](print, node) {
		return [print("namespace"), "::", print("segment")];
	},
	[NodeType.Comment](print, node) {
		return print_comment(node);
	},
	[NodeType.DocCommentAttribute](print, node) {
		return print_comment(node);
	},
	[NodeType.Attribute](print, node) {
		return [
			node.inner ? "#![" : "#[",
			isTransformed(node)
				? [print("segments"), printDanglingCommentsForInline(node)] //
				: node.segments.loc.sliceText(1, -1).trim(),
			"]",
		];
	},
	[NodeType.MacroInvocation](print, node) {
		const hasCurlyBrackets = node.segments.dk === DelimKind["{}"];
		const delim = getDelimChars(node.segments);
		if (node.segments.length === 0) {
			return [print("callee"), "!", hasCurlyBrackets ? " " : "", delim.left, printDanglingCommentsForInline(node), delim.right];
		}
		if (isTransformed(node)) {
			if (is_CallLikeMacroInvocation(node)) {
				return [print("callee"), "!", printCallArguments(print as print<CallLikeMacroInvocation>, node)];
			}
			if (is_BlockLikeMacroInvocation(node)) {
				return [print("callee"), "!", " ", printBlockBody(print as print<BlockLikeMacroInvocation>, node)];
			}
		}
		let content = node.segments.loc.sliceText(1, -1);
		if (content.trim().length === 0) {
			content = "";
		} else if (!content.includes("\n")) {
			content = content.trim();
			if (hasCurlyBrackets) content = " " + content + " ";
			content = content
				.replace(/\s+/g, " ")
				.replace(/ [,\)\];:](?!:)/g, (str) => str.trim())
				.replace(/[\(\[] /g, (str) => str.trim());
		}
		return [print("callee"), "!", hasCurlyBrackets ? " " : "", delim.left, content, delim.right];
	},
	[NodeType.MacroRulesDeclaration](print, node) {
		return ["macro_rules! ", print("id"), printMacroRules(print, node)];
	},
	[NodeType.MacroRuleDeclaration](print, node) {
		return [printRuleMatch(print, node), " => ", printRuleTransform(print, node), ";"];
	},
	[NodeType.MacroDeclaration](print, node) {
		return [print("pub"), "macro ", print("id"), printMacroRules(print, node)];
	},
	[NodeType.MacroInlineRuleDeclaration](print, node) {
		return [printRuleMatch(print, node), " ", printRuleTransform(print, node)];
	},
	[NodeType.MacroGroup](print, node) {
		return node.loc.getOwnText();
	},
	[NodeType.MacroParameterDeclaration](print, node) {
		return [print("id"), ":", print("ty")];
	},
	[NodeType.PubSpecifier](print, node) {
		if (!node.location) return "pub ";
		if (is_Identifier(node.location)) {
			switch (node.location.name) {
				case "crate":
					if (start(node) === start(node.location)) {
						return "crate ";
					} else {
						return ["pub(", print("location"), ") "];
					}
				case "self":
				case "super":
					return ["pub(", print("location"), ") "];
			}
		}
		return ["pub(in ", print("location"), ") "];
	},
	[NodeType.ExternSpecifier](print, node) {
		return ["extern ", f`${print("abi")} `];
	},
	[NodeType.ExpressionStatement](print, node) {
		return [print("expression"), stmtNeedsSemi(node) ? ";" : ""];
	},
	[NodeType.UseStatement](print, node) {
		return [print("pub"), "use ", print("import"), ";"];
	},
	[NodeType.DestructuredImport](print, node) {
		if (node.specifiers.length === 0) return [print("source"), "::{", printDanglingCommentsForInline(node, DCM["specifiers"]), "}"];
		let space = true;
		__DEV__: if (globalThis.TESTS_FORMAT_DEV) space = false;
		return [
			print("source"),
			group([
				"::{",
				indent([space ? line : softline, join([",", line], print("specifiers")), ifBreak(",")]),
				space ? line : softline,
				"}",
			]),
		];
	},
	[NodeType.AmbientImport](print, node) {
		return f`${print("source")}::*` || "*";
	},
	[NodeType.AnonymousImport](print, node) {
		return [print("source"), " as ", "_"];
	},
	[NodeType.NamedImport](print, node) {
		return [print("source"), f` as ${print("local")}`];
	},
	[NodeType.ExternCrateStatement](print, node) {
		return [print("pub"), "extern crate ", print("import"), ";"];
	},
	[NodeType.TypeAliasDeclaration](print, node) {
		return [
			print("pub"),
			"type",
			printAssignment(
				printGenerics_x_whereBounds(print, node, printDeclarationTypeBounds(print, node, ":")), //
				" =",
				"typeExpression"
			),
			";",
		];
	},
	[NodeType.LetVariableDeclaration](print, node) {
		return [
			"let ",
			printAssignment(
				printAnnotatedPattern(print, node), //
				" =",
				"expression"
			),
			f` else ${print("else")}`,
			";",
		];
	},
	[NodeType.ConstVariableDeclaration](print, node) {
		return [
			print("pub"),
			"const ",
			printAssignment(
				printAnnotatedPattern(print, node), //
				" =",
				"expression"
			),
			";",
		];
	},
	[NodeType.StaticVariableDeclaration](print, node) {
		return [
			print("pub"),
			"static ",
			printAssignment(
				printAnnotatedPattern(print, node), //
				" =",
				"expression"
			),
			";",
		];
	},
	[NodeType.ModuleDeclaration](print, node) {
		return [
			print("pub"), //
			print.b("unsafe"),
			"mod ",
			print("id"),
			printMaybeBlockBody(print, node),
		];
	},
	[NodeType.ExternBlockDeclaration](print, node) {
		return [
			print("pub"), //
			print.b("unsafe"),
			"extern ",
			f`${print("abi")} `,
			printBlockBody(print, node),
		];
	},
	[NodeType.FunctionDeclaration](print, node) {
		return [
			print("pub"),
			print.b("const"),
			print.b("async"),
			print.b("unsafe"),
			print("extern"),
			"fn",
			printGenerics_x_whereBounds(print, node, printParametersAndReturnType(node)),
			printMaybeBlockBody(print, node),
		];
	},
	[NodeType.FunctionSelfParameterDeclaration](print, node) {
		return group([print.b("ref", "&"), f`${print("lt")} `, print.b("mut"), "self", printTypeAnnotation(print, node)]);
	},
	[NodeType.FunctionParameterDeclaration](print, node) {
		return group(printAnnotatedPattern(print, node));
	},
	[NodeType.FunctionSpread](print, node) {
		return "...";
	},
	[NodeType.StructDeclaration](print, node) {
		return [print("pub"), "struct", printGenerics_x_whereBounds(print, node, ""), printObject(print, node)];
	},
	[NodeType.StructPropertyDeclaration](print, node) {
		return [print("pub"), print("id"), printTypeAnnotation(print, node)];
	},
	[NodeType.TupleStructDeclaration](print, node) {
		return [print("pub"), "struct", printGenerics_x_whereBounds(print, node, printArrayLike(print, node)), ";"];
	},
	[NodeType.TupleStructItemDeclaration](print, node) {
		return [print("pub"), print("typeAnnotation")];
	},
	[NodeType.UnionDeclaration](print, node) {
		return [print("pub"), "union", printGenerics_x_whereBounds(print, node, ""), printObject(print, node)];
	},
	[NodeType.EnumDeclaration](print, node) {
		return [print("pub"), "enum", printGenerics_x_whereBounds(print, node, ""), printEnumBody(print, node)];
	},
	[NodeType.EnumMemberDeclaration](print, node) {
		return [
			print("pub"),
			printAssignment(
				print("id"), //
				" =",
				"value"
			),
		];
	},
	[NodeType.EnumMemberTupleDeclaration](print, node) {
		return [
			print("pub"),
			printAssignment(
				[print("id"), printArrayLike(print, node)], //
				" =",
				"value"
			),
		];
	},
	[NodeType.EnumMemberStructDeclaration](print, node) {
		return [
			print("pub"),
			printAssignment(
				[print("id"), printObject(print, node)], //
				" =",
				"value"
			),
		];
	},
	[NodeType.TraitDeclaration](print, node) {
		return [
			print("pub"),
			print.b("unsafe"),
			"trait",
			printGenerics_x_whereBounds(print, node, printDeclarationTypeBounds(print, node, ":")),
			adjustClause(node, printBlockBody(print, node)),
		];
	},
	[NodeType.AutoTraitDeclaration](print, node) {
		return [
			print("pub"),
			print.b("unsafe"),
			"auto trait ",
			print("id"),
			" ",
			printBlockBody(print, node as any), // see "transform.ts"
		];
	},
	[NodeType.TraitAliasDeclaration](print, node) {
		return [
			print("pub"),
			print.b("unsafe"),
			"trait",
			printGenerics_x_whereBounds(print, node, printDeclarationTypeBounds(print, node, " =")),
			";",
		];
	},
	[NodeType.ImplDeclaration](print, node) {
		return [
			print("pub"),
			print.b("unsafe"),
			"impl",
			printGenerics_x_whereBounds(print, node, [print.b("const"), printImplTraitForType(print, node)]),
			adjustClause(node, printBlockBody(print, node)),
		];
	},
	[NodeType.NegativeImplDeclaration](print, node) {
		return [
			print("pub"),
			"impl",
			printGenerics_x_whereBounds(print, node, ["!", printImplTraitForType(print, node)]),
			" ",
			printBlockBody(print, node as any), // see "transform.ts"
		];
	},
	[NodeType.ExpressionTypeSelector](print, node) {
		return group(["<", print("typeTarget"), f` as ${print("typeExpression")}`, ">"]);
	},
	[NodeType.ExpressionTypeCast](print, node) {
		return [print("typeCallee"), f`::${printTypeArguments(print, node)}`];
	},
	[NodeType.ExpressionAsTypeCast](print, node) {
		return [print("expression"), " as ", print("typeExpression")];
	},
	[NodeType.ReturnExpression](print, node) {
		return ["return", printFlowControlExpression(print, node)];
	},
	[NodeType.BreakExpression](print, node) {
		return ["break", f` ${print("label")}`, printFlowControlExpression(print, node)];
	},
	[NodeType.ContinueExpression](print, node) {
		return ["continue", f` ${print("label")}`];
	},
	[NodeType.YieldExpression](print, node) {
		return ["yield", printFlowControlExpression(print, node)];
	},
	[NodeType.RangeLiteral](print, node) {
		return [print("lower"), "..", print.b("last", "="), print("upper")];
	},
	[NodeType.CallExpression](print, node) {
		return printCallExpression(print, node);
	},
	[NodeType.MemberExpression](print, node) {
		return printMemberExpression(print, node);
	},
	[NodeType.AwaitExpression](print, node) {
		return [print("expression"), ".await"];
	},
	[NodeType.UnwrapExpression](print, node) {
		return [print("expression"), "?"];
	},
	[NodeType.ParenthesizedExpression](print, node) {
		exit.never();
		const shouldHug = !hasComment(node.expression) && (is_ArrayOrTupleLiteral(node.expression) || is_StructLiteral(node.expression));
		if (shouldHug) return ["(", print("expression"), ")"];
		return group(["(", indent([softline, print("expression")]), softline, ")"]);
	},
	[NodeType.MinusExpression](print, node) {
		return printUnaryExpression("-", node);
	},
	[NodeType.NotExpression](print, node) {
		return printUnaryExpression("!", node);
	},
	[NodeType.OrExpression](print, node) {
		return printBinaryishExpression(print, node);
	},
	[NodeType.AndExpression](print, node) {
		return printBinaryishExpression(print, node);
	},
	[NodeType.ReassignmentExpression](print, node) {
		return printAssignment(print("left"), " =", "right");
	},
	[NodeType.UnassignedExpression](print, node) {
		return "_";
	},
	[NodeType.OperationExpression](print, node) {
		return printBinaryishExpression(print, node);
	},
	[NodeType.ReassignmentOperationExpression](print, node) {
		return printAssignment(print("left"), " " + node.kind, "right");
	},
	[NodeType.ComparisonExpression](print, node) {
		return printBinaryishExpression(print, node);
	},
	[NodeType.LetScrutinee](print, node) {
		return ["let ", printAssignment(print("pattern"), " =", "expression")];
	},
	[NodeType.ClosureFunctionExpression](print, node) {
		return printArrowFunction(print, node);
	},
	[NodeType.ClosureFunctionParameterDeclaration](print, node) {
		return group(printAnnotatedPattern(print, node));
	},
	[NodeType.BlockExpression](print, node) {
		return [
			f`${print("label")}: `,
			print.b("const"),
			print.b("async"),
			print.b("move"),
			print.b("unsafe"),
			printBlockBody(print, node),
		];
	},
	[NodeType.LoopBlockExpression](print, node) {
		return [f`${print("label")}: `, "loop ", printBlockBody(print, node)];
	},
	[NodeType.WhileBlockExpression](print, node) {
		return [f`${print("label")}: `, "while ", printCondition(print, node), printBlockBody(print, node)];
	},
	[NodeType.ForInBlockExpression](print, node) {
		return [f`${print("label")}: `, "for ", print("pattern"), " in ", print("expression"), " ", printBlockBody(print, node)];
	},
	[NodeType.IfBlockExpression](print, node) {
		return [f`${print("label")}: `, printIfBlock(print, node)];
	},
	[NodeType.TryBlockExpression](print, node) {
		return [f`${print("label")}: `, "try ", printBlockBody(print, node)];
	},
	[NodeType.MatchExpression](print, node) {
		const id = Symbol("match");
		const expr = print("expression");
		const needs_parens = pathCall(node, "expression", needsParens);

		let printed: Doc = [
			f`${print("label")}: `,
			"match ",
			needs_parens ? expr : group([indent([softline, expr]), softline], { id }),
			needs_parens ? " " : !willBreak(expr) ? ifBreak("", " ", { groupId: id }) : "" /*  ifBreak("", " ", { groupId: id }) */,
			printBlockBody(print, node),
		];

		const parent = getParentNode()!;
		if (is_ClosureFunctionExpression(parent) && parent.expression === node) {
			printed = parenthesize_if_break([indent([softline, printed]), softline]);
		}
		return printed;
	},
	[NodeType.MatchExpressionCase](print, node) {
		return group([
			group(print("pattern")),
			" ",
			printIfBlockCondition(print, node),
			"=>", //
			(is_BlockExpression(node.expression) || is_IfBlockExpression(node.expression)) &&
			!hasComment(node.expression, 0, (comment) => getOptions().danglingAttributes.includes(comment as any))
				? [" ", print("expression")]
				: group(indent([line, print("expression")])),
		]);
		return printAssignment(
			[print("pattern"), " ", printIfBlockCondition(print, node)], //
			"=>",
			"expression"
		);
		return [print("pattern"), " ", printIfBlockCondition(print, node)];
	},
	[NodeType.StructLiteral](print, node) {
		return [print("struct"), printObject(print, node)];
	},
	[NodeType.StructLiteralPropertyShorthand](print, node) {
		return print("value");
	},
	[NodeType.StructLiteralProperty](print, node) {
		return [print("key"), ": ", print("value")];
	},
	[NodeType.StructLiteralPropertySpread](print, node) {
		return ["..", print("expression")];
	},
	[NodeType.StructLiteralRestUnassigned](print, node) {
		return "..";
	},
	[NodeType.ArrayLiteral](print, node) {
		return printArrayLike(print, node);
	},
	[NodeType.SizedArrayLiteral](print, node) {
		return sg_duo`[${print("initExpression")};${print("sizeExpression")}]`;
	},
	[NodeType.TupleLiteral](print, node) {
		return printArrayLike(print, node);
	},
	[NodeType.ReferenceExpression](print, node) {
		return printUnaryExpression(["&", print.b("mut")], node);
	},
	[NodeType.RawReferenceExpression](print, node) {
		return printUnaryExpression(`&raw ${node.kind} `, node);
	},
	[NodeType.DereferenceExpression](print, node) {
		return printUnaryExpression("*", node);
	},
	[NodeType.BoxExpression](print, node) {
		return printUnaryExpression("box ", node);
	},
	[NodeType.UnionPattern](print, node) {
		return printUnionPattern(print, node);
	},
	[NodeType.ParenthesizedPattern](print, node) {
		exit.never();
		return sg_single`(${print("pattern")})`;
	},
	[NodeType.RestPattern](print, node) {
		return "..";
	},
	[NodeType.WildcardPattern](print, node) {
		return "_";
	},
	[NodeType.PatternVariableDeclaration](print, node) {
		return [print.b("ref"), print.b("mut"), printAssignment(print("id"), " @", "pattern")];
	},
	[NodeType.StructPattern](print, node) {
		return [print("struct"), printObject(print, node)];
	},
	[NodeType.StructPatternPropertyDestructured](print, node) {
		return [print("key"), ": ", print("pattern")];
	},
	[NodeType.StructPatternPropertyShorthand](print, node) {
		return [print.b("box"), print.b("ref"), print.b("mut"), print("id")];
	},
	[NodeType.TuplePattern](print, node) {
		return [print("struct"), printArrayLike(print, node)];
	},
	[NodeType.ArrayPattern](print, node) {
		return printArrayLike(print, node);
	},
	[NodeType.ReferencePattern](print, node) {
		return ["&", print.b("mut"), print("pattern")];
	},
	[NodeType.BoxPattern](print, node) {
		return ["box ", print("pattern")];
	},
	[NodeType.MinusPattern](print, node) {
		return ["-", print("pattern")];
	},
	[NodeType.RangePattern](print, node) {
		return [print("lower"), "..", print.b("last", "="), print("upper")];
	},
	[NodeType.TypeCall](print, node) {
		return [print("typeCallee"), printTypeArguments(print, node)];
	},
	[NodeType.TypeCallNamedArgument](print, node) {
		return printAssignment(print("target"), " =", "typeExpression");
	},
	[NodeType.TypeCallNamedBound](print, node) {
		return [print("typeTarget"), printTypeBounds(":", print, node)];
	},
	[NodeType.LtElided](print, node) {
		return "'_";
	},
	[NodeType.LtStatic](print, node) {
		return "'static";
	},
	[NodeType.TypeNever](print, node) {
		return "!";
	},
	[NodeType.TypeInferred](print, node) {
		return "_";
	},
	[NodeType.GenericTypeParameterDeclaration](print, node) {
		return printAssignment(
			[print("id"), printTypeBounds(":", print, node)], //
			" =",
			"typeDefault"
		);
	},
	[NodeType.ConstTypeParameterDeclaration](print, node) {
		return [
			"const ",
			printAssignment(
				[print("id"), printTypeAnnotation(print, node)], //
				" =",
				"typeDefault"
			),
		];
	},
	[NodeType.GenericLtParameterDeclaration](print, node) {
		return [print("id"), printLtBounds(":", print, node)];
	},
	[NodeType.WhereTypeBoundDeclaration](print, node) {
		return [printLtParameters(print, node), print("typeTarget"), printTypeBounds(":", print, node)];
	},
	[NodeType.WhereLtBoundDeclaration](print, node) {
		return [print("ltTarget"), printLtBounds(":", print, node)];
	},
	[NodeType.TypeTraitBound](print, node) {
		return [print.b("maybeConst", "~const "), print.b("optional", "?"), printLtParameters(print, node), print("typeExpression")];
	},
	[NodeType.TypeDynBounds](print, node) {
		return printTypeBounds("dyn", print, node);
	},
	[NodeType.TypeImplBounds](print, node) {
		return printTypeBounds("impl", print, node);
	},
	[NodeType.TypeFnPointer](print, node) {
		return [printLtParameters(print, node), print.b("unsafe"), print("extern"), "fn", printParametersAndReturnType(node)];
	},
	[NodeType.TypeFnPointerParameter](print, node) {
		return [f`${print("id")}: `, print("typeAnnotation")];
	},
	[NodeType.TypeFunction](print, node) {
		return [print("callee"), printParametersAndReturnType(node)];
	},
	[NodeType.TypeTuple](print, node) {
		return printArrayLike(print, node);
	},
	[NodeType.TypeSizedArray](print, node) {
		return sg_duo`[${print("typeExpression")};${print("sizeExpression")}]`;
		if (isSimpleType(node)) return ["[", print("typeExpression"), "; ", print("sizeExpression"), "]"];
	},
	[NodeType.TypeSlice](print, node) {
		if (isSimpleType(node)) return ["[", print("typeExpression"), "]"];
		return sg_single`[${print("typeExpression")}]`;
	},
	[NodeType.TypeReference](print, node) {
		return ["&", f`${print("lt")} `, print.b("mut"), print("typeExpression")];
	},
	[NodeType.TypeDereferenceConst](print, node) {
		return ["*const ", print("typeExpression")];
	},
	[NodeType.TypeDereferenceMut](print, node) {
		return ["*mut ", print("typeExpression")];
	},
	[NodeType.TypeParenthesized](print, node) {
		exit.never();
		return sg_single`(${print("typeExpression")})`;
	},
};
