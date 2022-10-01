# dprint-plugin-ebnf

Format EBNF grammar notations through [dprint](https://dprint.dev/).

## Install

Add the plugin to your config file by running `dprint config add RubixDev/ebnf`.

Don't forget to add `ebnf` to your `includes` pattern.

## Configuration

This plugin uses the `"ebnf"` config key. These options are available:

| Name                        | Type          | Default                 | Possible values                        | Description                                                                                 |
| --------------------------- | ------------- | ----------------------- | -------------------------------------- | ------------------------------------------------------------------------------------------- |
| `lineWidth`                 | `u32`         | global config or `100`  |                                        | Always wrap at the next possible point after this line width is reached                     |
| `indentWidth`               | `u8`          | global config or `2`    |                                        | The number of spaces to indent multiline comments                                           |
| `newLineKind`               | `NewLineKind` | global config or `"lf"` | `"auto"`, `"lf"`, `"crlf"`, `"system"` | The kind of line endings to use                                                             |
| `quoteStyle`                | `QuoteStyle`  | `"Single"`              | `"Single"`, `"Double"`                 | The preferred kind of quotes to use for terminal string                                     |
| `ignoreRuleCommentText`     | `String`      | `"dprint-ignore"`       |                                        | The text a comment should contain to ignore formatting for the next syntax rule             |
| `multilineCommentsMarkdown` | `bool`        | `true`                  | `true`, `false`                        | Format multiline comments like markdown (requires `dprint-plugin-markdown` to be installed) |
