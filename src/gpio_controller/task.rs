use crate::gpio_controller::send_command_trait::GpioTx;

use tokio::sync::mpsc;

pub fn start(mut platform: Box<dyn GpioTx + Send + Sync>) -> mpsc::Sender<TxGpioControllerMessage> {
    let (tx, mut rx) = mpsc::channel(100);
    tokio::spawn(async move {
        loop {
            match rx.recv().await {
                Some(TxGpioControllerMessage::SetHigh) => {
                    platform.set_high();
                }
                Some(TxGpioControllerMessage::SetLow) => {
                    platform.set_low();
                }
                None => (),
            }
        }
    });
    tx
}

#[derive(Debug)]
pub enum TxGpioControllerMessage {
    SetHigh,
    SetLow,
}

#[cfg(test)]
mod tests {

    use crate::platforms::mock::{MockPlatform, MockPlatformMessage};

    use super::*;

    #[tokio::test]
    async fn test_start_set_high() {
        let (tx_open_command, mut rx_open_commands) = mpsc::channel(100);
        let valve = MockPlatform::new(tx_open_command);
        let send_valve_command = start(Box::new(valve));
        send_valve_command
            .try_send(TxGpioControllerMessage::SetHigh)
            .unwrap();
        assert_eq!(
            rx_open_commands.recv().await.unwrap(),
            MockPlatformMessage::SetHigh
        );
    }

    #[tokio::test]
    async fn test_start_set_low() {
        let (tx_open_command, mut rx_open_commands) = mpsc::channel(100);
        let valve = MockPlatform::new(tx_open_command);
        let send_valve_command = start(Box::new(valve));
        send_valve_command
            .try_send(TxGpioControllerMessage::SetLow)
            .unwrap();
        assert_eq!(
            rx_open_commands.recv().await.unwrap(),
            MockPlatformMessage::SetLow
        );
    }
}
