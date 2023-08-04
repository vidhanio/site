(macro_invocation
    macro: (identifier) @_html (#eq? @_html "html")
    (token_tree) @injection.content
  (#set! injection.language "html")
  (#set! injection.combined)
  (#set! injection.include-children))
  

(call_expression
  function: (scoped_identifier
    path: (identifier) @_regex (#eq? @_regex "Regex")
    name: (identifier) @_new (#eq? @_new "new"))
  arguments: (arguments
    (raw_string_literal) @regex))

(call_expression
  function: (scoped_identifier
    path: (scoped_identifier (identifier) @_regex (#eq? @_regex "Regex").)
    name: (identifier) @_new (#eq? @_new "new"))
  arguments: (arguments
    (raw_string_literal) @regex))
