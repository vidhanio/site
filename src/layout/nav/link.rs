use maud::{html, Markup, Render};

#[derive(Copy, Clone, Debug)]
pub struct NavLink<'a> {
    pub name: &'a str,
    pub link: &'a str,
}

impl Render for NavLink<'_> {
    fn render(&self) -> Markup {
        html! {
            li {
                a.text-xl.font-extrabold."text-stone-500".transition-colors."hover:text-stone-700"."dark:hover:text-stone-300"
                href=(self.link) {
                    (self.name)
                }
            }
        }
    }
}
