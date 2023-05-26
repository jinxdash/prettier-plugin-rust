import { DelimGroup, DelimKind, Identifier, LocArray, PunctuationToken, Segment, TK } from "jinx-rust";
import { isTK, is_DelimGroup, is_Identifier, is_PunctuationToken } from "jinx-rust/utils";

export function isIdent(node: Segment | undefined, name?: string): node is Identifier {
	return !!node && is_Identifier(node) && (null == name || node.name === name);
}
export function isToken(node: Segment | undefined, tk?: TK): node is PunctuationToken {
	return !!node && (null == tk ? is_PunctuationToken(node) : isTK(node, tk));
}
export function isGroup<D extends DelimKind>(node: Segment | undefined, dk?: D): node is DelimGroup & { segments: LocArray<any, D> } {
	return !!node && is_DelimGroup(node) && (null == dk || node.segments.dk === dk);
}

export function isCallLike(tk_1: Segment | undefined, tk_2: Segment | undefined): boolean {
	return !!tk_1 && !!tk_2 && is_Identifier(tk_1) && is_DelimGroup(tk_2) && tk_2.segments.dk === DelimKind["()"];
}
