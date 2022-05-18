mod valve_controller;
mod valves;

use crate::valve_controller::valve_controller::{
    start as valve_controller_start, ValveControllerMessage,
};

use std::{thread, time::Duration};

use tokio::sync::mpsc;

use crate::valve_controller::valve_trait::ValveTrait;
use crate::valves::rasberrypie::pie_valve::PieValve;

use valves::mock_valve::mock_valve::{MockValve, MockValveAction};

use std::error::Error;

pub struct Config {
    valve_type: ValveType,
}

impl Config {
    pub fn new(valve_type: ValveType) -> Self {
        Self { valve_type }
    }
    pub fn get_valve_type(&self) -> &ValveType {
        &self.valve_type
    }
}

pub enum ValveType {
    RaspberryPie,
    Mock,
}

pub async fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let valve = match config.get_valve_type() {
        ValveType::RaspberryPie => get_valve_raspberry_pie()?,
        ValveType::Mock => {
            let (tx, rx) = mpsc::channel(100);
            get_valve_mock(tx)
        }
    };

    let tx = valve_controller_start(valve);
    tx.try_send(ValveControllerMessage::Open)?;
    thread::sleep(Duration::from_secs(30));
    tx.try_send(ValveControllerMessage::Close)?;
    Ok(())
}

fn get_valve_raspberry_pie() -> Result<Box<dyn ValveTrait + Sync + Send>, String> {
    Ok(Box::new(PieValve::new(4).map_err(|e| e.to_string())?))
}

fn get_valve_mock(tx: mpsc::Sender<MockValveAction>) -> Box<dyn ValveTrait + Sync + Send> {
    Box::new(MockValve::new(tx))
}
