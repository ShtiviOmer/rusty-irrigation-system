mod valve_controller;
mod valves;
mod watering_clock;

use crate::valve_controller::valve_controller::start as valve_controller_start;

use chrono::NaiveTime;
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
    let mut handles = Vec::new();

    let valve = match config.get_valve_type() {
        ValveType::RaspberryPie => get_valve_raspberry_pie()?,
        ValveType::Mock => {
            let (tx, rx) = mpsc::channel(100);
            handles.push(MockValve::log_valve_commands(rx));
            get_valve_mock(tx)
        }
    };

    let tx = valve_controller_start(valve);
    handles.push(
        watering_clock::watering_clock::start(
            tx,
            chrono::Duration::days(1),
            NaiveTime::from_hms(9, 0, 0),
            chrono::Duration::minutes(30),
        )
        .await
        .map_err(|e| Box::new(e))?,
    );

    futures::future::join_all(handles).await;
    Ok(())
}

fn get_valve_raspberry_pie() -> Result<Box<dyn ValveTrait + Sync + Send>, String> {
    Ok(Box::new(PieValve::new(4).map_err(|e| e.to_string())?))
}

fn get_valve_mock(tx: mpsc::Sender<MockValveAction>) -> Box<dyn ValveTrait + Sync + Send> {
    Box::new(MockValve::new(tx))
}
