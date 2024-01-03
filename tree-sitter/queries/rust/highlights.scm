; Macros

(macro_definition
  name: (identifier) @function)

(token_binding_pattern
  name: (metavariable) @variable.parameter)

(fragment_specifier) @type.builtin

(macro_invocation
  macro: [
    name: (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
  ]
  "!" @function)

; Declarations

(mod_item
  name: (identifier) @module)

(enum_variant
  name: (identifier) @constructor)

(extern_crate_declaration
  name: (identifier) @module
  alias: (identifier)? @module)

(const_item
  name: (identifier) @constant)

(static_item
  name: (identifier) @constant)

(function_item
  name: (_) @function)

(function_signature_item
  name: (_) @function)

(const_parameter
  name: (identifier) @constant)

(parameter
  pattern: (identifier) @variable.parameter
  (#match? @variable.parameter "^[a-z\\d_]+$"))

(closure_parameters
  (identifier) @variable.parameter)

; Function Calls

(call_expression
  function: [
    (identifier) @function
    (scoped_identifier
      name: (identifier) @function)
    (field_expression
      field: (field_identifier) @function)
  ])


; Types

(empty_type) @type.builtin

(generic_function
  function: (identifier) @function)

; Scoped Identifiers

; ; Modules

(scoped_identifier
  path: [
    (identifier) @module
    (scoped_identifier
      name: (identifier) @module)
  ]
  (#match? @module "^[a-z\\d_]+$"))

(scoped_type_identifier
  path: [
    (identifier) @module
    (scoped_identifier
      name: (identifier) @module)
  ]
  (#match? @module "^[a-z\\d_]+$"))

(scoped_use_list
  path: [
    (identifier) @module
    (scoped_identifier
      name: (identifier) @module)
  ]
  (#match? @module "^[a-z\\d_]+$"))

(use_declaration
  argument: (identifier) @module)

[
  (crate)
  (super)
] @module

; ; ; `self` Special Cases

(scoped_identifier
	name: (identifier) @module
    (#eq? @module "self"))

(scoped_identifier
	path: (self) @module)

(use_list (self) @module)

; ; Enums/Associated Types

(scoped_identifier
  path: [
    (identifier) @type
    (scoped_identifier
      name: (identifier) @type)
  ]
  name: [
    (
      (identifier) @constant
      (#match? @constant "^[A-Z][A-Z\\d_]+$")
    )
    (
      (identifier) @constructor
      (#match? @constructor "(^[A-Z]|^new$)")
    )
  ]
  (#match? @type "^[A-Z]"))

(struct_expression
  (scoped_type_identifier
    path: [
      (identifier) @type
      (scoped_identifier
        name: (identifier) @type)
    ]
    name: [
      (
        (type_identifier) @constructor
        (#match? @constructor "^[A-Z]")
      )
    ]
    (#match? @type "^[A-Z]")))

; Fields

(field_identifier) @property

(shorthand_field_initializer
  (identifier) @property)

; Patterns

(tuple_struct_pattern
  type: [
    (identifier) @type
    (scoped_identifier
      name: (identifier) @type)
  ])

; ; assume that these are glob-imported enum variants
(match_pattern
  (identifier) @constructor
  (#match? @constructor "^[A-Z]"))

(remaining_field_pattern) @punctuation
(tuple_struct_pattern "_" @punctuation)
(field_pattern "_" @punctuation)

; Literals

[
  (negative_literal)
  (integer_literal)
  (float_literal)
] @number

[
  (string_literal)
  (char_literal)
] @string
(raw_string_literal) @string.special

(escape_sequence) @escape

(boolean_literal) @constant.builtin

[
  (line_comment)
  (block_comment)
] @comment

; Types

(
  (type_identifier) @type.builtin
  (#eq? @type.builtin "Self")
)

(primitive_type) @type.builtin

(type_identifier) @type

; Case-Based Inference

(
  (identifier) @constant
  (#match? @constant "^[A-Z][A-Z\\d_]+$")
)

(
  (identifier) @type
  (#match? @type "^[A-Z]")
)

(identifier) @variable

; Built-In Variables

(self) @variable.builtin

; Keywords

[
  "as"
  "async"
  "await"
  "break"
  "const"
  "continue"
  "dyn"
  "else"
  "enum"
  "extern"
  "fn"
  "for"
  "if"
  "impl"
  "in"
  "let"
  "loop"
  "macro_rules!"
  "match"
  "mod"
  "move"
  (mutable_specifier)
  "pub"
  "ref"
  "return"
  "static"
  "struct"
  "trait"
  "type"
  "unsafe"
  "use"
  "where"
  "while"
] @keyword


; Punctuation

["(" ")" "[" "]" "{" "}"] @punctuation.bracket
(closure_parameters "|" @punctuation.bracket)

["," "." ":" "::" ";" "->" "=>"] @punctuation.delimiter

; ; Generic Brackets
(type_parameters ["<" ">"] @punctuation.bracket)
(bracketed_type ["<" ">"] @punctuation.bracket)
(for_lifetimes ["<" ">"] @punctuation.bracket)
(type_arguments  ["<" ">"] @punctuation.bracket)

; ; Attributes
(attribute_item "#" @punctuation)
(inner_attribute_item ["!" "#"] @punctuation)

; Operators

[
  "!"
  "!="
  "%"
  "%="
  "&"
  "&&"
  "&="
  "*"
  "*="
  "+"
  "+="
  "-"
  "-="
  ".."
  "..="
  "/"
  "/="
  "<"
  "<<"
  "<<="
  "<="
  "="
  "=="
  ">"
  ">="
  ">>"
  ">>="
  "?"
  "@"
  "^"
  "^="
  "|"
  "|="
  "||"
] @operator

