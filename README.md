# [`vidhan.io`](https://vidhan.io)

my personal site, written in rust.

## stack

- http server: [axum](https://docs.rs/axum)
- html templating: [hypertext](https://docs.rs/hypertext) (made by me!)
- syntax highlighting: [tree-sitter](https://tree-sitter.github.io/tree-sitter/)
- resume: [typst](https://typst.app)

## features

- no javascript
- completely usable with css disabled
- automated ci/deployment via github actions, deployed to google cloud run on push
- a blog generated automatically from markdown
- server-side syntax highlighting via tree-sitter, the same engine which powers syntax highlighting in full-fledged ides
- resume (made with typst) built and included at compile-time, and served like a normal file
- optimal caching of static assets which is automatically cache busted on commit via the git commit hash

## development

Run `cargo run --features reload` while working on the site. In this mode,
the `vidhan-site-assets` crate in `assets/` reloads and processes the stylesheet,
posts, media, fonts, resume, icons, and open graph images for every request, so
asset edits are visible without rebuilding or restarting the server.

Release builds use the default feature set. The build script processes the same
assets through `vidhan-site-assets` and embeds the results in the executable.
