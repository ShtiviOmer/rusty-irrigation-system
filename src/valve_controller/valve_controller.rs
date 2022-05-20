use crate::valve_controller::valve_trait::ValveTrait;

use tokio;
use tokio::sync::mpsc;

pub struct ValveController {
    valve: Box<dyn ValveTrait + Send + Sync>,
}

impl ValveController {
    fn new(valve: Box<dyn ValveTrait + Send + Sync>) -> Self {
        Self { valve }
    }
}

pub fn start(valve: Box<dyn ValveTrait + Send + Sync>) -> mpsc::Sender<ValveControllerMessage> {
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

#[derive(Debug)]
pub enum ValveControllerMessage {
    Open,
    Close,
}
