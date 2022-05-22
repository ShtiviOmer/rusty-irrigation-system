pub mod valve_trait;

use crate::valve_controller::valve_trait::ValveTrait;

use tokio::sync::mpsc;

pub fn start(mut valve: Box<dyn ValveTrait + Send + Sync>) -> mpsc::Sender<ValveControllerMessage> {
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(ValveControllerMessage::Open) => {
                    tracing::debug!("Opening valve");
                    valve.open();
                }
                Some(ValveControllerMessage::Close) => {
                    tracing::debug!("Closing valve");
                    valve.close();
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

#[cfg(test)]
mod tests {

    use crate::valves::mock_valve::{MockValve, MockValveAction};

    use super::*;

    #[tokio::test]
    async fn test_start() {
        let (tx_open_command, mut rx_open_commands) = mpsc::channel(100);
        let valve = MockValve::new(tx_open_command);
        let send_valve_command = start(Box::new(valve));
        send_valve_command
            .try_send(ValveControllerMessage::Open)
            .unwrap();
        assert_eq!(
            rx_open_commands.recv().await.unwrap(),
            MockValveAction::Open
        );
    }
}
