# tree-sitter-ebnf

EBNF grammar for [tree-sitter](https://github.com/tree-sitter/tree-sitter)

## Reference

This parser implements the EBNF syntax as described by the
[ISO/IEC 14977:1996 standard](https://www.iso.org/standard/26153.html) with two
notable differences:

1. The ISO standard does not allow underscores `_` in meta-identifiers
2. The ISO standard only allows characters from the
   [ISO/IEC 646:1991 character set](https://www.iso.org/standard/4777.html)

## Usage in Neovim

### Parser Installation

The [nvim-treesitter plugin](https://github.com/nvim-treesitter/nvim-treesitter)
does not include this parser
[currently](https://github.com/nvim-treesitter/nvim-treesitter/pull/3574). To
use it you must instead manually add it to your tree-sitter config and then
install it with `:TSInstall ebnf` or by adding it to your `ensure_installed`
list:

```lua
require('nvim-treesitter.parsers').get_parser_configs().ebnf = {
    install_info = {
        url = 'https://github.com/RubixDev/ebnf.git',
        files = { 'src/parser.c' },
        location = 'crates/tree-sitter-ebnf',
        branch = 'main',
    },
}
```

### File type detection

You will likely also have to add the `ebnf` file type:

```lua
vim.filetype.add { extension = { ebnf = 'ebnf' } }
```

### Highlighting

If you want to use this parser for highlighting, you will also have to add this
repository as a plugin, for example for
[packer.nvim](https://github.com/wbthomason/packer.nvim) add this:

```lua
use {
    'RubixDev/ebnf',
    rtp = 'crates/tree-sitter-ebnf',
}
```

I also recommend customizing these highlights:

- `@string.grammar`: terminal symbols enclosed with `'` or `"`, falls back to
  `@string`
- `@string.special.grammar`: special sequences enclosed with `?`, falls back to
  `@string.special`
- `@symbol.grammar`: non-terminal symbols, i.e., identifiers, falls back to
  `@symbol`
  - `@symbol.grammar.pascal`: non-terminal symbols in PascalCase
  - `@symbol.grammar.camel`: non-terminal symbols in camelCase
  - `@symbol.grammar.upper`: non-terminal symbols in UPPERCASE
  - `@symbol.grammar.lower`: non-terminal symbols in lowercase

As an example, here is my personal configuration:

```lua
vim.api.nvim_set_hl(0, '@string.special.grammar', { link = '@string.regex' })
vim.api.nvim_set_hl(0, '@symbol.grammar.pascal', { link = '@type' })
vim.api.nvim_set_hl(0, '@symbol.grammar.camel', { link = '@property' })
vim.api.nvim_set_hl(0, '@symbol.grammar.upper', { link = '@constant' })
vim.api.nvim_set_hl(0, '@symbol.grammar.lower', { link = '@parameter' })
```
