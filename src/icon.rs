//! icons sourced from heroicons.

use maud::{html, Markup, Render};

pub struct Icon {
    pub d: &'static str,
    pub class: &'static str,
}

impl Render for Icon {
    fn render(&self) -> Markup {
        html! {
            svg.(self.class) xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" {
                path fill-rule="evenodd" d=(self.d) clip-rule="evenodd";
            }
        }
    }
}

pub const CHEVRON: Icon = Icon {
    d: "M16.28 11.47a.75.75 0 010 1.06l-7.5 7.5a.75.75 0 01-1.06-1.06L14.69 12 7.72 5.03a.75.75 0 011.06-1.06l7.5 7.5z",
    class: "h-8",
};
