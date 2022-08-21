# Prettier Rust (prettier-plugin-rust) changelog

## Unreleased

## 0.1.7

- (fix) Removes forbidden trailing comma after `StructLiteralPropertySpread` ([#6](https://github.com/jinxdash/prettier-plugin-rust/pull/6))
- (fix) Moves `StructLiteralPropertySpread` to the end of struct literals ([#6](https://github.com/jinxdash/prettier-plugin-rust/pull/6))

## 0.1.6

- (fix) Adds required parenthesis around >1 length standalone type bounds (dyn, impl) nested in unary types. ([#4](https://github.com/jinxdash/prettier-plugin-rust/pull/4))
- (fix) Adds required comma for `MatchExpression.expression` when expression is a block-like Macro invocation. ([#4](https://github.com/jinxdash/prettier-plugin-rust/pull/4))

## 0.1.5

- (docs) Clarified README ([#1](https://github.com/jinxdash/prettier-plugin-rust/issues/1))
- (fix) Fixed ESM imports ([#2](https://github.com/jinxdash/prettier-plugin-rust/issues/2))
- (fix) Fixed block wrapping of closure expressions when returnType is defined
- (fix) Fixed let_else nightly feature formatting

### Extension

- Disabled config caching
