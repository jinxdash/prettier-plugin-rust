<div align="center">
  <img alt="Prettier Rust" height="256px" src="https://user-images.githubusercontent.com/109366411/181039409-b66d6a4c-bbc7-4fbb-8a79-d7bb1af87a63.png">
</div>

<h1 align="center">Prettier Rust</h1>

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/prettier-plugin-rust.svg?style=flat)](https://www.npmjs.com/package/prettier-plugin-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/prettier-plugin-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

_The massively popular [Prettier](https://prettier.io/) code formatter, now with [Rust](https://www.rust-lang.org/) support!_

-   **Get Started:** Search for the [Prettier - Code formatter (Rust)](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) extension in VSCode.

### Why Prettier?

_Prettier Rust brings 1:1 the great developer experience millions expect and love from Prettier Typescript to the Rust Language._

-   _Press save and the code is formatted._ What usually happens once people start using Prettier is that they realize how much time and mental energy they actually spend formatting their code. With Prettier editor integration, you can just press that magic key binding and poof, the code is formatted. This is an eye-opening experience if anything.

-   _Put an end to style debates._ Prettier is not a kitchen-sink code formatter that attempts to print your code in any way you wish. It is opinionated, and fully automatic.

-   _Prettier Rust is completely independent from Rust's Compiler and validation process._ It couldn't care less if your code doesn't compile, is missing annotations and has minor syntax issues. It formats and fills in the formalities on your behalf.

### Example

```rs
const LEET = 1337
/// My WIP code draft
#![feature(crate_visibility_modifier)]
async crate fn foo(arg) {
  arg.0 *= 3.14 + LEET & 1337
  arg.1(|b, c| -> B &c).await
}
```

-   #### Prettier Rust

    ```rs
    const LEET = 1337;
    #![feature(crate_visibility_modifier)]
    /// My WIP code draft
    crate async fn foo(arg) {
        arg.0 *= (3.14 + LEET) & 1337;
        (arg.1)(|b, c| -> B { &c }).await
    }
    ```

-   <details>
      <summary>Compare with cargo fmt</summary>

    ```sh
    # Actual attempt:
    [ERROR] ...84 lines, 9 errors, 6 help, 4 notes
    # After manually fixing all issues:
    +4 whitespace characters ✓
    ```

    </details>

## Configuration

Create a `.prettierrc.toml` file, or [any other supported format](https://prettier.io/docs/en/configuration.html).

```toml
# .prettierrc.toml # And yes, this is the full list
useTabs = false
tabWidth = 4
printWidth = 100
endOfLine = "lf"
trailingComma = "es5"               # not supported yet
embeddedLanguageFormatting = "auto" # not supported yet
```

### Ignoring stuff

To ignore files, list them in a `.prettierignore` file (same format as `.gitignore`)

To ignore something in a file, add `// prettier-ignore` above it.

`#[rustfmt::skip]` and `#![rustfmt::skip]` are also supported locally.

## Editor integration

-   ### _Recommended:_ Standalone Extension

    _Easy install, auto-updates._

    -   VSCode | Search and install `Prettier - Code formatter (Rust)` [[direct link]](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust)

    -   _Request your favorite editor:_ [[file an issue]](https://github.com/jinxdash/prettier-plugin-rust/issues/new)

-   ### _Alternative:_ Core Extension Plugin

    _Requires [NodeJS](https://nodejs.dev/download/) + [Prettier Extension](https://prettier.io/docs/en/editors.html)_ (built-in Jetbrains IDEs)

    ```sh
    npm install --global prettier-plugin-rust prettier
    ```

    _To update:_ `npm upgrade --global prettier-plugin-rust prettier`  
    _To check installed version:_ `npm ls -g --depth=0 prettier-plugin-rust prettier`  
    _To check latest version:_ `npm info prettier-plugin-rust version`

## Project integration

-   ### Rust Project

    _Not available yet_

    > In javascript projects, one usually installs prettier (and plugins if any) as devonly dependencies. The `prettier` CLI and extension then load prettier packages from deps, and this ensures that everyone working on the project has the same configuration. This doesn't interop with rust projects, as there's neither a `package.json` file or a way to install npm packages. There is no definitive solution as of time of writing, and help on this matter is appreciated.

-   ### Rust Project with npm

    _Requires [NodeJS](https://nodejs.dev/download/)_

    -   Install `prettier` and `prettier-plugin-rust` in the project

        ```sh
        npm install --save-dev prettier-plugin-rust prettier
        ```

    -   Link to the plugin's location in your prettier config:

        ```json
          "plugins": ["./node_modules/prettier-plugin-rust"]
        ```

    -   Prettier CLI can now format rust files. E.g. run:
        ```sh
        prettier --write **/*.rs
        ```

    Refer to the [Prettier documentation](https://prettier.io/docs/en/cli.html) for all available CLI commands.

    > _If you opted not to use the Standalone Extension_: Be aware that there is an issue with the core Prettier Extension where it's not able to register correctly as a rust formatter in a project that has the rust plugin installed. To make it register, you need to install any version of the plugin globally and reload your IDE. This issue does not concern the CLI.
