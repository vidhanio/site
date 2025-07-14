use std::str::FromStr;

use typst::{
    foundations::{Dict, IntoValue, Value},
    visualize::Color,
};

pub const COLORS: Colors = Colors {
    light: Palette {
        fg: "#681ac8",
        bg: "#f0f0f0",
    },
    dark: Palette {
        fg: "#753bbd",
        bg: "#0f0f0f",
    },
};

#[derive(Debug, Clone, Copy)]
pub struct Colors {
    light: Palette,
    dark: Palette,
}

impl Colors {
    pub const fn default_palette(self) -> Palette {
        self.dark
    }

    pub fn apply_to_css(&self, css: &str) -> String {
        let mut css = css.to_owned();

        for (pattern, color) in [
            ("/*{light.fg}*/", self.light.fg),
            ("/*{light.bg}*/", self.light.bg),
            ("/*{dark.fg}*/", self.dark.fg),
            ("/*{dark.bg}*/", self.dark.bg),
        ] {
            css = css.replace(pattern, color);
        }

        css
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Palette {
    pub fg: &'static str,
    pub bg: &'static str,
}

impl Palette {
    pub fn typst_dict(&self) -> Value {
        [("fg", self.fg), ("bg", self.bg)]
            .into_iter()
            .map(|(k, v)| {
                (
                    k.into(),
                    Color::from_str(v)
                        .expect("static color should be valid")
                        .into_value(),
                )
            })
            .collect::<Dict>()
            .into_value()
    }

    pub fn apply_to_css(&self, css: &str) -> String {
        let mut css = css.to_owned();

        for (pattern, color) in [
            ("/*{light.fg}*/", self.fg),
            ("/*{light.bg}*/", self.bg),
            ("/*{dark.fg}*/", self.fg),
            ("/*{dark.bg}*/", self.bg),
        ] {
            css = css.replace(pattern, color);
        }

        css
    }
}
