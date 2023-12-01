use maud::{html, Markup, Render};

pub struct FooterLink<'a> {
    pub name: &'a str,
    pub url: &'a str,
    pub rel: Option<&'a str>,
}

impl Render for FooterLink<'_> {
    fn render(&self) -> Markup {
        html! {
            li {
                a.text-sm.font-light."hover:text-stone-500".transition-colors href=(self.url) rel=[self.rel] {
                    (self.name)
                }
            }
        }
    }
}
