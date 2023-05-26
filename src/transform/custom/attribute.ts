import {
	AttrSegment,
	CallExpression,
	DelimKind,
	ExpressionPath,
	Identifier,
	Literal,
	LocArray,
	MacroInvocation,
	NodeType,
	PunctuationToken,
	ReassignmentExpression,
	TK,
	rs,
} from "jinx-rust";
import { isTK, start } from "jinx-rust/utils";
import { assert, exit } from "../../utils/common";
import { isIdent } from "./utils";

type SimpleAttrItem =
	| Identifier //
	| Literal
	| ExpressionPath
	| CallExpression
	| ReassignmentExpression
	| MacroInvocation;

export function transform_simpleAttrSyntax(segments: MacroInvocation["segments"]) {
	assert(segments.length !== 0, segments.loc.url());
	return transform_segments(segments, false);

	function transform_segments<N extends boolean>(
		seq: LocArray<AttrSegment>,
		nestedCall: N
	): N extends true ? LocArray<SimpleAttrItem, "()"> : SimpleAttrItem {
		let i = 0;

		if (nestedCall) {
			const args = rs.createLocArray<SimpleAttrItem, any>(DelimKind["()"], seq.loc.clone());
			while (i !== seq.length) {
				args.push(read(true));
				if (i === seq.length) break;
				assert(isTK(seq[i++], TK[","]));
			}
			return args as any;
		} else {
			const res = read(true);
			assert(i === seq.length, res.loc.url());
			return res as any;
		}

		function read(allowEq: boolean): SimpleAttrItem {
			let lhs: Identifier | ExpressionPath;

			switch (seq[i].nodeType) {
				case NodeType.Literal:
					return seq[i++] as Literal;
				case NodeType.Identifier:
					lhs = seq[i++] as Identifier;
					break;
				case NodeType.PunctuationToken:
					assert((seq[i] as PunctuationToken).tk === TK["::"], seq[i].loc.url());
					lhs = eatPathSegment(undefined);
					break;
				default:
					exit.never();
			}

			while (true) {
				if (i === seq.length) return lhs;
				const seg = seq[i];
				switch (seg.nodeType) {
					case NodeType.PunctuationToken:
						switch (seg.tk) {
							case TK[","]:
								assert(nestedCall);
								return lhs;
							case TK["="]: {
								assert(allowEq);
								const right = (i++, read(false));
								return rs.mockNode(NodeType.ReassignmentExpression, right.loc.cloneFrom(start(lhs)), {
									tk: TK["="],
									kind: DelimKind["="],
									left: lhs,
									right,
								});
							}
							case TK["::"]:
								lhs = eatPathSegment(lhs);
								continue;
							default:
								exit.never();
						}
					case NodeType.DelimGroup:
						assert(seg.segments.dk === DelimKind["()"]);
						return rs.mockNode(NodeType.CallExpression, seq[i++].loc.cloneFrom(start(lhs)), {
							callee: lhs,
							typeArguments: undefined,
							method: undefined,
							arguments: transform_segments(seg.segments, true),
						});
					default:
						exit.never();
				}
			}
		}

		function eatPathSegment(left: undefined | Identifier | ExpressionPath) {
			const segment = seq[i + 1];
			assert(isIdent(segment));
			const res = rs.mockNode(NodeType.ExpressionPath, segment.loc.cloneFrom(start(left ?? seq[i])), { namespace: left, segment });
			i += 2;
			return res;
		}
	}
}
