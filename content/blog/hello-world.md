---
title: hello, world!
date: 2023-08-04
description: welcome to my new website!
---

hi, i'm vidhan! welcome to my new site and blog, rewritten from the ground up using [rust](https://www.rust-lang.org/).
this site mainly uses two libraries, namely the http server, [axum](https://docs.rs/axum) and the html library
[maud](https://docs.rs/maud). one of the coolest features of this site, the code block syntax highlighting
is powered by [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

it also uses [tailwind css](https://tailwindcss.com/) for styling.

take a look at a cool code block example below!

```rust
pub fn document(path: Option<&str>, content: Markup) -> Markup {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                { seo(path) }
                <link rel="stylesheet" href="/static/styles.css" />
            </head>

            <body class="bg-stone-100 font-mono text-stone-900 [font-feature-settings:'ss05'] dark:bg-stone-900 dark:text-stone-100">
                { layout(content) }
            </body>
        </html>
    }
}
```
