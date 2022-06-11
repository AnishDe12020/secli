use std::path::PathBuf;

use clap::Values;

pub enum Command {
    Add(Option<Values<'static>>),
    Get(Option<Values<'static>>),
}

pub fn get_config_path() -> PathBuf {
    let config_dir = dirs::config_dir().unwrap();
    let config_path = config_dir.join("config.json");
    return config_path;
}
