use maud::{Markup, PreEscaped, Render};

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Debug, Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub metadata: Metadata,
    pub image: &'static [u8],
    pub content: PreEscaped<&'static str>,
}

impl Post {
    pub const ALL: &'static [Self] = posts::ALL;

    pub fn get(slug: &str) -> Option<Self> {
        Self::ALL.iter().find(|post| post.slug == slug).copied()
    }
}

impl Render for Post {
    fn render(&self) -> Markup {
        PreEscaped(self.content.0.to_owned())
    }

    fn render_to(&self, buffer: &mut String) {
        self.content.render_to(buffer);
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metadata {
    pub title: &'static str,
    pub year: u16,
    pub month: u8,
    pub day: u8,
}

impl Metadata {
    pub fn date_dashed(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.year, self.month, self.day)
    }

    pub fn date_slashed(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.year, self.month, self.day)
    }
}
