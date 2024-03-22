use std::env;

use crate::constants::EEnvKey;

/// Enviroment variable
pub struct EnvValue {
    /// Server port number
    /// Default: 3000
    pub port: u16,
}

impl EnvValue {
    pub fn init() -> Self {
        let port = env::var::<String>(EEnvKey::Port.into())
            .map_or_else(|_| 3000, |v| v.parse::<u16>().unwrap());

        Self { port }
    }
}
