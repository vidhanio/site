use std::{
    net::{IpAddr, SocketAddr},
    path::PathBuf,
};

use serde::Deserialize;

/// The configuration for this crate.
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    /// The ip address to bind to.
    pub ip: IpAddr,

    /// The port to bind to.
    pub port: u16,

    /// The directory containing content files.
    pub content_dir: PathBuf,

    /// The directory containing public files.
    pub public_dir: PathBuf,
}

impl Config {
    /// Load the configuration from the environment.
    pub fn from_env() -> Result<Self, envy::Error> {
        envy::from_env::<Self>()
    }

    /// Get the socket address to bind to.
    #[must_use]
    pub const fn socket_address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }
}
