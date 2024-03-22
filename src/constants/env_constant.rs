/// Constant env key mapping
pub enum EEnvKey {
    /// Mapping to: PORT
    Port,
}

impl From<EEnvKey> for String {
    fn from(value: EEnvKey) -> Self {
        match value {
            EEnvKey::Port => "PORT".to_string(),
        }
    }
}
