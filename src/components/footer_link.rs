use html_node::{html, text, Node};

#[derive(Copy, Clone, Debug)]
pub struct FooterLink<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub rel: Option<&'a str>,
}

impl<'a> From<FooterLink<'a>> for Node {
    fn from(link: FooterLink<'a>) -> Self {
        html! {
            <li>
                <a
                    href=link.url
                    class="text-sm font-light italic text-slate-400 underline transition-colors hover:text-slate-600 dark:text-slate-600 dark:hover:text-slate-400"
                    rel=link.rel as &str
                >
                    {text!("{}", link.name)}
                </a>
            </li>
        }
    }
}
