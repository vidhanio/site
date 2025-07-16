---
date: 2023-12-10
---

# hello, world

hi, i'm vidhan! welcome to my new site, written from the ground up using [rust](https://www.rust-lang.org/).
this site mainly uses two libraries, namely the http server, [axum](https://docs.rs/axum) and the html library
[hypertext](https://docs.rs/hypertext). one of the coolest features of this site, the code block syntax highlighting,
is powered by [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

take a look at the code which powers this site's syntax highlighting:

```rust
pub fn highlight(&self, language: &str, code: &str) -> Result<String, Box<dyn Error>> {
    let Some(config) = self.0.get(language) else {
        return Ok(html_escape::encode_text_minimal(code).into());
    };

    let mut highlighter = Highlighter::new();

    let mut highlights =
        highlighter.highlight(config, code.as_bytes(), None, |lang| self.0.get(lang))?;

    highlights.try_fold(String::new(), |mut buf, event| {
        match event? {
            HighlightEvent::Source { start, end } => {
                html_escape::encode_text_minimal_to_string(&code[start..end], &mut buf);
            }
            HighlightEvent::HighlightStart(Highlight(idx)) => {
                write!(
                    buf,
                    "<span class=\"{}\">",
                    HIGHLIGHT_NAMES[idx].replace('.', " ")
                )
                .expect("writing to a string should be infallible");
            }
            HighlightEvent::HighlightEnd => {
                buf.push_str("</span>");
            }
        }

        Ok(buf)
    })
}
```
