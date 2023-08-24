use std::net::{IpAddr, SocketAddr};

use serde::Deserialize;
use tokio::net::TcpListener;
use tracing::instrument;

use crate::Error;

/// The configuration for this crate.
#[derive(Copy, Clone, Debug, Deserialize)]
pub struct Config {
    /// The ip address to bind to.
    pub ip: IpAddr,

    /// The port to bind to.
    pub port: u16,

    /// Whether the site is running in a production environment.
    #[serde(default)]
    pub production: bool,
}

impl Config {
    /// Load the configuration from the environment.
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }

    /// Get the socket address to bind to.
    #[must_use]
    pub const fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    /// Bind the socket address to a TCP listener.
    #[instrument(level = "debug", err(Debug))]
    pub async fn tcp_listener(&self) -> crate::Result<TcpListener> {
        TcpListener::bind(self.socket_address())
            .await
            .map_err(Error::Serve)
    }
}
