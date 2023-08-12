pub mod config;
mod gpio_controller;
mod platforms;
mod watering_clock;

use crate::gpio_controller::task::start as valve_controller_start;

use config::{Config, Platform};
use futures::future;
use tokio::sync::mpsc;
use watering_clock::WateringClock;

use crate::platforms::rasberrypie::RaspberryPie;

use platforms::mock::MockPlatform;

use std::error::Error;

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let mut tasks = Vec::new();

    for clock in config.watering_clocks {
        let valve = match config.platform {
            Platform::RaspberryPie => RaspberryPie::boxed_new(clock.gpio_pin)?,
            Platform::Mock => {
                let (tx, rx) = mpsc::channel(100);
                MockPlatform::log_valve_commands(rx).await;
                MockPlatform::boxed_new(tx)
            }
        };

        let tx = valve_controller_start(valve);
        let watering_clock = WateringClock::try_from(clock)?;
        tasks.push(watering_clock.start(tx.clone()).await.map_err(Box::new)?);
    }
    future::join_all(tasks).await;
    Ok(())
}
