use config::{Config, ConfigError, Environment};
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
#[allow(unused)]
pub struct Settings {
    rpc_url: String,
    chatgpt_api_key: String,
}

impl Settings {
  pub fn new() -> Result<Self, ConfigError> {
      let s = Config::builder()
          .add_source(Environment::with_prefix("CAST"))
          .build()?;

      s.try_deserialize()
  }
}
