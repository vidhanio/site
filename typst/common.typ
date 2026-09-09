#let COLORS = sys.inputs.at("colors", default: (
  fg: rgb("#753BBD"),
  bg: rgb("#0f0f0f"),
));

#let MARGIN = 30pt

#let logo(size) = text(
  font: "Berkeley Mono",
  size: size,
  weight: "bold",
  fill: COLORS.fg,
)[\[v\]]

#let open-graph(body) = {
  set text(font: "Berkeley Mono", size: 30pt, fill: COLORS.fg, top-edge: "bounds", bottom-edge: "bounds")
  set page(width: 600pt, height: 315pt, margin: MARGIN, fill: COLORS.bg)
  set par(spacing: 0pt)

  body
}
