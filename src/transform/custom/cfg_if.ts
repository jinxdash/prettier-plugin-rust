import {
	DelimGroup,
	DelimKind,
	IfBlockExpression,
	LocArray,
	MacroInvocation,
	NodeType,
	NodeWithBody,
	rs,
	Segment,
	Snippet,
	StatementNode,
	TK,
} from "jinx-rust";
import { insertNodes, start, transferAttributes } from "jinx-rust/utils";
import { assert, iLast } from "../../utils/common";
import { isGroup, isIdent, isToken } from "../utils";

export function transform_macro_cfg_if(segments: MacroInvocation["segments"]) {
	const danglingAttributes: Snippet["danglingAttributes"] = [];
	const comments: Snippet["comments"] = [];

	const block = (function create_if_block(i: number): IfBlockExpression | undefined {
		if (i >= segments.length) return undefined;

		const _if = segments[i];
		const pound = segments[i + 1];
		const grp = segments[i + 2];
		const block = segments[i + 3];
		const _else = segments[i + 4];

		assert(
			isIdent(_if, "if") &&
				isToken(pound, TK["#"]) &&
				isGroup(grp, DelimKind["[]"]) &&
				isGroup(block, DelimKind["{}"]) &&
				(!_else || isIdent(_else, "else"))
		);

		return create_block(block, (body) =>
			rs.mockNode(NodeType.IfBlockExpression, block.loc.cloneFrom(start(_if)), {
				label: undefined,
				condition: rs.mockNode(NodeType.Attribute, grp.loc.cloneFrom(start(pound)), {
					segments: grp.segments,
					value: grp.segments.loc.sliceText(),
					line: false,
					inner: false,
				}) as any,
				body: body,
				else: (_else && iLast(i + 5, segments)
					? function create_else_block(i: number) {
							const block = segments[i];
							assert(isGroup(block, DelimKind["{}"]));
							return create_block(block, (body) =>
								rs.mockNode(NodeType.BlockExpression, body.loc.clone(), {
									label: undefined,
									body,
								})
							);
					  }
					: create_if_block)(i + 5),
			})
		);
	})(0);

	const ast = rs.createLocArray(
		segments.dk,
		segments.loc,
		block && [
			rs.mockNode(NodeType.ExpressionStatement, block.loc.clone(), {
				expression: block,
				semi: false,
			}),
		]
	);

	return rs.mockNode(NodeType.Snippet, segments.loc.clone(), { ast, danglingAttributes, comments });

	function create_block<R extends NodeWithBody>(
		group: DelimGroup<Segment> & { segments: { dk: 3 } },
		fn: (statements: LocArray<StatementNode, "{}">) => R
	): R {
		const snippet = rs.toBlockBody(group.segments);

		insertNodes(danglingAttributes, snippet.danglingAttributes);
		insertNodes(comments, snippet.comments);

		const block = fn(snippet.ast);
		transferAttributes(snippet, block);
		return block;
	}
}
