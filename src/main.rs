mod valve_controller;
mod valves;
use crate::valve_controller::valve_controller::{
    start as valve_controller_start, ValveControllerMessage,
};
use crate::valve_controller::valve_trait::ValveTrait;
use crate::valves::rasberrypie::pie_valve::PieValve;

use std::thread;
use std::time::Duration;
use tokio;
use tokio::sync::mpsc;
#[cfg(test)]
use valves::mock_valve::mock_valve::{MockValve, MockValveAction};

#[tokio::main]
async fn main() {
    let valve = get_valve_raspberry_pie().unwrap();
    #[cfg(test)]
    let (tx, rx) = mpsc::channel(100);
    #[cfg(test)]
    let valve = get_valve_mock(tx);

    let tx = valve_controller_start(valve);
    tx.try_send(ValveControllerMessage::Open);
    thread::sleep(Duration::from_secs(30));
    tx.try_send(ValveControllerMessage::Close);
}

fn get_valve_raspberry_pie() -> Result<Box<dyn ValveTrait + Sync + Send>, String> {
    Ok(Box::new(PieValve::new(4).map_err(|e| e.to_string())?))
}

#[cfg(test)]
fn get_valve_mock(tx: mpsc::Sender<MockValveAction>) -> Box<dyn ValveTrait + Sync + Send> {
    Box::new(MockValve::new(tx))
}
