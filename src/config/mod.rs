use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[cfg_attr(test, derive(Eq, PartialEq))]
pub struct Config {
    pub platform: Platform,
    pub watering_clock: WateringClockConfig,
    pub gpio_pins: u8,
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
}

#[derive(Debug)]
pub enum ConfigError {
    Yaml(serde_yaml::Error),
    Io(std::io::ErrorKind),
}

impl From<serde_yaml::Error> for ConfigError {
    fn from(err: serde_yaml::Error) -> Self {
        ConfigError::Yaml(err)
    }
}

impl From<std::io::Error> for ConfigError {
    fn from(err: std::io::Error) -> Self {
        ConfigError::Io(err.kind())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example_file() {
        let results = load_from_yaml("example_config.yaml".to_owned()).unwrap();
        let expected = Config {
            platform: Platform::RaspberryPie,
            watering_clock: WateringClockConfig {
                start_time: "05:00:00".to_owned(),
                duration: 30,
                interval: 24,
            },
            gpio_pins: 4,
        };

        assert_eq!(results, expected);
    }

    #[test]
    fn test_example_file_not_found() {
        let err = load_from_yaml("missing_file.yaml".to_owned()).unwrap_err();
        match err {
            ConfigError::Io(std::io::ErrorKind::NotFound) => (),
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
