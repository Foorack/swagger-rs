//! Utility methods for instantiating common connectors for clients.
#[cfg(any(target_os = "macos", target_os = "windows", target_os = "ios"))]
use std::convert::From as _;

/// HTTP Connector construction
#[derive(Debug)]
pub struct Connector;

impl Connector {
    /// Alows building a HTTP(S) connector. Used for instantiating clients with custom
    /// connectors.
    pub fn builder() -> Builder {
        Builder {}
    }
}

/// Builder for HTTP(S) connectors
#[derive(Debug)]
pub struct Builder {}

impl Builder {
    /// Build a HTTP connector
    pub fn build(self) -> hyper::client::HttpConnector {
        hyper::client::HttpConnector::new()
    }
}

