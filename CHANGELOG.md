# Prettier Rust Changelog

## Unreleased

- fix: clear some issues related to formatting `#[attr]` like full-on expressions ([#25](jinxdash/prettier-plugin-rust/issues/25))

## 0.1.9

- feat: format `cfg_if!` macros
- feat: format `@` character in `macro_rules`
- fix: format `<number>.` and `<number>e0` to float instead of int ([#14](https://github.com/jinxdash/prettier-plugin-rust/issues/14), [#16](https://github.com/jinxdash/prettier-plugin-rust/issues/16))
- fix: always end files with a newline ([#21](https://github.com/jinxdash/prettier-plugin-rust/issues/21))
- fix: avoid removing optional semi in rare [rust compiler bug](https://github.com/rust-lang/rust/issues/70844) ([#22](https://github.com/jinxdash/prettier-plugin-rust/issues/22))

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
