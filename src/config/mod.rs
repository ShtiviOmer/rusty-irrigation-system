use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Config {
    pub platform: Platform,
    pub watering_clocks: Vec<WateringClockConfig>,
    // support for versioning of the config file, might be useful for breaking changes
    #[serde(rename = "version")]
    pub _version: u32,
}

pub fn load_from_yaml(path: String) -> Result<Config, ConfigError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_yaml::from_reader(reader)?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub enum Platform {
    RaspberryPie,
    Mock,
}

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct WateringClockConfig {
    pub start_time: String,
    /// Duration in minutes
    pub duration: i64,
    /// Interval of when opening the clock in hours
    pub interval: i64,
    pub gpio_pin: u8,
}

#[derive(Debug, thiserror::Error)]
pub enum ConfigError {
    #[error("Failed to parse YAML: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_file() {
        let results = load_from_yaml("example_config.yaml".to_owned()).unwrap();
        let expected = Config {
            _version: 1,
            platform: Platform::RaspberryPie,
            watering_clocks: vec![WateringClockConfig {
                start_time: "05:00:00".to_owned(),
                duration: 30,
                interval: 24,
                gpio_pin: 4,
            }],
        };

        assert_eq!(results, expected);
    }

    #[test]
    fn test_example_file_not_found() {
        let err = load_from_yaml("missing_file.yaml".to_owned()).unwrap_err();
        match err {
            ConfigError::Io(_) => (),
            _ => panic!("Expected NotFound error"),
        }
    }

    #[test]
    fn test_invalid_file() {
        let err = load_from_yaml("tests/helpers/invalid_config.yaml".to_owned()).unwrap_err();
        match err {
            ConfigError::Yaml(_) => (),
            _ => panic!("Expected YAML error"),
        }
    }
}
