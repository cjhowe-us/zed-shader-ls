(int_literal) @number
(float_literal) @number
(bool_literal) @constant

(type_declaration [ "bool" "u32" "i32" "f16" "f32" ] @type.builtin)
(type_declaration) @type

(function_declaration
    (identifier) @function)

(parameter
    (variable_identifier_declaration (identifier) @variable.parameter))

(struct_declaration
    (identifier) @type)

(struct_declaration
    (struct_member (variable_identifier_declaration (identifier) @property)))

(attribute
    (identifier) @attribute)

(identifier) @variable

(type_constructor_or_function_call_expression
    (type_declaration) @function)

[
    "struct"
    "bitcast"
    "discard"
    "enable"
    "fallthrough"
    "let"
    "type"
    "var"
    "override"
    (texel_format)
] @keyword

[
    "private"
    "storage"
    "uniform"
    "workgroup"
] @keyword.storage

[
    "read"
    "read_write"
    "write"
] @keyword

"fn" @keyword

"return" @keyword

[ "," "." ":" ";" "->" ] @punctuation.delimiter

["(" ")" "[" "]" "{" "}"] @punctuation.bracket

[
    "loop"
    "for"
    "while"
    "break"
    "continue"
    "continuing"
] @keyword

[
    "if"
    "else"
    "switch"
    "case"
    "default"
] @keyword

[
    "&"
    "&&"
    "/"
    "!"
    "="
    "=="
    "!="
    ">"
    ">="
    ">>"
    "<"
    "<="
    "<<"
    "%"
    "-"
    "+"
    "|"
    "||"
    "*"
    "~"
    "^"
    "@"
    "++"
    "--"
] @operator

[
    (line_comment)
    (block_comment)
] @comment
