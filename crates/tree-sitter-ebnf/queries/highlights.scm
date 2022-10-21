;;;; Simple tokens ;;;;
(terminal) @string.grammar

(special_sequence) @string.special.grammar

(integer) @number

(comment) @comment.block

;;;; Identifiers ;;;;
(identifier) @variable.grammar

; Allow different highlighting for specific casings
((identifier) @variable.grammar.pascal
 (#match? @variable.grammar.pascal "^[A-Z]"))

((identifier) @variable.grammar.camel
 (#match? @variable.grammar.camel "^[a-z]"))

((identifier) @variable.grammar.upper
 (#match? @variable.grammar.upper "^[A-Z][A-Z0-9_]+$"))

((identifier) @variable.grammar.lower
 (#match? @variable.grammar.lower "^[a-z][a-z0-9_]+$"))

;;; Punctuation ;;;;
[
 ";"
 ","
] @punctuation.delimiter

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
