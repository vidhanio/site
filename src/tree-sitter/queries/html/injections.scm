(
  (script_element (raw_text) @injection.content)
  (#set! injection.language "javascript")
)

(
  (style_element (raw_text) @injection.content)
  (#set! injection.language "css")
)

(
  (_) @_rust @injection.content
  (#match? @_rust "^\\{")
  (#match? @injection.content "\\}$")
  (#set! injection.language "rust")
)
