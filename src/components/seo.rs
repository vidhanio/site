use html_node::{html, Node, Text};

pub fn seo(path: Option<&str>) -> Node {
    let title = path.map_or_else(
        || "vidhan".into(),
        |path| format!("vidhan / {}", path.replace('/', " / ")),
    );
    let url = path.map_or_else(
        || "https://vidhan.io".into(),
        |path| format!("https://vidhan.io/{path}"),
    );

    html! {
        <head>
            <title>{Text::from(&title)}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta charset="utf-8" />

            <title>{Text::from(&title)}</title>

            <meta name="description" content="vidhan's home on the internet." />
            <meta name="theme-color" content="#6466e9" />
            <link rel="icon" href="/public/favicon.ico" />

            <meta name="og:title" content=&title />
            <meta name="og:description" content="vidhan's home on the internet." />
            <meta name="og:url" content=&url />
            <meta name="og:type" content="website" />

            <meta name="twitter:card" content="summary_large_image" />
            <meta name="twitter:site" content="@vidhanio" />
            <meta name="twitter:creator" content="@vidhanio" />
            <meta name="twitter:title" content=&title />
            <meta
                name="twitter:description"
                content="vidhan's home on the internet."
            />
            <meta name="og:url" content=&url />
        </head>
    }
}
