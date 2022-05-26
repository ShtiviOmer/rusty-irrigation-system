use std::error::Error;
use std::{fs::File, io::BufReader};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    valve_type: ValveType,
    pub watering_clock: WateringClockConfig,
    pub gpio_pins: u8,
}

impl Config {
    pub fn get_valve_type(&self) -> &ValveType {
        &self.valve_type
    }
}

pub fn load_from_yaml(path: String) -> Result<Config, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let config = serde_yaml::from_reader(reader)?;

    Ok(config)
}

#[derive(Debug, Deserialize)]
pub enum ValveType {
    RaspberryPie,
    Mock,
}

#[derive(Debug, Deserialize)]
pub struct WateringClockConfig {
    pub start_time: String,
    /// Duration in minutes
    pub duration: i64,
    /// Interval of when opening the clock in hours
    pub interval: i64,
}
