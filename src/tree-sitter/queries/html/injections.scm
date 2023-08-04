(
  (script_element (raw_text) @injection.content)
  (#set! injection.language "javascript")
)

(
  (style_element (raw_text) @injection.content)
  (#set! injection.language "css")
)

(
  (_) @injection.content
  (#match? @injection.content "^\\{")
  (#match? @injection.content "\\}$")
  (#set! injection.language "rust")
)
