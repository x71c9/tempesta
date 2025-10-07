// ****************************************************************************
// Print configuration values
// ****************************************************************************

use super::common::PanicOnError;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct Config {
  pub git: bool,
  pub remote: Option<String>,
  pub dir: String,
}

pub fn run() {
  let config = load_config();
  println!("Git enabled: {}", config.git);
  if let Some(remote) = &config.remote {
    println!("Remote: {}", remote);
  } else {
    println!("Remote: None");
  }
  println!("Bookmark store directory: {}", config.dir);
}

pub fn load_config() -> Config {
  let config_file_path = get_config_file_path();
  let content = fs::read_to_string(&config_file_path)
    .panic_on_error("Cannot read config file");
  let config: Config =
    toml::from_str(&content).panic_on_error("Cannot read toml config file");
  config
}

pub fn get_config_file_path() -> PathBuf {
  super::common::CONFIG_FILE_PATH
    .get()
    .cloned()
    .unwrap_or_else(|| {
      let home_dir =
        dirs::home_dir().panic_on_error("Could not find home directory");
      let mut config_path = home_dir;
      config_path.push(".config/tempesta");
      fs::create_dir_all(&config_path)
        .panic_on_error("Failed to create config directory");
      config_path.push("tempesta.toml");
      config_path
    })
}

pub fn save_config(config: &Config) {
  let config_file_path = get_config_file_path();
  let content =
    toml::to_string(config).panic_on_error("Cannot write toml config file");
  fs::write(config_file_path, content)
    .panic_on_error("Cannot write config file");
}
