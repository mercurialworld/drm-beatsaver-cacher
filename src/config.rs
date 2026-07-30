use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct CacherConfig {
    pub cache_file: String,
    pub init_cache_on_start: bool,
}

impl CacherConfig {
    pub fn new(filename: &str) -> Result<Self, ConfigError> {
        Config::builder()
            .add_source(File::with_name(filename))
            .build()
            .unwrap()
            .try_deserialize::<CacherConfig>()
    }
}
