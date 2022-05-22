use crate::valve_controller::valve_trait::ValveTrait;
use tokio::{sync::mpsc, task::JoinHandle};
use tracing::{info_span, instrument::Instrumented, Instrument};

pub struct MockValve {
    valve_output: mpsc::Sender<MockValveAction>,
}

/// For testing, or development, we can use a mock valve
impl MockValve {
    pub fn new(valve_output: mpsc::Sender<MockValveAction>) -> Self {
        Self { valve_output }
    }
    /// Log all valve commands to the valve output channel
    pub fn log_valve_commands(
        mut rx: mpsc::Receiver<MockValveAction>,
    ) -> Instrumented<JoinHandle<()>> {
        tokio::spawn(async move {
            while let Some(valve_action) = rx.recv().await {
                match valve_action {
                    MockValveAction::Open => {
                        tracing::info!("Opening valve");
                    }
                    MockValveAction::Close => {
                        tracing::info!("Closing valve");
                    }
                }
            }
        })
        .instrument(info_span!("mock_valve"))
    }
}

impl ValveTrait for MockValve {
    fn open(&mut self) {
        self.valve_output.try_send(MockValveAction::Open).unwrap();
    }
    fn close(&mut self) {
        self.valve_output.try_send(MockValveAction::Close).unwrap();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum MockValveAction {
    Open,
    Close,
}
