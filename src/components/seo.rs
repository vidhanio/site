use html_node::{html, text, Node};

pub fn seo(path: Option<&str>) -> Node {
    let title = path.map_or_else(|| "vidhan".into(), |path| format!("vidhan / {path}"));
    let url = path.map_or_else(
        || "https://vidhan.io".into(),
        |path| format!("https://vidhan.io/{path}"),
    );

    html! {
        <head>
            <title>{text!("{title}")}</title>
            <meta name="viewport" content="width=device-width, initial-scale=1.0" />
            <meta charset="utf-8" />

            <title>{text!("{title}")}</title>

            <meta name="description" content="vidhan's home on the internet." />
            <meta name="theme-color" content="#6466e9" />
            <link rel="icon" href="/favicon.ico" />

            <meta name="og:title" content=&title />
            <meta name="og:description" content="vidhan's home on the internet." />
            <meta name="og:image" content="https://vidhan.io/images/og-image.png" />
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
            <meta
                name="twitter:image"
                content="https://vidhan.io/images/og-image.png"
            />
            <meta name="og:url" content=&url />
        </head>
    }
}
