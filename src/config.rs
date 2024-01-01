use std::net::{IpAddr, SocketAddr};

use serde::Deserialize;

/// The configuration for this crate.
#[derive(Debug, Clone, Copy, Deserialize)]
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
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variables are not valid.
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env()
    }

    /// Get the socket address to bind to.
    #[must_use]
    pub const fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}
