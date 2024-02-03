use dirs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use toml;

#[derive(Debug, Serialize, Deserialize)]
pub enum ConfigError {
    NoConfigDirectory,
    InvalidOperatingSystem,
    NoConfigFile,
    BadTOMLFormat,
    BadConfigPath,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScrambleConfig {
    pub length: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct InspectionConfig {
    pub length: usize,
    pub key_hold: usize,
}

/// Models the config file found in the app config directory
/// Example config.toml:
///
/// [scramble]
/// length = 16 # Length of scramble.
///
/// [inspection]
/// length = 16 # length of inspection in seconds.
/// key_hold = 2 # how long is needed to start the timer in seconds.
///
///
///
///
///  TODO: Add More Config Options
///
///
///
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub scramble: ScrambleConfig,
    pub inspection: InspectionConfig,
}

impl Config {
    /// Returns the config directory used for the configuration.
    /// @return PathBuf: the config directory
    pub fn get_history_path() -> Result<String, ConfigError> {
        #[cfg(target_os = "windows")]
        {
            if let Some(app_data) = dirs::data_dir() {
                let config_dir = app_data.join("CursedTimer");
                std::fs::create_dir_all(&config_dir).map_err(|_| ConfigError::NoConfigDirectory)?;

                let result = app_data.join("history.csv").to_string_lossy();
                return Ok(String::from(result));
            }
        }

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if let Some(mut home_dir) = dirs::home_dir() {
                home_dir.push(".config");
                home_dir.push("cursed-timer");
                std::fs::create_dir_all(&home_dir).map_err(|_| ConfigError::NoConfigDirectory)?;

                return Ok(String::from(home_dir.join("history.csv").to_string_lossy()));
            }
        }
        return Err(ConfigError::InvalidOperatingSystem);
    }

    /// Returns the config directory used for the configuration.
    /// @return PathBuf: the config directory
    pub fn get_config_path() -> Result<String, ConfigError> {
        #[cfg(target_os = "windows")]
        {
            if let Some(app_data) = dirs::data_dir() {
                let config_dir = app_data.join("CursedTimer");
                std::fs::create_dir_all(&config_dir).map_err(|_| ConfigError::NoConfigDirectory)?;

                let result = app_data.join("config.toml").to_string_lossy();
                return Ok(String::from(result));
            }
        }

        #[cfg(any(target_os = "macos", target_os = "linux"))]
        {
            if let Some(mut home_dir) = dirs::home_dir() {
                home_dir.push(".config");
                home_dir.push("cursed-timer");
                std::fs::create_dir_all(&home_dir).map_err(|_| ConfigError::NoConfigDirectory)?;

                return Ok(String::from(home_dir.join("config.toml").to_string_lossy()));
            }
        }
        return Err(ConfigError::InvalidOperatingSystem);
    }

    /// Loads the config file inside the config directory.
    /// Windows: %APPDATA%/CursedTimer/config.toml
    /// Mac/Linux: $HOME/.config/cursed-timer/config.toml
    /// @return: Config
    pub fn new() -> Result<Config, ConfigError> {
        let config_path = Config::get_config_path()?;
        let mut file = File::open(config_path).map_err(|_| ConfigError::NoConfigFile)?;
        let mut contents = String::new();
        let _ = file.read_to_string(&mut contents);
        return toml::de::from_str(&contents).map_err(|_| ConfigError::BadTOMLFormat);
    }
}
