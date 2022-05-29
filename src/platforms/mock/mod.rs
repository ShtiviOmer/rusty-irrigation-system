use crate::gpio_controller::send_command_trait::GpioTx;
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::{info_span, instrument::Instrumented, Instrument};

pub struct MockPlatform {
    valve_output: mpsc::Sender<MockPlatformMessage>,
}

/// For testing, or development, we can use a mock platform
impl MockPlatform {
    pub fn new(valve_output: mpsc::Sender<MockPlatformMessage>) -> Self {
        Self { valve_output }
    }
    pub fn boxed_new(valve_output: mpsc::Sender<MockPlatformMessage>) -> Box<Self> {
        Box::new(Self::new(valve_output))
    }
    /// Log all platform commands to the valve output channel
    pub fn log_valve_commands(
        mut rx: mpsc::Receiver<MockPlatformMessage>,
    ) -> Instrumented<JoinHandle<()>> {
        tokio::spawn(async move {
            while let Some(valve_action) = rx.recv().await {
                match valve_action {
                    MockPlatformMessage::SetHigh => {
                        tracing::info!("Opening valve");
                    }
                    MockPlatformMessage::SetLow => {
                        tracing::info!("Closing valve");
                    }
                }
            }
        })
        .instrument(info_span!("mock_valve"))
    }
}

impl GpioTx for MockPlatform {
    fn set_high(&mut self) {
        self.valve_output
            .try_send(MockPlatformMessage::SetHigh)
            .unwrap();
    }
    fn set_low(&mut self) {
        self.valve_output
            .try_send(MockPlatformMessage::SetLow)
            .unwrap();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MockPlatformMessage {
    SetHigh,
    SetLow,
}
