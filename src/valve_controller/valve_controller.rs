use rppal::gpio::{Gpio, OutputPin};
use tokio;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Sender;
use std::sync::Arc;
use crate::valve_controller::valve_trait::ValveTrait;
use std::ops::DerefMut;

pub struct ValveController {
    valve: Box<dyn ValveTrait + Send + Sync>,
}

impl ValveController {
    fn new(valve:Box<dyn ValveTrait + Send + Sync>) -> Self {
        Self { valve }
    }

}

pub fn start(valve: Box<dyn ValveTrait  + Send + Sync>) -> mpsc::Sender<ValveControllerMessage> {
    let mut valve_controller = ValveController::new(valve);
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(ValveControllerMessage::Open) => {
                    valve_controller.valve.open();
                }
                Some(ValveControllerMessage::Close) => {
                    valve_controller.valve.close();
                }
                None => (),
            }
        }
    });
    tx
}

pub enum ValveControllerMessage {
    Open,
    Close,
}
