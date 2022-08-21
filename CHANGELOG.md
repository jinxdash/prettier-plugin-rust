# Prettier Rust Changelog

## Unreleased

## 0.1.8

- feat(extension): display message when formatting fails due to non-parser errors.
- feat: move `..spread`, `..` to the end of patterns and reassignments ([#7](https://github.com/jinxdash/prettier-plugin-rust/pull/7))
- fix: remove comma after `..` in patterns and reassignments ([#7](https://github.com/jinxdash/prettier-plugin-rust/pull/7))
- fix: support malformed `macro_rules!` ([jinx-rust@0.1.6](/jinxdash/jinx-rust/pull/2))
- fix: unprinted comment errors in failed macros ([#8](https://github.com/jinxdash/prettier-plugin-rust/pull/8))

## 0.1.7

- feat: move `..spread` to the end of struct literals ([#6](https://github.com/jinxdash/prettier-plugin-rust/pull/6))
- fix: remove comma after `..spread` in struct literals ([#6](https://github.com/jinxdash/prettier-plugin-rust/pull/6))

## 0.1.6

- fix: parenthesize >1 length `dyn`/`impl` types nested in unary types. ([#4](https://github.com/jinxdash/prettier-plugin-rust/pull/4))
- fix: remove comma after `match` cases with block-like macro expressions. ([#4](https://github.com/jinxdash/prettier-plugin-rust/pull/4))

## 0.1.5

- feat: wrap non-block closure expressions with a block when a `->` ReturnType is defined
- fix: add extension to filepath in ESM imports ([#2](https://github.com/jinxdash/prettier-plugin-rust/issues/2))
- fix: add missing whitespace in `let_else` feature
- fix(extension): disable config caching
