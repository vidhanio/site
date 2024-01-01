use std::{error::Error, fs, path::Path};

use base64::{engine::general_purpose::STANDARD_NO_PAD, Engine as _};
use headless_chrome::{protocol::cdp::Page::CaptureScreenshotFormatOption, Browser};
use maud::{html, PreEscaped, DOCTYPE};

pub fn generate_image(
    browser: &Browser,
    out_dir: &Path,
    filename: impl AsRef<Path>,
    title: &str,
    footer: Option<&str>,
) -> Result<(), Box<dyn Error>> {
    let html = html! {
        (DOCTYPE)
        html {
            head {
                style { (PreEscaped(include_str!("opengraph.css"))) }
            }

            body {
                h1 {
                    (title)
                }

                @if let Some(footer) = footer {
                    footer {
                        hr;

                        (PreEscaped(footer.replace("vidhan.io", "<b>vidhan.io</b>")))
                    }
                }
            }
        }
    }
    .into_string();

    let tab = browser.new_tab()?;

    tab.navigate_to(&format!(
        "data:text/html;base64,{}",
        STANDARD_NO_PAD.encode(html)
    ))?;

    let png_data = tab.capture_screenshot(CaptureScreenshotFormatOption::Png, None, None, true)?;

    fs::write(out_dir.join(filename), png_data)?;

    Ok(())
}
