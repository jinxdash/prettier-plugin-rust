<div align="center">
  <img alt="Prettier Rust" height="256px" src="https://user-images.githubusercontent.com/109366411/181039409-b66d6a4c-bbc7-4fbb-8a79-d7bb1af87a63.png">
</div>

<h1 align="center">Prettier Rust</h1>

<div align="center">

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/prettier-plugin-rust.svg?style=flat)](https://www.npmjs.com/package/prettier-plugin-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/prettier-plugin-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

_The massively popular [Prettier](https://prettier.io/) code formatter, now with [Rust](https://www.rust-lang.org/) support!_

**Get Started:** Install [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) `Prettier - Code formatter (Rust)`

</div>

## Why Prettier?

> What usually happens once people start using Prettier is that they realize how much time and mental energy they actually spend formatting their code. With Prettier editor integration, you can just press the `Format Document` key binding and poof, the code is formatted. This is an eye-opening experience if anything.

- **Beautiful, uniform and consistent** — Prettier is strongly opinionated, it has zero style options.
- **Life-changing Editor Integration** — Prettier can format WIP code before it can compile _(e.g. missing annotations)_
- **Stop wasting time on formalities** — Prettier autocorrects bad syntax _(e.g. missing semicolons, blocks, parentheses...)_

<table align="center">
<tr> <th>> input</th> <th>> formatted</th> </tr>
<tr>
  <td>

<!-- prettier-ignore -->
```rs
const LEET = 1337
/// My WIP code draft
#![feature(crate_visibility_modifier)]
async crate fn foo(arg) {
  arg.0 *= 3.14 + LEET & 1337
  arg.1(|b, c| -> T &c).await
}
```

  </td>
  <td>

<!-- prettier-ignore -->
```rs
const LEET = 1337;
#![feature(crate_visibility_modifier)]
/// My WIP code draft
crate async fn foo(arg) {
    arg.0 *= (3.14 + LEET) & 1337;
    (arg.1)(|b, c| -> T { &c }).await
}
```

  </td>
</tr>
</table>
<div align="center">

_Formatting succeeds and fixes 7 syntax errors._

</div>

## Configuration

https://prettier.io/docs/en/configuration

```json5
// .prettierrc.json
{
  "useTabs": false,
  "tabWidth": 4,
  "printWidth": 100,
  "endOfLine": "lf",

  // -- Not supported yet --
  // "trailingComma": "es5",
  // "embeddedLanguageFormatting": "auto",

  // Example override
  "overrides": { "files": ["tests/*.rs"], "options": { "printWidth": 80 } }
}
```

<details>
    <summary>Alternative config using TOML</summary>

```toml
# .prettierrc.toml

useTabs = false
tabWidth = 4
printWidth = 100
endOfLine = "lf"

# -- Not supported yet --
# trailingComma = "es5"
# embeddedLanguageFormatting = "auto"

# Example override
overrides = [
  { files = ["tests/*.rs"], options = { printWidth = 80 } }
]
```

</details>

### How to ignore things

- Add `// prettier-ignore` or `#[rustfmt::skip]` above it
- Add `#![rustfmt::skip]` inside blocks or files
- Create a `.prettierignore` file to glob-match files, like `.gitignore`

### How are macros formatted?

- Curlies `!{}` format like blocks, `![]` and `!()` like comma-separated expressions
- Formatting inside macro invocations is more conservative, since macros can be token-sensitive
- Popular/built-in macros with original syntax rules get custom formatting (e.g. `matches!`, `if_chains!`...)
- Macro Declarations are only partially formatted (the transformed part isn't yet, but could be in the future)
- Macros that can't be formatted are silently ignored

### Are nightly features supported?

Yes! Prettier Rust formats most nightly features. Support depends on [`jinx-rust`](https://github.com/jinxdash/jinx-rust).

## Editor integration

- ### `Recommended` Extension Standalone

  _Easy install + auto-updates_

  - VSCode | Search and install `Prettier - Code formatter (Rust)` [[direct link]](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust)

  - _Request your favorite editor:_ [[file an issue]](https://github.com/jinxdash/prettier-plugin-rust/issues/new)

- ### `Alternative` Core Extension Plugin

  _Requires [NodeJS](https://nodejs.dev/download/) + [Prettier Extension](https://prettier.io/docs/en/editors.html)_ (built-in Jetbrains IDEs)

  ```sh
  npm install --global prettier-plugin-rust prettier
  ```

  _Restart IDE after installing._  
  _To update (manual only!!):_ `npm upgrade --global prettier-plugin-rust prettier`  
  _To check installed version:_ `npm ls -g --depth=0 prettier-plugin-rust prettier`  
  _To check latest version:_ `npm info prettier-plugin-rust version`

## Project integration

- ### Command line

  _Requires [NodeJS](https://nodejs.dev/download/)_

  - Install `prettier` and `prettier-plugin-rust` globally

    ```sh
    npm install --global prettier-plugin-rust prettier
    ```

  - Use the [prettier CLI](https://prettier.io/docs/en/cli.html) to format rust files. E.g. run:

    ```sh
    prettier --write **/*.rs
    ```

- ### NodeJS package

  _Requires [NodeJS](https://nodejs.dev/download/)_

  - Install `prettier` and `prettier-plugin-rust` in the project

    ```sh
    npm install --save-dev prettier-plugin-rust prettier
    ```

  - Link to the plugin's location in your prettier config:

    ```json
    "plugins": ["./node_modules/prettier-plugin-rust"]
    ```

  - Use the [prettier CLI](https://prettier.io/docs/en/cli.html) to format rust files. E.g. run:

    ```sh
    npx prettier --write **/*.rs
    ```

- ### Rust crate

  _No crate yet. Above options are available in the meantime._

---

## Q&A

- ### _Why would I use this and not the established `cargo fmt` ?_

  **It mostly comes down to the Editor Integration.** — With Prettier Rust, you have the ability to hit the `Format Document` keybind on incomplete code and everything fits right into place. Again reiterating on what was said in the introduction, but this is _actually_ life-changing. Just try it out, 1-click install the extension, write a few lines and hit the keybind.

   There's only little difference in terms of how code is printed, so adopting Prettier Rust won't drastically change a codebase. Prettier's only downside against Rustfmt is its troublesome integration into the Rust ecosystem. Fortunately it's only a matter of time before it gets resolved.

  See also:

  - Prettier Rust does not have style options.
  - Prettier Rust produces slightly more readable code (e.g. parenthesis around compound bin ops)
  - Prettier Rust supports more things by default (nightly features, macros, ...)
  - Prettier consistently prints code in the same way, whereas rustfmt preserves arbitrary style at places
  - Prettier is available for many languages (e.g. markdown, html, typescript, java, python, ruby...)
  - Prettier supports language embeds. It formats rust code blocks in non-rust files (e.g. markdown), and conversely formats supported languages within rust doc comments.

- ### _How can Prettier Rust format files that the Rust Compiler cannot parse?_

  Prettier Rust is based on `jinx-rust`, a Rust Parser specially built for Rust Tooling. [Learn more about jinx-rust here.](https://github.com/jinxdash/jinx-rust)

- ### _How does Prettier Rust compare to Prettier Typescript?_

  Prettier Rust is essentially a port of Prettier Typescript. The Rust plugin barely introduces style opinions on its own.