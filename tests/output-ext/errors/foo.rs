> Error.toString()
> 
----------------------------------------------------------------------------------------------------
  -1   
       
  1  | ~
       ^ Unexpected End Of File, expected Expression
  2    
  3    
----------------------------------------------------------------------------------------------------
ParserError at ext/jinx-rust/tests/samples/errors/foo.rs:1:1
> inspect(Error)
> Unexpected End Of File, expected Expression
    at read_expression_lhs                   ext/jinx-rust/src/parser/read/expressions.ts:851:11
    at fn                                    ext/jinx-rust/src/parser/read/expressions.ts:875:13
    at ES_withPrecedence                          ext/jinx-rust/src/parser/state/index.ts:784:14
    at ES_withContext                             ext/jinx-rust/src/parser/state/index.ts:777:14
    at read_stmt_expression                   ext/jinx-rust/src/parser/read/expressions.ts:874:9
    at ExpressionStatement.read                ext/jinx-rust/src/parser/read/statements.ts:78:22
    at READ_NODE                            ext/jinx-rust/src/parser/state/constructor.ts:259:46
    at ExpressionStatement.call              ext/jinx-rust/src/parser/state/constructor.ts:245:4
    at read_expr_or_macroInvocation_stmt       ext/jinx-rust/src/parser/read/statements.ts:85:29
    at read_statement                         ext/jinx-rust/src/parser/read/statements.ts:814:11
    at EACH                                   ext/jinx-rust/src/parser/read/statements.ts:821:42
    at fn                                         ext/jinx-rust/src/parser/state/index.ts:500:66
    at with_outerAttributes                        ext/jinx-rust/src/parser/state/index.ts:957:9
    at with_outerAttributes_fromStatementContext   ext/jinx-rust/src/parser/state/index.ts:975:9
    at read_group_noDelim                         ext/jinx-rust/src/parser/state/index.ts:500:18
                                                                             ...filtered 5 lines {
  [stack]: 'Unexpected End Of File, expected Expression\n' +
    '    at \x1B[36mread_expression_lhs\x1B[39m                   \x1B[90mext/jinx-rust/src/parser/read/expressions.ts:851:11\x1B[39m\n' +
    '    at \x1B[36mfn\x1B[39m                                    \x1B[90mext/jinx-rust/src/parser/read/expressions.ts:875:13\x1B[39m\n' +
    '    at \x1B[36mES_withPrecedence\x1B[39m                          \x1B[90mext/jinx-rust/src/parser/state/index.ts:784:14\x1B[39m\n' +
    '    at \x1B[36mES_withContext\x1B[39m                             \x1B[90mext/jinx-rust/src/parser/state/index.ts:777:14\x1B[39m\n' +
    '    at \x1B[36mread_stmt_expression\x1B[39m                   \x1B[90mext/jinx-rust/src/parser/read/expressions.ts:874:9\x1B[39m\n' +
    '    at \x1B[34mExpressionStatement.read\x1B[39m                \x1B[90mext/jinx-rust/src/parser/read/statements.ts:78:22\x1B[39m\n' +
    '    at \x1B[36mREAD_NODE\x1B[39m                            \x1B[90mext/jinx-rust/src/parser/state/constructor.ts:259:46\x1B[39m\n' +
    '    at \x1B[36mExpressionStatement.call\x1B[39m              \x1B[90mext/jinx-rust/src/parser/state/constructor.ts:245:4\x1B[39m\n' +
    '    at \x1B[36mread_expr_or_macroInvocation_stmt\x1B[39m       \x1B[90mext/jinx-rust/src/parser/read/statements.ts:85:29\x1B[39m\n' +
    '    at \x1B[36mread_statement\x1B[39m                         \x1B[90mext/jinx-rust/src/parser/read/statements.ts:814:11\x1B[39m\n' +
    '    at \x1B[36mEACH\x1B[39m                                   \x1B[90mext/jinx-rust/src/parser/read/statements.ts:821:42\x1B[39m\n' +
    '    at \x1B[36mfn\x1B[39m                                         \x1B[90mext/jinx-rust/src/parser/state/index.ts:500:66\x1B[39m\n' +
    '    at \x1B[36mwith_outerAttributes\x1B[39m                        \x1B[90mext/jinx-rust/src/parser/state/index.ts:957:9\x1B[39m\n' +
    '    at \x1B[36mwith_outerAttributes_fromStatementContext\x1B[39m   \x1B[90mext/jinx-rust/src/parser/state/index.ts:975:9\x1B[39m\n' +
    '    at \x1B[36mread_group_noDelim\x1B[39m                         \x1B[90mext/jinx-rust/src/parser/state/index.ts:500:18\x1B[39m\n' +
    '\x1B[90m                                                                             ...filtered 5 lines\x1B[39m',
  [message]: 'Unexpected End Of File, expected Expression',
  loc: { url: 'ext/jinx-rust/tests/samples/errors/foo.rs:1:1', start: { line: 1, column: 1 } },
  ctx: [ [length]: 0 ],
  toString: <ref *1> [Function (anonymous)] { [length]: 0, [name]: '', [prototype]: { [constructor]: [Circular *1] } },
  parserState: {
    nodes: [ 'SourceFile (--1)', 'Program (...parsing)', 'ExpressionStatement (...parsing)', [length]: 3 ],
    discarded_nodes: [ [length]: 0 ],
    __ctx_Precedence: [ 0, 3, [length]: 2 ],
    max_Precedence_depth: 20,
    max_ES_ctx_depth: 12,
    max_TY_depth: 12,
    __es_optional_start: -1,
    __ctx_ES_i: 1,
    __ctx_ES_PRCD_i: 1,
    __ctx_TY_i: 0,
    __ctx_MC_i: 0
  }
}