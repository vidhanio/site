#let COLORS = sys.inputs.at("color", default: (
  fg: rgb("#753BBD"),
  bg: rgb("#0f0f0f"),
));

#let MARGIN = 30pt

#let logo(size) = {
  let components = range(5)
    .map(i => {
      let offset = size * (i / 10)
      let length = size - 2 * offset
      let lines = (
        curve.line((offset + length, offset)),
        curve.line((offset + length, offset + length)),
        curve.line((offset, offset + length)),
      )

      if calc.even(i) {
        // reverse for even-odd cutouts
        lines = lines.rev()
      }

      (
        curve.move((offset, offset)),
        ..lines,
        curve.close(),
      )
    })
    .flatten()

  curve(
    fill: COLORS.fg,
    ..components,
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
