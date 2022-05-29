use std::{error, fmt};

use crate::gpio_controller::send_command_trait::GpioTx;
use rppal::gpio::{Gpio, OutputPin};

pub struct RaspberryPie {
    valve_output: OutputPin,
}

impl RaspberryPie {
    pub fn new(valve_pin_output: u8) -> Result<Self, PieValveError> {
        let gpio = Gpio::new().map_err(|e| PieValveError::PermissionDenied(e.to_string()))?;
        let valve_output = gpio
            .get(valve_pin_output)
            .map_err(|e| PieValveError::PinNotAvailable(e.to_string()))?
            .into_output();
        Ok(Self { valve_output })
    }

    pub fn boxed_new(valve_pin_output: u8) -> Result<Box<dyn GpioTx + Sync + Send>, PieValveError> {
        Ok(Box::new(Self::new(valve_pin_output)?))
    }
}

impl GpioTx for RaspberryPie {
    fn set_high(&mut self) {
        self.valve_output.set_high();
    }
    fn set_low(&mut self) {
        self.valve_output.set_low();
    }
}

#[derive(Debug)]
pub enum PieValveError {
    PermissionDenied(String),
    PinNotAvailable(String),
}
impl fmt::Display for PieValveError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            PieValveError::PermissionDenied(s) => write!(f, "Permission denied: {}", s),
            PieValveError::PinNotAvailable(s) => write!(f, "Pin not available: {}", s),
        }
    }
}

impl error::Error for PieValveError {}
