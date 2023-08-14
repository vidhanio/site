#![warn(clippy::nursery)]
#![warn(clippy::pedantic)]

use std::env;

use vidhan_site::Config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let config = Config::from_env()?;

    let fmt = tracing_subscriber::fmt().with_env_filter(env::var("RUST_LOG").as_deref().unwrap_or(
        "\
        info,\
        tower_http=debug,\
        axum::rejection=trace,\
        vidhan_site=debug\
        ",
    ));

    if config.production {
        fmt.json().init();
    } else {
        fmt.pretty().init();
    }

    vidhan_site::serve(config).await?;

    Ok(())
}
