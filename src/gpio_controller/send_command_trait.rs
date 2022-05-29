pub trait GpioTx {
    fn set_high(&mut self);
    fn set_low(&mut self);
}
