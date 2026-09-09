use std::str::FromStr;

use typst::{
    foundations::{Dict, IntoValue, Value},
    visualize::Color,
};

/// The two palettes used by the site.
pub const COLORS: Colors = Colors {
    light: Palette {
        fg: "#151515",
        bg: "#d5d5d5",
    },
    dark: Palette {
        fg: "#d5d5d5",
        bg: "#151515",
    },
};

/// The site's light and dark palettes.
#[derive(Debug, Clone, Copy)]
pub struct Colors {
    light: Palette,
    dark: Palette,
}

impl Colors {
    /// Returns the palette used when rendering generated images.
    #[must_use]
    pub const fn default_palette(self) -> Palette {
        self.dark
    }

    /// Replaces the color placeholders in an asset.
    #[must_use]
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

/// A foreground/background pair.
#[derive(Debug, Clone, Copy)]
pub struct Palette {
    /// Foreground color as a CSS color string.
    pub fg: &'static str,
    /// Background color as a CSS color string.
    pub bg: &'static str,
}

impl Palette {
    pub(crate) fn typst_dict(&self) -> Value {
        [("fg", self.fg), ("bg", self.bg)]
            .into_iter()
            .map(|(key, value)| {
                (
                    key.into(),
                    Color::from_str(value)
                        .expect("static color should be valid")
                        .into_value(),
                )
            })
            .collect::<Dict>()
            .into_value()
    }
}
