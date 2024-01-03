use std::{error::Error, fs, path::Path};

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use maud::{html, PreEscaped, Render, DOCTYPE};

pub fn generate_image(
    browser: &Browser,
    out_dir: &Path,
    filename: impl AsRef<Path>,
    title: &str,
    footer: Option<impl Render>,
) -> Result<(), Box<dyn Error>> {
    let html = html! {
        (DOCTYPE)
        html {
            head {
                style { (PreEscaped(include_str!("open-graph.css"))) }
            }

            body {
                h1 {
                    (title)
                }

                @if let Some(footer) = footer {
                    footer {
                        hr;

                        (footer)
                    }
                }
            }
        }
    };

    let tab = browser.new_tab()?;

    tab.navigate_to(&format!(
        "data:text/html;base64,{}",
        STANDARD_NO_PAD.encode(html.into_string())
    ))?;

    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    fs::write(out_dir.join(filename), png_data)?;

    Ok(())
}
