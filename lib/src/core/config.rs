use std::fs;
use std::path::{PathBuf};
use std::sync::{Mutex, MutexGuard, Arc};

use serde::{Serialize, Deserialize};
use lazy_static::lazy_static;
use directories::ProjectDirs;

#[derive(Debug)]
pub enum WriteCfgError {
    Serde(serde_yaml::Error),
    Io(std::io::Error),
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct SpotifyToken {
    /// An access token that can be provided in subsequent calls, for example to Spotify Web API services.
    pub access_token: String,
    /// How the access token may be used.
    pub token_type: String,
    /// The time period (in seconds) for which the access token is valid.
    pub expires_in: u32,
    /// The timestamp for which the token will expire at.
    pub expires_at: Option<u128>,
    /// A token that can be sent to the Spotify Accounts service in place of an authorization code to request a new ``access_token``.
    pub refresh_token: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Config {
    pub spotify_token: Option<SpotifyToken>,
    /// The spotify app client id
    pub client_id: String,
    /// The spotify app client secret
    pub client_secret: String,
}

pub fn get_config_path() -> PathBuf {
    let project_dirs = ProjectDirs::from(
        "es",
        "jugacu",
        "spookify",
    ).unwrap();

    let config_dir = project_dirs.config_dir();

    config_dir.join("spooky.yml")
}

fn load_config() -> Option<Config> {
    let config_path = get_config_path();

    fs::read_to_string(config_path)
        .map_err(|e| e.to_string())
        .and_then(|ld| serde_yaml::from_str(&ld).map_err(|e| e.to_string()))
        .unwrap_or(None)
}

fn write_config(config: Option<Config>) -> Result<(), WriteCfgError> {
    let yaml = serde_yaml::to_string(&config)
        .map_err(|err| WriteCfgError::Serde(err))?;

    let config_path = get_config_path();

    // Gets the parent dir so we can create it if the directory does not exist
    let config_dir = config_path.parent().unwrap();

    fs::create_dir_all(config_dir).unwrap();

    fs::write(config_path, yaml).map_err(|e| WriteCfgError::Io(e))
}

impl Config {
    pub fn new(client_id: String, client_secret: String) -> Config {
        Config {
            client_id,
            client_secret,
            spotify_token: None
        }
    }

    pub fn global<'a>() -> MutexGuard<'a, Option<Config>> {
        CONFIG.lock().unwrap()
    }

    pub fn write(cfg: Config) -> Result<(), WriteCfgError> {
        let mut current_config = Config::global();

        *current_config = cfg.clone().into();

        write_config(cfg.into())
    }
}

lazy_static! {
    static ref CONFIG: Mutex<Option<Config>> = Mutex::new(load_config());
}