#let COLORS = sys.inputs.at("colors", default: (
  fg: rgb("#753BBD"),
  bg: rgb("#0f0f0f"),
));

#let MARGIN = 30pt

#let logo(size) = {
  let svg-data = read("../static/icon.svg")
    .replace("/*{light.fg}*/", COLORS.fg.to-hex())
    .replace("/*{dark.fg}*/", COLORS.fg.to-hex())
    .replace("/*{light.bg}*/", COLORS.bg.to-hex())
    .replace("/*{dark.bg}*/", COLORS.bg.to-hex())

  image(
    bytes(svg-data),
    width: size,
    height: size,
  )
}

#let wordmark = context {
  let vidhanio = [*vidhan.io \/*]
  let height = measure(vidhanio).height
  let logo = logo(height)

  stack(spacing: height * 0.75, dir: ltr, vidhanio, logo)
}

#let open-graph(body) = {
  set text(font: "Berkeley Mono", size: 30pt, fill: COLORS.fg, top-edge: "bounds", bottom-edge: "bounds")
  set page(width: 600pt, height: 315pt, margin: MARGIN, fill: COLORS.bg)
  set par(spacing: 0pt)

  body
}
