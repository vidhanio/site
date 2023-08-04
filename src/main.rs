#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use tracing_subscriber::EnvFilter;
use vidhan_site::Config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env()?;

    let fmt = tracing_subscriber::fmt().with_env_filter(EnvFilter::from_default_env());

    if cfg!(debug_assertions) {
        fmt.pretty().init();
    } else {
        fmt.init();
    }

    vidhan_site::serve(config).await?;

    Ok(())
}
