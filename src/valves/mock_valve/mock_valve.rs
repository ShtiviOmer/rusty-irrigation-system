use crate::valve_controller::valve_trait::ValveTrait;
use tokio::sync::mpsc;

pub struct MockValve {
    valve_output: mpsc::Sender<MockValveAction>,
}

impl MockValve {
    pub fn new(valve_output: mpsc::Sender<MockValveAction>) -> Self {
        Self { valve_output }
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

#[derive(Debug)]
pub enum MockValveAction {
    Open,
    Close,
}
