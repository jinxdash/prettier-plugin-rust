import { DelimGroup, DelimKind, Identifier, LocArray, PunctuationToken, TK } from "jinx-rust";
import { isTK, is_DelimGroup, is_Identifier } from "jinx-rust/utils";

export function isIdent(node: any, name: string): node is Identifier {
	return !!node && is_Identifier(node) && node.name === name;
}
export function isToken(node: any, tk: TK): node is PunctuationToken {
	return !!node && isTK(node, tk);
}
export function isGroup<D extends DelimKind>(node: any, dk: D): node is DelimGroup & { segments: LocArray<any, D> } {
	return !!node && is_DelimGroup(node) && node.segments.dk === dk;
}
