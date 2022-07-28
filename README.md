<div align="center">
  <img alt="Prettier Rust" height="256px" src="https://user-images.githubusercontent.com/109366411/181039409-b66d6a4c-bbc7-4fbb-8a79-d7bb1af87a63.png">
</div>

<h1 align="center">Prettier Rust</h1>

![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg) [![npm version](https://img.shields.io/npm/v/prettier-plugin-rust.svg?style=flat)](https://www.npmjs.com/package/prettier-plugin-rust) ![GitHub Repo stars](https://img.shields.io/github/stars/jinxdash/prettier-plugin-rust?style=social) [![Twitter Follow](https://img.shields.io/twitter/follow/jinxdash?style=social)](https://twitter.com/jinxdash)

_The massively popular [Prettier](https://prettier.io/) code formatter, now with [Rust](https://www.rust-lang.org/) support!_

-   **Get Started:** Search for the [Prettier - Code formatter (Rust)](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) extension in VSCode.

### A _formatter_ -- not a linter!

Prettier Rust doesn't panic on missing semicolons, _in fact it auto-completes them!_

Thanks to the specially built [`jinx-rust`](https://www.github.com/jinxdash/jinx-rust/) parser, Prettier is completely independent from Rust's strict validation process.  
As such, it is able to format rust just like it formats typescript!

## Configuration

Configure `prettier` by creating a `.prettierrc.toml` file, or [any other file format listed in prettier docs](https://prettier.io/docs/en/configuration.html).

| API Option                   | CLI Option                     | Default | Docs                                                           |
| ---------------------------- | ------------------------------ | :-----: | -------------------------------------------------------------- |
| `useTabs`                    | --use-tabs                     |  false  | _[link](https://prettier.io/docs/en/options.html#tabs)_        |
| `tabWidth`                   | --tab-width                    |    4    | _[link](https://prettier.io/docs/en/options.html#tab-width)_   |
| `printWidth`                 | --print-width                  |   100   | _[link](https://prettier.io/docs/en/options.html#print-width)_ |
| `endOfLine`                  | --end-of-line                  |  "lf"   | _[link](https://prettier.io/docs/en/options.html#end-of-line)_ |
| `trailingComma`              | --trailing-comma               |         | Not supported yet.                                             |
| `embeddedLanguageFormatting` | --embedded-language-formatting |         | Not supported yet.                                             |

### Ignoring stuff

To ignore files, list them in `.prettierignore` (like you would `.gitignore`)

To ignore something in a file, add `// prettier-ignore` above it.

`#[rustfmt::skip]` and `#![rustfmt::skip]` are also supported (locally).

## Installation

### VSCode

-   Install the standalone extension [Prettier - Code formatter (Rust)](https://marketplace.visualstudio.com/items?itemName=jinxdash.prettier-rust) **[recommended]**

-   Alternatively, if your project uses javascript and includes a `package.json`, you may install the plugin locally and use the official extension.  
    _Note that the Prettier extension has issues auto-loading plugins, and it won't auto-update them._
    <details>
    <summary>Instructions</summary>

    -   Install the official extension [Prettier - Code formatter](https://marketplace.visualstudio.com/items?itemName=esbenp.prettier-vscode)

    -   Install the plugin in the project

    ```
    npm install --save-dev prettier-plugin-rust
    ```

    -   The plugin _should_ be detected, some things can help if it doesn't:

        -   Add `"plugins": ["prettier-plugin-rust"]` to your project's prettier config
        -   If your project only uses prettier for rust files:  
            Try adding `"parser": "jinx-rust"`, (that's the parser used by `prettier-plugin-rust`).
        -   [Read more about Prettier plugins](https://prettier.io/docs/en/plugins.html)
        -   Use the standalone extension instead.

</details>


### crate

_Not available yet_

### npm

The plugin is published on npm as `prettier-plugin-rust` and can be used like any other prettier plugin in the CLI.

```
npm install --save-dev prettier-plugin-rust
```

Refer to the [Prettier documentation](https://prettier.io/docs/en/cli.html) for available CLI commands.
