use std::collections::HashMap;

use kk_core::DocumentMode;
use log::info;

pub struct Config {
    pub keys: HashMap<DocumentMode, String>,
}

impl Config {
    pub fn load(global_config: &str) -> anyhow::Result<()> {
        let global_config = toml::from_str::<toml::Value>(global_config)?;  
        info!("{:#?}", global_config);
        Ok(())
    }
}
