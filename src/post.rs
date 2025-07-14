use hypertext::Raw;

include!(concat!(env!("OUT_DIR"), "/posts.rs"));

#[derive(Debug, Clone, Copy)]
pub struct Post {
    pub slug: &'static str,
    pub title: &'static str,
    pub date: (u16, u8, u8),
    pub image: &'static [u8],
    pub content: Raw<&'static str>,
    pub footnotes: &'static [(&'static str, Raw<&'static str>)],
}

impl Post {
    pub const ALL: &'static [Self] = posts::ALL;

    pub fn get(slug: &str) -> Option<Self> {
        Self::ALL.iter().find(|post| post.slug == slug).copied()
    }

    pub fn date_dashed(&self) -> String {
        format!("{:04}-{:02}-{:02}", self.date.0, self.date.1, self.date.2)
    }

    pub fn date_slashed(&self) -> String {
        format!("{:04}/{:02}/{:02}", self.date.0, self.date.1, self.date.2)
    }
}
