pub mod config;
mod valve_controller;
mod valves;
mod watering_clock;

use crate::valve_controller::start as valve_controller_start;

use config::{Config, ValveType};
use tokio::sync::mpsc;
use watering_clock::WateringClock;

use crate::valve_controller::valve_trait::ValveTrait;
use crate::valves::rasberrypie::PieValve;

use valves::mock_valve::{MockValve, MockValveAction};

use std::error::Error;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();

    let valve = match config.get_valve_type() {
        ValveType::RaspberryPie => get_valve_raspberry_pie(config.gpio_pins)?,
        ValveType::Mock => {
            let (tx, rx) = mpsc::channel(100);
            handles.push(MockValve::log_valve_commands(rx));
            get_valve_mock(tx)
        }
    };

    let tx = valve_controller_start(valve);
    let watering_clock = WateringClock::try_from(config.watering_clock)?;
    handles.push(watering_clock.start(tx).await.map_err(Box::new)?);

    futures::future::join_all(handles).await;
    Ok(())
}

fn get_valve_raspberry_pie(gpio_pins: u8) -> Result<Box<dyn ValveTrait + Sync + Send>, String> {
    Ok(Box::new(
        PieValve::new(gpio_pins).map_err(|e| e.to_string())?,
    ))
}

fn get_valve_mock(tx: mpsc::Sender<MockValveAction>) -> Box<dyn ValveTrait + Sync + Send> {
    Box::new(MockValve::new(tx))
}
