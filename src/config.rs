use std::net::{IpAddr, SocketAddr};

use rspotify::{AuthCodeSpotify, Credentials, OAuth, Token, scopes};
use serde::Deserialize;

/// The configuration for this crate.
#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// The ip address to bind to.
    pub ip: IpAddr,

    /// The port to bind to.
    pub port: u16,

    /// Whether the site is running in a production environment.
    #[serde(default)]
    pub production: bool,

    spotify_client_id: String,
    spotify_client_secret: String,
    spotify_refresh_token: String,
}

impl Config {
    /// Load the configuration from the environment.
    ///
    /// # Errors
    ///
    /// Returns an error if the environment variables are not valid.
    pub fn from_env() -> envy::Result<Self> {
        envy::from_env()
    }

    /// Get the socket address to bind to.
    #[must_use]
    pub const fn socket_addr(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    pub(crate) fn spotify_client(&self) -> AuthCodeSpotify {
        let token = Token {
            refresh_token: Some(self.spotify_refresh_token.clone()),
            scopes: scopes!(
                "user-read-currently-playing",
                "user-read-recently-played",
                "user-read-playback-state"
            ),
            ..Default::default()
        };

        let credentials = Credentials {
            id: self.spotify_client_id.clone(),
            secret: Some(self.spotify_client_secret.clone()),
        };

        AuthCodeSpotify::from_token_with_config(
            token,
            credentials,
            OAuth::default(),
            rspotify::Config::default(),
        )
    }
}
