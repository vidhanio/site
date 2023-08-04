---
title: "hello, world!"
description: "first post!"
date: 2023-08-04
---

hi, i'm Vidhan! welcome to my new site and blog, rewritten from the ground up using [rust](https://www.rust-lang.org/).
this site mainly uses two libraries, namely the http server, [axum](https://docs.rs/axum) and the html
"templating" library (written by myself!), [html-node](https://docs.rs/axum). One of the coolest features
of this site, the code block syntax highlighting is powered by [tree-sitter](https://tree-sitter.github.io/tree-sitter/).

it also uses [tailwind css](https://tailwindcss.com/) for styling.

take a look at a cool code block example below!

```rust
pub fn document(path: Option<&str>, content: Node) -> Node {
    html! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                {seo(path)}
                <link rel="stylesheet" href="/public/css/styles.css" />
            </head>

            <body class="bg-slate-100 font-mono text-slate-900 [font-feature-settings:'ss05'] dark:bg-slate-900 dark:text-slate-100">
                {layout(content)}
            </body>
        </html>
    }
}
```