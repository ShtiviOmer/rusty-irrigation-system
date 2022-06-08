pub mod config;
mod gpio_controller;
mod platforms;
mod watering_clock;
mod web_server;

use crate::gpio_controller::task::start as valve_controller_start;

use config::{Config, Platform};
use tokio::sync::mpsc;
use watering_clock::WateringClock;

use crate::platforms::rasberrypie::RaspberryPie;

use platforms::mock::MockPlatform;

use std::error::Error;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut handles = Vec::new();

    let valve = match config.platform {
        Platform::RaspberryPie => RaspberryPie::boxed_new(config.gpio_pins)?,
        Platform::Mock => {
            let (tx, rx) = mpsc::channel(100);
            handles.push(MockPlatform::log_valve_commands(rx));
            MockPlatform::boxed_new(tx)
        }
    };

    let tx = valve_controller_start(valve);
    let watering_clock = WateringClock::try_from(config.watering_clock)?;
    handles.push(watering_clock.start(tx.clone()).await.map_err(Box::new)?);

    web_server::start(tx.clone()).await?;
    Ok(())
}
