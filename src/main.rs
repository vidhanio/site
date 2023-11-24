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
        env::var(EnvFilter::DEFAULT_ENV)
            .as_deref()
            .unwrap_or("info,tower_http=debug,vidhan_site=debug"),
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
