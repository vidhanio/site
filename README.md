# [`vidhan.io`](https://vidhan.io)

my personal site, written in rust.

## stack

- http server: [axum](https://docs.rs/axum)
- html templating: [maud](https://docs.rs/maud)
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
