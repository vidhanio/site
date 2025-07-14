#import "common.typ": *

#show: open-graph

#let footer = {
  set text(size: 25pt)

  pad(y: MARGIN, line(length: 100%, stroke: (paint: COLORS.fg, thickness: 2pt)))
  align(right, wordmark)
}


#let truncate(text, f) = {
  layout(size => {
    let text = text
    let fits(text) = measure(width: size.width, f(text)).height <= size.height - measure(footer).height

    if fits(text) {
      return f(text)
    } else {
      while not fits(text + "…") {
        text = text.slice(0, -1).trim()
      }
      return f(text + "…")
    }
  })
}


#[
  #let post-title = sys.inputs.at("post-title", default: "hello, world!")

  #truncate(post-title, post-title => [*post / * #post-title])
]

#align(bottom, footer)
