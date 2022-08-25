<div align="center">
  <img alt="Prettier Rust" height="256px" src="https://user-images.githubusercontent.com/109366411/181039409-b66d6a4c-bbc7-4fbb-8a79-d7bb1af87a63.png">
</div>

<h1 align="center">Prettier Rust</h1>

<div align="center">

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/prettier-plugin-rust.svg?style=flat)](https://www.npmjs.com/package/prettier-plugin-rust) [![extension installs](https://img.shields.io/visual-studio-marketplace/i/jinxdash.prettier-rust?logo=visualstudiocode&style=social)](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/prettier-plugin-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

_The massively popular [Prettier](https://prettier.io/) code formatter, now with [Rust](https://www.rust-lang.org/) support!_

**Get Started:** Install [VSCode Extension](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) `Prettier - Code formatter (Rust)`

</div>

## Why Prettier?

> What usually happens once people start using Prettier is that they realize how much time and mental energy they actually spend formatting their code. No matter how incomplete or broken the code you're working on is, with the Prettier Editor Extension you can always just press the `Format Document` key binding and \*poof\*, the code snaps right into place.

<br>

- **Beautiful, uniform and consistent** — Prettier is strongly opinionated, with no style options.
- **There when you need it the most** — Prettier can format code that won't compile _(e.g. missing annotations)_
- **Speed up the day-to-day** — Prettier auto-fixes common syntax errors _(e.g. missing semicolons, blocks, parentheses)_

<br>

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

<br>

## Configuration

https://prettier.io/docs/en/configuration

<!-- prettier-ignore -->
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
    <summary>See alternative configuration using a TOML file</summary>

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
- Popular/built-in macros with original syntax rules get custom formatting (e.g. `matches!`, `if_chains!`...) _[Not implemented yet]_
- Macro Declarations are only partially formatted (the transformed part isn't yet, but could be in the future)
- Macros that can't be formatted are silently ignored

### Are nightly features supported?

Yes! Prettier Rust formats most nightly features. Support depends on [`jinx-rust`](https://github.com/jinxdash/jinx-rust).

<br>

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

<br>

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

  - You can also use the plugin programmatically:

    ```ts
    import prettier from "prettier";
    import * as rustPlugin from "prettier-plugin-rust";

    prettier.format(code, { plugins: [rustPlugin] });
    ```

- ### Rust crate

  _No crate yet. Above options are available in the meantime._

<br>

## Q&A

- ### _Why would I use this and not the established `cargo fmt` ?_

  _It's all about the Editor Integration_ — Having the ability to format your code while you work on it really makes for a great developer experience, and autocompletion for Rust's strict syntax is such a massive time save. Once you've tried the extension there really is no coming back.

  All-in-all the difference in code style is minimal, so adopting Prettier Rust won't drastically change your codebase. The real downside is the harsher integration with the Rust ecosystem, but it'll get better eventually.

  Point by point:

  - the extension streamlines your work in the editor
    - it can format code that won't compile _(e.g. code with missing type annotations)_
    - it autocorrects syntax errors _(e.g. missing semicolons, blocks, parentheses...)_
  - it is strongly opinionated with no style options, so code is uniform across projects.
  - it produces more readable code in some cases (e.g. condition chains, compound expressions, patterns)
  - it supports everything out-of-the-box (e.g. nightly features, macros)
  - it consistently prints code in the same way, whereas Rustfmt preserves arbitrary style at places
  - it can be used for other languages (e.g. markdown, html, typescript, java, python, ruby)
  - it formats language embeds. So rust code blocks in non-rust files (e.g. markdown), and supported languages in rust doc comments. _[NOTE: the latter is not yet implemented]_

- ### _Why not just add those features to rustfmt instead?_

  Unfortunately Rustfmt cannot implement those features by design.

  Rustfmt parses code with rustc. Rustc is strict and unforgiving as it always assumes code is at its "final version", thus every slight deviation from the accepted syntax crashes the parser. There's also that rustc has many lint-like checks within the parser. The intention is to save work for the compiler down the line, unfortunately it also means that rustc sometimes fails to parse syntactically correct code.

  Prettier Rust however is based on [jinx-rust](https://github.com/jinxdash/jinx-rust). Jinx-rust is built specifically for Rust tooling. Hence it's designed to tolerate a wide range of syntax errors, supports missing nodes and sometimes even infers user intent (e.g. Javascript's `!==`)

  Jinx-rust has a little _plaidoyer_ in its readme arguing for Rust Tooling _not_ to use the official rustc parser [here](https://github.com/jinxdash/jinx-rust#why-jinx-rust-and-why-in-typescript).

- ### _When exactly does Prettier Rust change code syntax?_

  The Prettier Rust syntax autocorrection feature is intended to be an adaptation of how Prettier Typescript autocorrects javascript code with missing semicolons.

  You can effectively think of Prettier Rust syntax autocorrection as auto-applying the Rust compiler's suggested syntax fixes automatically (e.g. "semicolon missing here", "parenthesize this" or "add a block around that")

  Otherwise if your codebase compiles, then Prettier Rust is just a formatter like any other. It won't change the syntax of valid rust code. Moreover, it doesn't reorganize imports, split comments or combine attributes.

- ### _This is an "opinionated formatter". But it's brand new! How reliable are those opinions?_

  Rest assured, Prettier Rust actually does not take style decisions on its own. Prettier Rust is essentially a 1:1 adaptation of Prettier Typescript, hence the opinions it implements have been battle tested and agreed-upon by [millions and millions of users](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode) already.
