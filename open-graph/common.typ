#let COLORS = (
  fg: rgb("#753BBD"),
  bg: rgb("#0f0f0f"),
);

#let MARGIN = 30pt

#let logo = image("../static/icon.svg")

#let wordmark = context {
  let vidhanio = [*vidhan.io*]
  let height = measure(vidhanio).height
  let logo = image("../static/icon.svg", height: height)

  stack(spacing: height * 0.75, dir: ltr, vidhanio, logo)
}

#let open-graph(body) = {
  set text(font: "Berkeley Mono", size: 30pt, fill: COLORS.fg, top-edge: "bounds", bottom-edge: "bounds")
  set page(width: 600pt, height: 315pt, margin: MARGIN, fill: COLORS.bg)
  set par(spacing: 0pt)

  body
}
