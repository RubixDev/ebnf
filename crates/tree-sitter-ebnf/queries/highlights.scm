(terminal) @string @grammar.terminal

; Default to string.regex
(special_sequence) @string.regex @grammar.other

(integer) @constant.numeric @number

(comment) @comment.block

; Default to parameter
(identifier) @parameter @grammar.nonterminal
; Allow different highlighting for fully UPPERCASE and lowercase identifiers
((identifier) @grammar.nonterminal.upper
 (#match? @grammar.nonterminal.upper "^[A-Z][A-Z0-9_]+$"))
((identifier) @grammar.nonterminal.lower
 (#match? @grammar.nonterminal.lower "^[a-z][a-z0-9_]+$"))

[
 ";"
 ","
] @punctuation.delimeter

[
 "|"
 "*"
 "-"
] @operator

"=" @keyword.operator

[
 "("
 ")"
 "["
 "]"
 "{"
 "}"
] @punctuation.bracket
