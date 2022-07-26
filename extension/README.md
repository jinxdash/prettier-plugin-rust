<div align="center">
  <img alt="Prettier Rust" height="256px" src="https://user-images.githubusercontent.com/109366411/181039409-b66d6a4c-bbc7-4fbb-8a79-d7bb1af87a63.png">
</div>

<h1 align="center">Prettier Rust</h1>

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/prettier-plugin-rust.svg?style=flat)](https://www.npmjs.com/package/prettier-plugin-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/prettier-plugin-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

_The massively popular [Prettier](https://prettier.io/) code formatter, now with [Rust](https://www.rust-lang.org/) support!_


### A _formatter_ -- not a linter?!

Prettier Rust doesn't panic on missing semicolons, _in fact it AUTO-COMPLETES them!_

Thanks to the specially built [`jinx-rust`](https://www.github.com/jinxdash/jinx-rust/) parser, Prettier is completely independent from Rust's strict validation process.  
As such, it is able to format rust just like it formats typescript!

## Configuration

Configure `prettier` by creating a `.prettierrc.toml` file, or [any other file format listed in prettier docs](https://prettier.io/docs/en/configuration.html).

To ignore files, list them in `.prettierignore` (like you would `.gitignore`)

| API Option                   | CLI Option                     | Default | Docs                                                                           |
| ---------------------------- | ------------------------------ | :-----: | ------------------------------------------------------------------------------------- |
| `tabWidth`                   | --tab-width                    |    4    | _[link](https://prettier.io/docs/en/options.html#tab-width)_   |
| `printWidth`                 | --print-width                  |   100   | _[link](https://prettier.io/docs/en/options.html#print-width)_ |
| `endOfLine`                  | --end-of-line                  |  "lf"   | _[link](https://prettier.io/docs/en/options.html#end-of-line)_ |
| `trailingComma`              | --trailing-comma               |         | Not supported yet.                                                                    |
| `embeddedLanguageFormatting` | --embedded-language-formatting |         | Not supported yet.                                                                    |

### Ignoring stuff

To ignore something, add `// prettier-ignore` above it.

`#[rustfmt::skip]` and `#![rustfmt::skip]` are also supported (locally).

##  Report issues at https://www.github.com/jinxdash/prettier-plugin-rust/issues