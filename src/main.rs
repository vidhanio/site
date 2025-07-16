#![allow(missing_docs)]

use std::env;

use tracing::info;
use tracing_subscriber::EnvFilter;
use vidhan_site::Config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    _ = dotenvy::dotenv();

    let config = Config::from_env()?;

    let fmt = tracing_subscriber::fmt().with_env_filter(
        EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            EnvFilter::new(format!(
                "info,tower_http=debug,{}=debug",
                env!("CARGO_CRATE_NAME")
            ))
        }),
    );

    if config.production {
        fmt.json().init();
    } else {
        fmt.pretty().init();
    }

    if !config.production {
        let address = format!("http://{}", config.socket_addr());
        info!(%address, "serving site...");
    }

    vidhan_site::serve(config).await?;

    Ok(())
}
