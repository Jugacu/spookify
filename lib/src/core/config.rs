use std::fs;
use std::path::{PathBuf};

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use directories::ProjectDirs;

#[derive(Debug)]
pub enum WriteCfgError {
    Serde(serde_yaml::Error),
    Io(std::io::Error),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Tokens {
    pub access_token: String,
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub tokens: Option<Tokens>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            tokens: None
        }
    }
}

fn get_config_path() -> PathBuf {
    let project_dirs = ProjectDirs::from(
        "es",
        "jugacu",
        "spookify",
    ).unwrap();

    let config_dir = project_dirs.config_dir();

    config_dir.join("spooky.yml")
}

fn load_config() -> Config {
    let config_path = get_config_path();

    fs::read_to_string(config_path)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_yaml::from_str(&ld).map_err(|e| e.to_string()))
        .unwrap_or(Config::default())
}

pub fn write_config(config: Config) -> Result<(), WriteCfgError> {
    let yaml = serde_yaml::to_string(&config)
        .map_err(|err| WriteCfgError::Serde(err))?;

    let config_path = get_config_path();

    // Gets the parent dir so we can create it if the directory does not exist
    let config_dir = config_path.parent().unwrap();

    fs::create_dir_all(config_dir).unwrap();

    fs::write(config_path, yaml).map_err(|e| WriteCfgError::Io(e))
}

lazy_static! {
	pub static ref CONFIG: Config = load_config();
}