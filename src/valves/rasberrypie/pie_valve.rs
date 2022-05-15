use crate::valve_controller::valve_trait::ValveTrait;
use rppal::gpio::{Gpio, OutputPin};

pub struct PieValve {
    valve_output: OutputPin,
}

impl PieValve {
    pub fn new(valve_pin_output: u8) -> Result<Self, PieValveError> {
        let gpio = Gpio::new().map_err(|e| PieValveError::PermissionDenied(e.to_string()))?;
        let valve_output = gpio
            .get(valve_pin_output)
            .map_err(|e| PieValveError::PinNotAvialable(e.to_string()))?
            .into_output();
        Ok(Self { valve_output })
    }
}

impl ValveTrait for PieValve {
    fn open(&mut self) {
        self.valve_output.set_high();
    }
    fn close(&mut self) {
        self.valve_output.set_low();
    }
}


#[derive(Debug)]
pub enum PieValveError {
    PermissionDenied(String),
    PinNotAvialable(String),
}