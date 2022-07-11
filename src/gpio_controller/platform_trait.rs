use super::task::GpioState;

pub trait Platform {
    fn set_high(&mut self);
    fn set_low(&mut self);
    fn get_state(&self) -> GpioState;
}
