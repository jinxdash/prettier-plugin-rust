import {
	ForLtParametersBody,
	FunctionSpread,
	GenericParameterDeclaration,
	MaybeGenericArgsTarget,
	MissingNode,
	Node,
	NodeType,
	TypeBound,
	TypeBoundsConstaint,
	TypeCallArgument,
	TypeNamespaceTargetNoSelector,
	TypeNode,
} from "jinx-rust";
import {
	getAstPath,
	getOwnChildAstPath,
	is_BareTypeTraitBound,
	is_FunctionSpread,
	is_LetScrutinee,
	is_Literal,
	is_MissingNode,
	is_TypeBoundsStandaloneNode,
	is_TypeFunctionNode,
	is_TypeNode,
	is_VariableDeclarationNode,
} from "jinx-rust/utils";
import { exit, has_key_defined, last_of, spliceAll } from "../utils/common";
import { canBreak } from "./external";
import { getContext, getNode, getOptions, getPrintFn } from "./plugin";

let DEPTH = 0;
const ANCESTRY: Node[] = [];
const LONE_SHORT_ARGUMENT_THRESHOLD_RATE = 0.25;

export function withCheckContext<R>(fn: () => R): R {
	if (0 === DEPTH) {
		return fn();
	} else {
		DEPTH = 0;
		const prev = spliceAll(ANCESTRY);
		try {
			return fn();
		} finally {
			DEPTH = ANCESTRY.push(...prev);
		}
	}
}

export function is_short(str: string) {
	return str.length <= LONE_SHORT_ARGUMENT_THRESHOLD_RATE * getOptions().printWidth;
}
function print(target: Node) {
	const current = getNode();
	const keys: (string | number)[] = [...getAstPath(ANCESTRY[0], getNode())];
	for (let i = 1; i < ANCESTRY.length; i++) keys.push(...getOwnChildAstPath(ANCESTRY[i - 1], ANCESTRY[i]));
	keys.push(...getOwnChildAstPath(last_of(ANCESTRY), target));
	try {
		return getContext().path.call(() => getPrintFn(target)(), ...keys);
	} catch (e) {
		console.log({ current, target, keys, ANCESTRY });
		throw e;
	}
}

function IsSimpleFunction<T extends Node>(fn: (node: T) => boolean): (node: T) => boolean {
	return function (node: T) {
		if (0 !== DEPTH && node === ANCESTRY[DEPTH - 1]) {
			return fn(node);
		}

		if (DEPTH >= 2) {
			return isShortBasic(node);
		}

		try {
			return fn((ANCESTRY[DEPTH++] = node) as any);
		} finally {
			ANCESTRY.length = --DEPTH;
		}
	} as any;
}

function HasComplexFunction<T extends Node>(fn: (node: T) => boolean): (node: T) => boolean {
	return function (node: T) {
		if (0 !== DEPTH && node === ANCESTRY[DEPTH - 1]) {
			return fn(node);
		}

		if (DEPTH >= 2) {
			return !isShortBasic(node);
		}

		try {
			return fn((ANCESTRY[DEPTH++] = node) as any);
		} finally {
			ANCESTRY.length = --DEPTH;
		}
	} as any;
}

const isShortBasic = (node: Node) => {
	switch (node.nodeType) {
		case NodeType.MissingNode:
			return true;
		case NodeType.Identifier:
		case NodeType.Index:
		case NodeType.LtIdentifier:
		case NodeType.LbIdentifier:
		case NodeType.McIdentifier:
			return is_short(node.name);
		case NodeType.Literal:
			return is_short(node.value) && !/\n/.test(node.value);
	}
	return false;
};

export const isSimpleType = IsSimpleFunction<FunctionSpread | TypeNode | MissingNode>((node): boolean => {
	switch (node.nodeType) {
		case NodeType.MissingNode:
		case NodeType.FunctionSpread:
			return true;
		case NodeType.MacroInvocation:
			return false;
		case NodeType.Identifier:
		case NodeType.TypeNever:
		case NodeType.TypeInferred:
			return true;
		case NodeType.TypePath:
			return isShortBasic(node.segment) && (!node.namespace || isSimpleType(node.namespace));
		case NodeType.TypeCall:
			return isSimpleType(node.typeCallee) && !hasComplexTypeArguments(node);
		case NodeType.ExpressionTypeSelector:
			return isSimpleType(node.typeTarget) && (!node.typeExpression || isSimpleType(node.typeExpression));
		case NodeType.TypeDynBounds:
			return !hasComplexTypeBounds(node);
		case NodeType.TypeImplBounds:
			return !hasComplexTypeBounds(node);
		case NodeType.TypeFnPointer: {
			const param = node.parameters[0];
			return (
				(!node.extern || !node.extern.abi || isShortBasic(node.extern.abi)) &&
				!hasComplexLtParameters(node) &&
				(node.parameters.length === 0 ||
					(node.parameters.length === 1 &&
						(is_FunctionSpread(param) ||
							(!is_TypeFunctionNode(param.typeAnnotation) && isSimpleType(param.typeAnnotation))))) &&
				(!node.returnType || isSimpleType(node.returnType))
			);
		}
		case NodeType.TypeFunction:
			return isSimpleType(node.callee) && node.parameters.every(isSimpleType) && (!node.returnType || isSimpleType(node.returnType));
		case NodeType.TypeSizedArray:
			return isSimpleType(node.typeExpression) && isShortBasic(node.sizeExpression);
		case NodeType.TypeSlice:
			return isSimpleType(node.typeExpression);
		case NodeType.TypeTuple:
			return node.items.length === 0 || (node.items.length === 1 && isSimpleType(node.items[0]));
		case NodeType.TypeReference:
		case NodeType.TypeDereferenceMut:
		case NodeType.TypeDereferenceConst:
		case NodeType.TypeParenthesized:
			return isSimpleType(node.typeExpression);
		default:
			__DEV__: exit.never(node);
			return false;
	}
});

export const hasComplexTypeBounds = HasComplexFunction<Extract<Node, TypeBoundsConstaint>>((node) => {
	return !!node.typeBounds && node.typeBounds.length > 1 && !node.typeBounds.every(isSimpleTypeBound);
});

export const isSimpleTypeBound = (node: TypeBound): boolean => {
	switch (node.nodeType) {
		case NodeType.TypeParenthesized:
			return isSimpleTypeBound(node.typeExpression);
		// #Lifetime
		case NodeType.LtIdentifier:
		case NodeType.LtElided:
		case NodeType.LtStatic:
			return true;
		case NodeType.TypeTraitBound:
			return is_BareTypeTraitBound(node) && isSimpleTypeNamespaceTargetNoSelector(node.typeExpression);
		default:
			__DEV__: exit.never(node);
			return false;
	}
	function isSimpleTypeNamespaceTargetNoSelector(node: TypeNamespaceTargetNoSelector): boolean {
		switch (node.nodeType) {
			case NodeType.Identifier:
				return true;
			case NodeType.TypePath:
				return undefined === node.namespace || isSimpleTypeNamespaceTargetNoSelector(node.namespace);
			case NodeType.TypeCall:
				return false;
			case NodeType.TypeFunction:
				return isSimpleTypeNamespaceTargetNoSelector(node.callee) && node.parameters.length === 0 && !node.returnType;
			default:
				__DEV__: exit.never(node);
				return false;
		}
	}
};

const isSimpleTypeArgument = IsSimpleFunction<TypeCallArgument>((node) => {
	if (is_TypeNode(node)) {
		return isSimpleType(node);
	}
	switch (node.nodeType) {
		// #Lifetime
		case NodeType.LtIdentifier:
		case NodeType.LtElided:
		case NodeType.LtStatic:
		case NodeType.Literal:
			return true;
		case NodeType.MinusExpression:
			return is_Literal(node.expression);
		case NodeType.BlockExpression:
			return false; //willBreak(getPrintFn(node)("body"));
		case NodeType.TypeCallNamedArgument:
			return isSimpleType(node.typeExpression);
		case NodeType.TypeCallNamedBound:
			return isSimpleType(node.typeTarget) && !hasComplexTypeBounds(node);
		default:
			__DEV__: exit.never(node);
			return false;
	}
});

export const hasComplexTypeArguments = HasComplexFunction<Extract<Node, MaybeGenericArgsTarget>>((node) =>
	!node.typeArguments || node.typeArguments.length === 0
		? false
		: node.typeArguments.length === 1
		? (() => {
				const arg = node.typeArguments[0];
				return is_TypeBoundsStandaloneNode(arg) || canBreak(print(arg));
		  })()
		: true
);

export const hasComplexLtParameters = HasComplexFunction<Extract<Node, ForLtParametersBody>>((node) => {
	const ltParameters = node.ltParameters;
	if (!ltParameters || ltParameters.length === 0) {
		return false;
	}
	if (ltParameters.length === 1) {
		const arg = ltParameters[0];
		if (arg.ltBounds && arg.ltBounds.length > 1) {
			return true;
		}

		return false;
	}
	return true;
});

export const isShortGenericParameterDeclaration = IsSimpleFunction<GenericParameterDeclaration>((node) => {
	switch (node.nodeType) {
		case NodeType.GenericTypeParameterDeclaration:
			return !node.typeBounds && !node.typeDefault;
		case NodeType.ConstTypeParameterDeclaration:
			return (!node.typeAnnotation || is_MissingNode(node)) && !node.typeDefault;
		case NodeType.GenericLtParameterDeclaration:
			return !node.ltBounds;
		default:
			exit.never();
	}
});

export const hasComplexGenerics = HasComplexFunction<Node>((node) => {
	return has_key_defined(node, "generics") && node.generics.length > 0 && !node.generics.every(isShortGenericParameterDeclaration);
});

export const hasComplexTypeAnnotation = HasComplexFunction<Node>((node) => {
	if (is_VariableDeclarationNode(node) && !is_LetScrutinee(node)) {
		const { typeAnnotation } = node;
		return !!typeAnnotation && !is_MissingNode(typeAnnotation) && !isSimpleType(typeAnnotation);
	} else {
		return false;
	}
});
