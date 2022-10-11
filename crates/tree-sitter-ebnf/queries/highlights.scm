;;;; Simple tokens ;;;;
(terminal) @string @grammar.terminal

(special_sequence) @string.special @grammar.special

(integer) @number

(comment) @comment.block

;;;; Identifiers ;;;;
(identifier) @variable @grammar.nonterminal

; Allow different highlighting for specific casings
((identifier) @grammar.nonterminal.pascal
 (#match? @grammar.nonterminal.pascal "^[A-Z]"))

((identifier) @grammar.nonterminal.camel
 (#match? @grammar.nonterminal.camel "^[a-z]"))

((identifier) @grammar.nonterminal.upper
 (#match? @grammar.nonterminal.upper "^[A-Z][A-Z0-9_]+$"))

((identifier) @grammar.nonterminal.lower
 (#match? @grammar.nonterminal.lower "^[a-z][a-z0-9_]+$"))

;;; Punctuation ;;;;
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
