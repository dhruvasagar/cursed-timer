use dirs;

use std::{fs::File, io::Write};

use config::Config;
use config::File as ConfigFile;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScrambleConfig {
    pub length: usize,
}

#[derive(Debug, Deserialize)]
pub struct InspectionConfig {
    pub length: usize,
    pub key_hold: usize,
}

const DEFAULT_CONFIG: &str = "[scramble]
length = 16

[inspection]
length = 16
key_hold = 2
";

/// Models the config file found in the app config directory
/// see DEFAULT_CONFIG for example config.toml.
///
///  TODO: Add More Config Options
///

#[derive(Debug, Deserialize)]
pub struct CubeConfig {
    pub scramble: ScrambleConfig,
    pub inspection: InspectionConfig,
}

impl CubeConfig {
    /// Returns the config directory used for the configuration.
    /// @return PathBuf: the config directory
    pub fn get_history_path() -> Option<String> {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if let Some(home_dir) = dirs::home_dir() {
                let config_path = home_dir.join(".config/cursed-timer/history.csv");
                return Some(config_path.to_string_lossy().to_string());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(data_local_dir) = dirs::data_local_dir() {
                let config_path = data_local_dir.join("CursedTimer/history.csv");
                return Some(config_path.to_string_lossy().to_string());
            }
        }

        None
    }

    /// Returns the config directory used for the configuration.
    /// @return Option<String>: the config directory path as a string, or None if not found
    pub fn get_config_path() -> Option<String> {
        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if let Some(home_dir) = dirs::home_dir() {
                let config_path = home_dir.join(".config/cursed-timer/config.toml");
                return Some(config_path.to_string_lossy().to_string());
            }
        }

        #[cfg(target_os = "windows")]
        {
            if let Some(data_local_dir) = dirs::data_local_dir() {
                let config_path = data_local_dir.join("CursedTimer/config.toml");
                return Some(config_path.to_string_lossy().to_string());
            }
        }

        None
    }
    /// Loads the config file inside the config directory.
    /// Windows: %APPDATA%/CursedTimer/config.toml
    /// Mac/Linux: $HOME/.config/cursed-timer/config.toml
    /// @return: Config
    pub fn new() -> Option<CubeConfig> {
        let path = CubeConfig::get_config_path()?;

        println!("{:?}", path);
        if File::open(&path).is_err() {
            let mut file = File::create(path.clone()).unwrap();
            let _ = file.write(DEFAULT_CONFIG.as_bytes());
        }

        Config::builder()
            .add_source(ConfigFile::with_name(&path))
            .build()
            .ok()?
            .try_deserialize::<CubeConfig>()
            .ok()
    }
}
