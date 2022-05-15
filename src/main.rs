mod valve_controller;
mod valves;
use crate::valves::rasberrypie::pie_valve::PieValve;
use crate::valve_controller::valve_trait::ValveTrait;
use crate::valve_controller::valve_controller::{start as valve_controller_start, ValveControllerMessage};

use std::thread;
use std::time::Duration;
use tokio;
use std::sync::Arc;

#[tokio::main]
async fn main() {

    let valve: Box<dyn ValveTrait + Sync + Send> = Box::new(PieValve::new(4).unwrap());
    // let mut valve = Valve::new(4).unwrap();
    let tx = valve_controller_start(valve);
    tx.try_send(ValveControllerMessage::Open);
    thread::sleep(Duration::from_secs(30));
    tx.try_send(ValveControllerMessage::Close);
}
